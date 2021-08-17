use std::ops::Deref;

use anyhow::{anyhow, Context};
use crossbeam_epoch as epoch;
use crossbeam_skiplist::SkipList;
use epoch::Guard;
use hyper::Body;
#[allow(unused_imports)]
use log::{debug, error, info, warn};
use nanoid::nanoid;
use parking_lot::lock_api::RwLockWriteGuard;
use parking_lot::{RawRwLock, RwLock};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::core::{skiplist_serde, AddResponse};
use crate::server::{ApiError, ApiResult, HttpRequest, HttpResponse, HttpResult, HttpRoute};
use crate::service::AbOptimisationService;

use super::project::Project;

#[derive(Serialize, Deserialize, Validate)]
pub struct App {
    #[serde(skip_deserializing)]
    pub id: String,

    #[validate(length(min = 1))]
    pub name: String,

    #[validate(length(min = 1, max = 5))]
    pub short_name: String,

    // todo: pub auth_key: String,
    #[serde(skip)]
    #[serde(default = "default_projects")]
    pub projects: SkipList<String, RwLock<Project>>,

    #[serde(skip)]
    #[serde(default)]
    pub modification_time: i64,
}

fn default_projects() -> SkipList<String, RwLock<Project>> {
    SkipList::new(epoch::default_collector().clone())
}

impl AbOptimisationService {
    pub(crate) fn load_app(&self, app_id: &str, mut app: App, modification_time: i64) -> anyhow::Result<()> {
        let guard = &epoch::pin();

        app.id = app_id.to_string();
        app.modification_time = modification_time;

        let entry = self.apps.get(app_id, guard);
        match entry {
            None => {
                info!("Loading app for id:{}", app_id);
                self.apps.insert(app_id.to_string(), RwLock::new(app), guard);
            }
            Some(entry) => {
                let mut app_guard = entry.value().write();

                if modification_time == 0 || modification_time > app_guard.modification_time {
                    info!("Updated app for id: {}", app.id);
                    AbOptimisationService::update_app_data(app, &mut app_guard);
                }
            }
        }

        Ok(())
    }

    pub async fn add_app(&self, route: &HttpRoute<'_>, body: Body) -> HttpResult {
        let mut req_data = HttpRequest::value::<App>(route, body).await?;

        let guard = &epoch::pin();

        // validate app data
        self.validate_app_data(&req_data, None, guard)?;

        let id = nanoid!();
        req_data.id = id.to_string();
        self.experiment_store.write_app_data(&req_data)?;

        self.apps.insert(id.to_string(), RwLock::new(req_data), guard);

        HttpResponse::binary_or_json(route, &AddResponse { id })
    }

    pub async fn update_app(&self, route: &HttpRoute<'_>, app_id: &str, body: Body) -> HttpResult {
        let req = HttpRequest::value::<App>(route, body).await?;

        let guard = &epoch::pin();

        // validate app data
        self.validate_app_data(&req, Some(app_id), guard)?;

        self.visit_app(app_id, guard, |entry: crossbeam_skiplist::base::Entry<String, RwLock<App>>| {
            let mut app_guard = entry.value().write();
            AbOptimisationService::update_app_data(req, &mut app_guard);

            self.experiment_store.write_app_data(&app_guard)?;

            HttpResponse::str(route, "SUCCESS")
        })
    }

    fn update_app_data(req_data: App, app_guard: &mut RwLockWriteGuard<RawRwLock, App>) {
        if app_guard.short_name != req_data.short_name {
            app_guard.short_name = req_data.short_name
        }

        if app_guard.name != req_data.name {
            app_guard.name = req_data.name
        }

        app_guard.modification_time = req_data.modification_time;
    }

    fn validate_app_data(&self, data_to_validate: &App, update_id: Option<&str>, guard: &Guard) -> Result<(), ApiError> {
        data_to_validate.validate().with_context(|| format!("Error in validating app data"))?;

        for entry in self.apps.iter(guard) {
            let value = entry.value();
            let app = value.read();

            if let Some(update_id) = update_id {
                if app.id.eq(update_id) {
                    continue;
                }
            }

            if app.short_name.eq(&data_to_validate.short_name) {
                return Err(ApiError::BadRequest(anyhow!("App with same short_name={} already exists", app.short_name)));
            }

            if app.name.eq(&data_to_validate.name) {
                return Err(ApiError::BadRequest(anyhow!("App with same name={} already exists", app.name)));
            }
        }

        Ok(())
    }

    pub async fn get_app(&self, route: &HttpRoute<'_>, app_id: &str) -> HttpResult {
        let guard = &epoch::pin();

        self.visit_app(app_id, guard, |entry: crossbeam_skiplist::base::Entry<String, RwLock<App>>| {
            let app_lock = entry.value();
            let app_guard = app_lock.read();
            let app = app_guard.deref();

            HttpResponse::binary_or_json(route, app)
        })
    }

    pub async fn list_apps(&self, route: &HttpRoute<'_>) -> HttpResult {
        let wrapper = skiplist_serde::SerdeListWrapper(&self.apps);

        HttpResponse::binary_or_json(route, &wrapper)
    }

    pub fn visit_app<'g, F, R>(&self, app_id: &str, guard: &'g Guard, visitor: F) -> ApiResult<R>
    where
        F: FnOnce(crossbeam_skiplist::base::Entry<String, RwLock<App>>) -> ApiResult<R>,
    {
        let entry = self.apps.get(app_id, guard);
        match entry {
            None => {
                // insert here
                Err(ApiError::NotFound(format!("No app found for id: {}", app_id)))
            }
            Some(entry) => visitor(entry),
        }
    }
}
