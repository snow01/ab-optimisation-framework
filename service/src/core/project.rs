use std::ops::Deref;

use anyhow::{anyhow, Context};
use crossbeam_epoch as epoch;
use crossbeam_epoch::Guard;
use crossbeam_skiplist::SkipList;
use hyper::Body;
#[allow(unused_imports)]
use log::{debug, error, info, warn};
use nanoid::nanoid;
use parking_lot::lock_api::RwLockWriteGuard;
use parking_lot::{RawRwLock, RwLock};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::core::audience_list::AudienceList;
use crate::core::{skiplist_serde, AddResponse, App, HasId};
use crate::server::{ApiError, ApiResult, HttpRequest, HttpResponse, HttpResult, HttpRoute};
use crate::service::AbOptimisationService;

use super::experiment::Experiment;

#[derive(Serialize, Deserialize, Validate)]
pub struct Project {
    #[serde(skip_deserializing)]
    pub id: String,

    #[validate(length(min = 1))]
    pub name: String,

    #[validate(length(min = 1, max = 5))]
    pub short_name: String,

    #[serde(default = "default_tracking_method")]
    pub tracking_method: TrackingMethod,

    #[serde(skip)]
    #[serde(with = "skiplist_serde")]
    #[serde(default = "default_experiments")]
    pub experiments: SkipList<String, RwLock<Experiment>>,

    #[serde(skip)]
    #[serde(with = "skiplist_serde")]
    #[serde(default = "default_audience_lists")]
    pub audience_lists: SkipList<String, RwLock<AudienceList>>,

    #[serde(skip)]
    #[serde(default)]
    pub modification_time: i64,
}

fn default_experiments() -> SkipList<String, RwLock<Experiment>> {
    SkipList::new(epoch::default_collector().clone())
}

fn default_audience_lists() -> SkipList<String, RwLock<AudienceList>> {
    SkipList::new(epoch::default_collector().clone())
}

impl HasId for Project {
    fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq)]
pub enum TrackingMethod {
    Both,
    Cookie,
    Data,
}

fn default_tracking_method() -> TrackingMethod {
    TrackingMethod::Both
}

impl AbOptimisationService {
    pub(crate) fn load_project(&self, file: &str, app_id: &str, project_id: &str, mut project: Project, modification_time: i64) -> anyhow::Result<()> {
        let guard = &epoch::pin();

        self.visit_app(app_id, guard, |entry| {
            project.id = project_id.to_string();
            project.modification_time = modification_time;

            let projects = &entry.value().read().projects;
            match projects.get(project_id, guard) {
                None => {
                    info!("Loading project for app:{}, id:{}", app_id, project_id);
                    projects.insert(project_id.to_string(), RwLock::new(project), guard);
                }
                Some(entry) => {
                    let project_lock = entry.value();
                    let mut project_guard = project_lock.write();

                    if modification_time == 0 || modification_time > project_guard.modification_time {
                        info!("Updated project for app:{}, id:{}", app_id, project_id);
                        AbOptimisationService::update_project_data(project, &mut project_guard);
                    }
                }
            }

            Ok(())
        })
        .with_context(|| format!("Error in loading project from file: {}", file))
    }

    pub async fn add_project(&self, route: &HttpRoute<'_>, app_id: &str, body: Body) -> HttpResult {
        let mut req_data = HttpRequest::value::<Project>(route, body).await?;

        let guard = &epoch::pin();

        let visitor = |entry: crossbeam_skiplist::base::Entry<String, RwLock<App>>| {
            let app_lock = entry.value();
            let app_guard = app_lock.read();

            self.validate_project_data(&app_guard, &req_data, None, guard)?;

            let id = nanoid!();
            req_data.id = id.to_string();
            self.experiment_store.write_project_data(app_id, &req_data)?;

            app_guard.projects.insert(id.to_string(), RwLock::new(req_data), guard);

            HttpResponse::binary_or_json(route, &AddResponse { id })
        };

        self.visit_app(app_id, guard, visitor)
    }

