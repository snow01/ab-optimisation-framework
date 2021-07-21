use std::collections::HashSet;
use std::ops::Deref;

use crossbeam_epoch as epoch;
use crossbeam_epoch::Guard;
use crossbeam_skiplist::SkipList;
use hyper::Body;
use nanoid::nanoid;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

use crate::core::{skiplist_serde, App, HasId};
use crate::server::{HttpError, HttpRequest, HttpResponse, HttpRoute};
use crate::service::AbOptimisationService;

use super::experiment::Experiment;

#[derive(Serialize, Deserialize)]
pub struct Project {
    #[serde(skip_deserializing)]
    pub id: String,
    pub name: String,
    pub short_name: String,

    #[serde(skip)]
    #[serde(with = "skiplist_serde")]
    #[serde(default = "default_experiments")]
    pub experiments: SkipList<String, RwLock<Experiment>>,

    #[serde(skip)]
    #[serde(with = "skiplist_serde")]
    #[serde(default = "default_audience_lists")]
    pub audience_lists: SkipList<String, RwLock<AudienceList>>,
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

#[derive(Serialize, Deserialize)]
pub struct AudienceList {
    #[serde(skip_deserializing)]
    pub id: String,
    pub name: String,
    pub list: HashSet<String>,
}

impl HasId for AudienceList {
    fn id(&self) -> &str {
        &self.id
    }
}

impl AbOptimisationService {
    pub async fn add_project(
        &self,
        route: &HttpRoute<'_>,
        app_id: &str,
        body: Body,
    ) -> anyhow::Result<http::Response<Body>> {
        let mut req_data = HttpRequest::value::<Project>(route, body).await?;

        // TODO: validate same name and short name doesn't exist
        // TODO: validate short name can be max 5 chars

        let guard = &epoch::pin();

        let visitor = |entry: crossbeam_skiplist::base::Entry<String, RwLock<App>>| {
            let app_lock = entry.value();
            let app_guard = app_lock.read();

            let id = nanoid!();
            req_data.id = id.to_string();
            self.write_project_data(app_id, &req_data)?;

            app_guard.projects.insert(id, RwLock::new(req_data), guard);

            HttpResponse::str(route, "SUCCESS")
        };

        let x = self.visit_app(app_id, guard, visitor);

        match x {
            Ok(result) => result,
            Err(err) => err.into(),
        }
    }

    pub async fn update_project(
        &self,
        route: &HttpRoute<'_>,
        app_id: &str,
        project_id: &str,
        body: Body,
    ) -> anyhow::Result<http::Response<Body>> {
        let req_data = HttpRequest::value::<Project>(route, body).await?;

        // TODO: validate same name and short name doesn't exist
        // TODO: validate short name can be max 5 chars

        let guard = &epoch::pin();

        let visitor = |entry: crossbeam_skiplist::base::Entry<String, RwLock<Project>>| {
            let mut existing_data = entry.value().write();

            if existing_data.short_name != req_data.short_name {
                existing_data.short_name = req_data.short_name
            }

            if existing_data.name != req_data.name {
                existing_data.name = req_data.name
            }

            self.write_project_data(app_id, &existing_data)?;

            HttpResponse::str(route, "SUCCESS")
        };

        let x = self.visit_project(app_id, project_id, guard, visitor);

        match x {
            Ok(result) => result,
            Err(err) => err.into(),
        }
    }

    pub async fn get_project(
        &self,
        route: &HttpRoute<'_>,
        app_id: &str,
        project_id: &str,
    ) -> anyhow::Result<http::Response<Body>> {
        let guard = &epoch::pin();

        let visitor = |entry: crossbeam_skiplist::base::Entry<String, RwLock<Project>>| {
            let proj = entry.value().read();
            let proj = proj.deref();

            HttpResponse::binary_or_json(route, proj)
        };

        let x = self.visit_project(app_id, project_id, guard, visitor);

        match x {
            Ok(result) => result,
            Err(err) => err.into(),
        }
    }

    pub async fn list_projects(
        &self,
        route: &HttpRoute<'_>,
        app_id: &str,
    ) -> anyhow::Result<http::Response<Body>> {
        let guard = &epoch::pin();

        let visitor = |entry: crossbeam_skiplist::base::Entry<String, RwLock<App>>| {
            let app_guard = entry.value().read();

            let wrapper = skiplist_serde::SerdeListWrapper(&app_guard.projects);

            HttpResponse::binary_or_json(route, &wrapper)
        };

        let x = self.visit_app(app_id, guard, visitor);

        match x {
            Ok(result) => result,
            Err(err) => err.into(),
        }
    }

