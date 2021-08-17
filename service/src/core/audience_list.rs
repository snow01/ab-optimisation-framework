use std::collections::HashSet;
use std::ops::Deref;

use anyhow::{anyhow, Context};
use crossbeam_epoch as epoch;
use crossbeam_epoch::Guard;
use hyper::Body;
#[allow(unused_imports)]
use log::{debug, error, info, warn};
use nanoid::nanoid;
use parking_lot::lock_api::RwLockWriteGuard;
use parking_lot::{RawRwLock, RwLock};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::core::{skiplist_serde, AddResponse, HasId, Project};
use crate::server::{ApiError, ApiResult, HttpRequest, HttpResponse, HttpResult, HttpRoute};
use crate::service::AbOptimisationService;

#[derive(Serialize, Deserialize, Validate)]
pub struct AudienceList {
    #[serde(skip_deserializing)]
    pub id: String,

    #[validate(length(min = 1))]
    pub name: String,

    pub list: HashSet<String>,

    #[serde(skip)]
    #[serde(default)]
    pub modification_time: i64,
}

impl HasId for AudienceList {
    fn id(&self) -> &str {
        &self.id
    }
}

impl AbOptimisationService {
    pub(crate) fn load_audience_list(
        &self,
        file: &str,
        app_id: &str,
        project_id: &str,
        list_id: &str,
        mut audience_list: AudienceList,
        modification_time: i64,
    ) -> anyhow::Result<()> {
        let guard = &epoch::pin();

        self.visit_project(app_id, project_id, guard, |entry| {
            audience_list.id = list_id.to_string();
            audience_list.modification_time = modification_time;

            let audience_lists = &entry.value().read().audience_lists;

            match audience_lists.get(list_id, guard) {
                None => {
                    info!("Loading audience_list for app:{}, project:{}, list_id:{}", app_id, project_id, list_id);
                    audience_lists.insert(list_id.to_string(), RwLock::new(audience_list), guard);
                }
                Some(entry) => {
                    let mut audience_list_guard = entry.value().write();

                    if modification_time == 0 || modification_time > audience_list_guard.modification_time {
                        info!("Updated audience_list for app:{}, project:{}, list_id:{}", app_id, project_id, list_id);
                        AbOptimisationService::update_audience_list_data(audience_list, &mut audience_list_guard);
                    }
                }
            }

            Ok(())
        })
        .with_context(|| format!("Error in loading audience list from file: {}", file))
    }

    pub async fn add_audience_list(&self, route: &HttpRoute<'_>, app_id: &str, project_id: &str, body: Body) -> HttpResult {
        let mut req_data = HttpRequest::value::<AudienceList>(route, body).await?;

        let guard = &epoch::pin();

        let visitor = |entry: crossbeam_skiplist::base::Entry<String, RwLock<Project>>| {
            let project = entry.value().read();

            self.validate_audience_list_data(&project, &req_data, None, guard)?;

            let id = nanoid!();
            req_data.id = id.to_string();
            self.experiment_store.write_audience_list_data(app_id, project_id, &req_data)?;

            project.audience_lists.insert(id.to_string(), RwLock::new(req_data), guard);

            HttpResponse::binary_or_json(route, &AddResponse { id })
        };

        self.visit_project(app_id, project_id, guard, visitor)
    }

    pub async fn update_audience_list(&self, route: &HttpRoute<'_>, app_id: &str, project_id: &str, list_id: &str, body: Body) -> HttpResult {
        let req_data = HttpRequest::value::<AudienceList>(route, body).await?;

        let guard = &epoch::pin();

        let validation_visitor = |entry: crossbeam_skiplist::base::Entry<String, RwLock<Project>>| {
            let project = entry.value().read();

            self.validate_audience_list_data(&project, &req_data, Some(list_id), guard)
        };

        self.visit_project(app_id, project_id, guard, validation_visitor)?;

        let visitor = |entry: crossbeam_skiplist::base::Entry<String, RwLock<AudienceList>>| {
            let mut existing_data = entry.value().write();

            AbOptimisationService::update_audience_list_data(req_data, &mut existing_data);

            self.experiment_store.write_audience_list_data(app_id, project_id, &existing_data)?;

            HttpResponse::str(route, "SUCCESS")
        };

        self.visit_audience_list(app_id, project_id, list_id, guard, visitor)
    }

    fn update_audience_list_data(req_data: AudienceList, existing_data: &mut RwLockWriteGuard<RawRwLock, AudienceList>) {
        if existing_data.name != req_data.name {
            existing_data.name = req_data.name
        }

        existing_data.list = req_data.list;

        existing_data.modification_time = req_data.modification_time;
    }

    fn validate_audience_list_data(&self, project: &Project, data_to_validate: &AudienceList, update_id: Option<&str>, guard: &Guard) -> ApiResult<()> {
        data_to_validate.validate().with_context(|| format!("Error in validating audience list data"))?;

        for entry in project.audience_lists.iter(guard) {
            let value = entry.value();
            let experiment = value.read();

            if let Some(update_id) = update_id {
                if experiment.id.eq(update_id) {
                    continue;
                }
            }

            if experiment.name.eq(&data_to_validate.name) {
                return Err(ApiError::BadRequest(anyhow!("Audience List with same name={} already exists", experiment.name)));
            }
        }

        Ok(())
    }

    pub async fn get_audience_list(&self, route: &HttpRoute<'_>, app_id: &str, project_id: &str, list_id: &str) -> HttpResult {
        let guard = &epoch::pin();

        let visitor = |entry: crossbeam_skiplist::base::Entry<String, RwLock<AudienceList>>| {
            let pojo = entry.value().read();
            let pojo = pojo.deref();

            HttpResponse::binary_or_json(route, pojo)
        };

        self.visit_audience_list(app_id, project_id, list_id, guard, visitor)
    }

    pub async fn list_audience_lists(&self, route: &HttpRoute<'_>, app_id: &str, project_id: &str) -> HttpResult {
        let guard = &epoch::pin();

        let visitor = |entry: crossbeam_skiplist::base::Entry<String, RwLock<Project>>| {
            let pojo = entry.value().read();
            let pojo = pojo.deref();

            let wrapper = skiplist_serde::SerdeListWrapper(&pojo.audience_lists);

            HttpResponse::binary_or_json(route, &wrapper)
        };

        self.visit_project(app_id, project_id, guard, visitor)
    }

    pub fn visit_audience_list<'g, F, R>(&self, app_id: &str, project_id: &str, list_id: &str, guard: &'g Guard, visitor: F) -> ApiResult<R>
    where
        F: FnOnce(crossbeam_skiplist::base::Entry<String, RwLock<AudienceList>>) -> ApiResult<R>,
    {
        let proj_visitor = |entry: crossbeam_skiplist::base::Entry<String, RwLock<Project>>| {
            let project_guard = entry.value().read();
            let audience_list_entry = project_guard.audience_lists.get(list_id, guard);
            match audience_list_entry {
                None => {
                    // insert here
                    Err(ApiError::NotFound(format!(
                        "Audience List not found for list id: {}, project id: {} and app id: {}",
                        list_id, project_id, app_id
                    )))
                }
                Some(audience_list_entry) => visitor(audience_list_entry),
            }
        };

        self.visit_project(app_id, project_id, guard, proj_visitor)
    }
}
