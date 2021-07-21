use std::ops::Deref;

use crossbeam_epoch as epoch;
use crossbeam_skiplist::SkipList;
use epoch::Guard;
use hyper::Body;
use nanoid::nanoid;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

use crate::core::skiplist_serde;
use crate::server::{HttpError, HttpRequest, HttpResponse, HttpRoute};
use crate::service::AbOptimisationService;

use super::project::Project;

#[derive(Serialize, Deserialize)]
pub struct App {
    #[serde(skip_deserializing)]
    pub id: String,
    pub name: String,
    pub short_name: String,

    // todo: pub auth_key: String,
    #[serde(skip)]
    // #[serde(with = "skiplist_serde")]
    #[serde(default = "default_projects")]
    pub projects: SkipList<String, RwLock<Project>>,
}

fn default_projects() -> SkipList<String, RwLock<Project>> {
    SkipList::new(epoch::default_collector().clone())
}

impl AbOptimisationService {
    pub async fn update_app(
        &self,
        route: &HttpRoute<'_>,
        app_id: &str,
        body: Body,
    ) -> anyhow::Result<http::Response<Body>> {
        let req = HttpRequest::value::<App>(route, body).await?;

        let guard = &epoch::pin();

        match self.visit_app(
            app_id,
            guard,
            |entry: crossbeam_skiplist::base::Entry<String, RwLock<App>>| {
                let app_lock = entry.value();
                let mut app_guard = app_lock.write();
                if app_guard.short_name != req.short_name {
                    app_guard.short_name = req.short_name
                }

                if app_guard.name != req.name {
                    app_guard.name = req.name
                }

                self.write_app_data(&app_guard)?;

                HttpResponse::str(route, "SUCCESS")
            },
        ) {
            Ok(result) => result,
            Err(err) => err.into(),
        }
    }

    pub async fn add_app(
        &self,
        route: &HttpRoute<'_>,
        body: Body,
    ) -> anyhow::Result<http::Response<Body>> {
        let mut req_data = HttpRequest::value::<App>(route, body).await?;

        let guard = &epoch::pin();

        let id = nanoid!();
        req_data.id = id.to_string();
        self.write_app_data(&req_data)?;

        self.apps.insert(id, RwLock::new(req_data), guard);

        HttpResponse::str(route, "SUCCESS")
    }

    pub async fn get_app(
        &self,
        route: &HttpRoute<'_>,
        app_id: &str,
    ) -> anyhow::Result<http::Response<Body>> {
        let guard = &epoch::pin();

        match self.visit_app(
            app_id,
            guard,
            |entry: crossbeam_skiplist::base::Entry<String, RwLock<App>>| {
                let app_lock = entry.value();
                let app_guard = app_lock.read();
                let app = app_guard.deref();

                HttpResponse::binary_or_json(route, app)
            },
        ) {
            Ok(result) => result,
            Err(err) => err.into(),
        }
    }

    pub async fn list_apps(&self, route: &HttpRoute<'_>) -> anyhow::Result<http::Response<Body>> {
        let wrapper = skiplist_serde::SerdeListWrapper(&self.apps);

        HttpResponse::binary_or_json(route, &wrapper)
    }

    pub fn visit_app<'g, F, R>(
        &self,
        app_id: &str,
        guard: &'g Guard,
        visitor: F,
    ) -> Result<R, HttpError>
    where
        F: FnOnce(crossbeam_skiplist::base::Entry<String, RwLock<App>>) -> R,
    {
        let entry = self.apps.get(app_id, guard);
        match entry {
            None => {
                // insert here
                Err(HttpError::NotFound(format!(
                    "No app found for id: {}",
                    app_id
                )))
            }
            Some(entry) => Ok(visitor(entry)),
        }
    }
}