    pub fn visit_project<'g, F, R>(
        &self,
        app_id: &str,
        project_id: &str,
        guard: &'g Guard,
        visitor: F,
    ) -> Result<R, HttpError>
    where
        F: FnOnce(crossbeam_skiplist::base::Entry<String, RwLock<Project>>) -> R,
    {
        let app_visitor = |entry: crossbeam_skiplist::base::Entry<String, RwLock<App>>| {
            let app_lock = entry.value();
            let app_guard = app_lock.read();
            let proj_entry = app_guard.projects.get(project_id, guard);
            match proj_entry {
                None => {
                    // insert here
                    Err(HttpError::NotFound(format!(
                        "Project not found for project id: {} and app id: {}",
                        project_id, app_id
                    )))
                }
                Some(proj_entry) => Ok(visitor(proj_entry)),
            }
        };

        let x = self.visit_app(app_id, guard, app_visitor);
        match x {
            Ok(result) => result,
            Err(err) => Err(err),
        }
    }
}

impl AbOptimisationService {
    pub async fn add_audience_list(
        &self,
        route: &HttpRoute<'_>,
        app_id: &str,
        project_id: &str,
        body: Body,
    ) -> anyhow::Result<http::Response<Body>> {
        let mut req_data = HttpRequest::value::<AudienceList>(route, body).await?;

        // TODO: validate same name doesn't exist

        let guard = &epoch::pin();

        let visitor = |entry: crossbeam_skiplist::base::Entry<String, RwLock<Project>>| {
            let project = entry.value().read();

            let id = nanoid!();
            req_data.id = id.to_string();
            self.write_audience_list_data(app_id, project_id, &req_data)?;

            project
                .audience_lists
                .insert(id, RwLock::new(req_data), guard);

            HttpResponse::str(route, "SUCCESS")
        };

        let x = self.visit_project(app_id, project_id, guard, visitor);

        match x {
            Ok(result) => result,
            Err(err) => err.into(),
        }
    }

    pub async fn update_audience_list(
        &self,
        route: &HttpRoute<'_>,
        app_id: &str,
        project_id: &str,
        list_id: &str,
        body: Body,
    ) -> anyhow::Result<http::Response<Body>> {
        let req_data = HttpRequest::value::<AudienceList>(route, body).await?;

        // TODO: validate same name doesn't exist

        let guard = &epoch::pin();

        let visitor = |entry: crossbeam_skiplist::base::Entry<String, RwLock<AudienceList>>| {
            let mut existing_data = entry.value().write();

            if existing_data.name != req_data.name {
                existing_data.name = req_data.name
            }

            existing_data.list = req_data.list;

            self.write_audience_list_data(app_id, project_id, &existing_data)?;

            HttpResponse::str(route, "SUCCESS")
        };

        let x = self.visit_audience_list(app_id, project_id, list_id, guard, visitor);

        match x {
            Ok(result) => result,
            Err(err) => err.into(),
        }
    }

    pub async fn get_audience_list(
        &self,
        route: &HttpRoute<'_>,
        app_id: &str,
        project_id: &str,
        list_id: &str,
    ) -> anyhow::Result<http::Response<Body>> {
        let guard = &epoch::pin();

        let visitor = |entry: crossbeam_skiplist::base::Entry<String, RwLock<AudienceList>>| {
            let pojo = entry.value().read();
            let pojo = pojo.deref();

            HttpResponse::binary_or_json(route, pojo)
        };

        let x = self.visit_audience_list(app_id, project_id, list_id, guard, visitor);

        match x {
            Ok(result) => result,
            Err(err) => err.into(),
        }
    }

    pub async fn list_audience_lists(
        &self,
        route: &HttpRoute<'_>,
        app_id: &str,
        project_id: &str,
    ) -> anyhow::Result<http::Response<Body>> {
        let guard = &epoch::pin();

        let visitor = |entry: crossbeam_skiplist::base::Entry<String, RwLock<Project>>| {
            let pojo = entry.value().read();
            let pojo = pojo.deref();

            let wrapper = skiplist_serde::SerdeListWrapper(&pojo.audience_lists);

            HttpResponse::binary_or_json(route, &wrapper)
        };

        let x = self.visit_project(app_id, project_id, guard, visitor);

        match x {
            Ok(result) => result,
            Err(err) => err.into(),
        }
    }

    pub fn visit_audience_list<'g, F, R>(
        &self,
        app_id: &str,
        project_id: &str,
        list_id: &str,
        guard: &'g Guard,
        visitor: F,
    ) -> Result<R, HttpError>
    where
        F: FnOnce(crossbeam_skiplist::base::Entry<String, RwLock<AudienceList>>) -> R,
    {
        let proj_visitor = |entry: crossbeam_skiplist::base::Entry<String, RwLock<Project>>| {
            let project_guard = entry.value().read();
            let audience_list_entry = project_guard.audience_lists.get(list_id, guard);
            match audience_list_entry {
                None => {
                    // insert here
                    Err(HttpError::NotFound(format!(
                        "Audience List not found for list id: {}, project id: {} and app id: {}",
                        list_id, project_id, app_id
                    )))
                }
                Some(audience_list_entry) => Ok(visitor(audience_list_entry)),
            }
        };

        let x = self.visit_project(app_id, project_id, guard, proj_visitor);
        match x {
            Ok(result) => result,
            Err(err) => Err(err),
        }
    }
}

// pub struct ExperimentGroup {
//     pub id: String,
//     pub
// }