    pub async fn update_project(&self, route: &HttpRoute<'_>, app_id: &str, project_id: &str, body: Body) -> HttpResult {
        let req_data = HttpRequest::value::<Project>(route, body).await?;

        let guard = &epoch::pin();

        let validation_visitor = |entry: crossbeam_skiplist::base::Entry<String, RwLock<App>>| {
            let app_lock = entry.value();
            let app_guard = app_lock.read();

            self.validate_project_data(&app_guard, &req_data, Some(project_id), guard)
        };

        self.visit_app(app_id, guard, validation_visitor)?;

        let visitor = |entry: crossbeam_skiplist::base::Entry<String, RwLock<Project>>| {
            let mut existing_data = entry.value().write();

            AbOptimisationService::update_project_data(req_data, &mut existing_data);

            self.experiment_store.write_project_data(app_id, &existing_data)?;

            HttpResponse::str(route, "SUCCESS")
        };

        self.visit_project(app_id, project_id, guard, visitor)
    }

    fn update_project_data(req_data: Project, existing_data: &mut RwLockWriteGuard<RawRwLock, Project>) {
        if existing_data.short_name != req_data.short_name {
            existing_data.short_name = req_data.short_name
        }

        if existing_data.name != req_data.name {
            existing_data.name = req_data.name
        }

        if existing_data.tracking_method != req_data.tracking_method {
            existing_data.tracking_method = req_data.tracking_method
        }

        existing_data.modification_time = req_data.modification_time;
    }

    fn validate_project_data(&self, app: &App, data_to_validate: &Project, update_id: Option<&str>, guard: &Guard) -> ApiResult<()> {
        data_to_validate.validate().with_context(|| format!("Error in validating project data"))?;

        for entry in app.projects.iter(guard) {
            let value = entry.value();
            let proj = value.read();

            if let Some(update_id) = update_id {
                if proj.id.eq(update_id) {
                    continue;
                }
            }

            if proj.short_name.eq(&data_to_validate.short_name) {
                return Err(ApiError::BadRequest(anyhow!("Project with same short_name={} already exists", app.short_name)));
            }

            if proj.name.eq(&data_to_validate.name) {
                return Err(ApiError::BadRequest(anyhow!("Project with same name={} already exists", app.name)));
            }
        }

        Ok(())
    }

    pub async fn get_project(&self, route: &HttpRoute<'_>, app_id: &str, project_id: &str) -> HttpResult {
        let guard = &epoch::pin();

        let visitor = |entry: crossbeam_skiplist::base::Entry<String, RwLock<Project>>| {
            let proj = entry.value().read();
            let proj = proj.deref();

            HttpResponse::binary_or_json(route, proj)
        };

        self.visit_project(app_id, project_id, guard, visitor)
    }

    pub async fn list_projects(&self, route: &HttpRoute<'_>, app_id: &str) -> HttpResult {
        let guard = &epoch::pin();

        let visitor = |entry: crossbeam_skiplist::base::Entry<String, RwLock<App>>| {
            let app_guard = entry.value().read();

            let wrapper = skiplist_serde::SerdeListWrapper(&app_guard.projects);

            HttpResponse::binary_or_json(route, &wrapper)
        };

        self.visit_app(app_id, guard, visitor)
    }

    pub fn visit_project<'g, F, R>(&self, app_id: &str, project_id: &str, guard: &'g Guard, visitor: F) -> ApiResult<R>
    where
        F: FnOnce(crossbeam_skiplist::base::Entry<String, RwLock<Project>>) -> ApiResult<R>,
    {
        let app_visitor = |entry: crossbeam_skiplist::base::Entry<String, RwLock<App>>| {
            let app_lock = entry.value();
            let app_guard = app_lock.read();
            let proj_entry = app_guard.projects.get(project_id, guard);
            match proj_entry {
                None => {
                    // insert here
                    Err(ApiError::NotFound(format!(
                        "Project not found for project id: {} and app id: {}",
                        project_id, app_id
                    )))
                }
                Some(proj_entry) => visitor(proj_entry),
            }
        };

        self.visit_app(app_id, guard, app_visitor)
    }
}
