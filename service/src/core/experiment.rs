use std::ops::Deref;

use crossbeam_epoch as epoch;
use crossbeam_epoch::Guard;
use hyper::Body;
use nanoid::nanoid;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::core::{skiplist_serde, HasId, Project};
use crate::server::{HttpError, HttpRequest, HttpResponse, HttpRoute};
use crate::service::AbOptimisationService;

use super::variation::Variation;
use std::sync::atomic::AtomicU64;

#[derive(Serialize, Deserialize)]
pub struct Experiment {
    #[serde(skip_deserializing)]
    pub id: String,
    pub name: String,
    pub short_name: String,

    #[serde(default)]
    pub version: i64,

    #[serde(default)]
    pub kind: ExperimentKind,

    #[serde(default)]
    pub inactive: bool,

    pub start_time: Option<String>,
    pub end_time: Option<String>,

    pub audiences: Vec<Audience>,

    pub variations: Option<Vec<Variation>>,
    pub data: Option<JsonValue>,

    #[serde(default = "default_sampler")]
    #[serde(skip)]
    pub variation_sampler: rand::distributions::Uniform<u64>,

    #[serde(default)]
    #[serde(skip)]
    pub control_size: AtomicU64,

    #[serde(default)]
    #[serde(skip)]
    pub test_size: AtomicU64,
}

impl HasId for Experiment {
    fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Serialize, Deserialize)]
pub enum ExperimentKind {
    Feature,
    Experiment,
}

impl Default for ExperimentKind {
    fn default() -> Self {
        Self::Experiment
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "kind")]
pub struct Audience {
    pub name: String,

    #[serde(flatten)]
    pub audience: AudienceSpec,

    #[serde(flatten)]
    pub size: SizeSpec,

    #[serde(default)]
    #[serde(skip)]
    pub picked_size: AtomicU64,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "audience_kind")]
pub enum AudienceSpec {
    Script { script_src: Option<String> },
    List { list_id: String },
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "size_kind")]
pub enum SizeSpec {
    Absolute {
        #[serde(alias = "size_value")]
        value: u64,
    },
    Percent {
        #[serde(alias = "size_value")]
        value: u32,
        // #[serde(default = "default_sampler")]
        // #[serde(skip)]
        // sampler: rand::distributions::Uniform<u64>,
    },
}

fn default_sampler() -> rand::distributions::Uniform<u64> {
    rand::distributions::Uniform::<u64>::from(0..10000)
}

impl AbOptimisationService {
    pub async fn add_experiment(
        &self,
        route: &HttpRoute<'_>,
        app_id: &str,
        project_id: &str,
        body: Body,
    ) -> anyhow::Result<http::Response<Body>> {
        let mut req_data = HttpRequest::value::<Experiment>(route, body).await?;

        // TODO: validate same name and short name doesn't exist
        // TODO: validate short name can be max 5 chars
        // TODO: validate experiment data

        let guard = &epoch::pin();

        let visitor = |entry: crossbeam_skiplist::base::Entry<String, RwLock<Project>>| {
            let project = entry.value().read();

            let id = nanoid!();

            req_data.id = id.to_string();
            self.write_experiment_data(app_id, project_id, &req_data)?;

            project.experiments.insert(id, RwLock::new(req_data), guard);

            HttpResponse::str(route, "SUCCESS")
        };

        let x = self.visit_project(app_id, project_id, guard, visitor);

        match x {
            Ok(result) => result,
            Err(err) => err.into(),
        }
    }

    pub async fn update_experiment(
        &self,
        route: &HttpRoute<'_>,
        app_id: &str,
        project_id: &str,
        experiment_id: &str,
        body: Body,
    ) -> anyhow::Result<http::Response<Body>> {
        let req_data = HttpRequest::value::<Experiment>(route, body).await?;

        // TODO: validate same name and short name doesn't exist
        // TODO: validate short name can be max 5 chars
        // TODO: validate experiment data

        let guard = &epoch::pin();

        let visitor = |entry: crossbeam_skiplist::base::Entry<String, RwLock<Experiment>>| {
            let mut existing_data = entry.value().write();

            // TODO: identify what got changed

            if existing_data.name != req_data.name {
                existing_data.name = req_data.name
            }

            self.write_experiment_data(app_id, project_id, &existing_data)?;

            HttpResponse::str(route, "SUCCESS")
        };

        let x = self.visit_experiment(app_id, project_id, experiment_id, guard, visitor);

        match x {
            Ok(result) => result,
            Err(err) => err.into(),
        }
    }

    pub async fn get_experiment(
        &self,
        route: &HttpRoute<'_>,
        app_id: &str,
        project_id: &str,
        experiment_id: &str,
    ) -> anyhow::Result<http::Response<Body>> {
        let guard = &epoch::pin();

        let visitor = |entry: crossbeam_skiplist::base::Entry<String, RwLock<Experiment>>| {
            let pojo = entry.value().read();
            let pojo = pojo.deref();

            HttpResponse::binary_or_json(route, pojo)
        };

        let x = self.visit_experiment(app_id, project_id, experiment_id, guard, visitor);

        match x {
            Ok(result) => result,
            Err(err) => err.into(),
        }
    }

    pub async fn list_experiments(
        &self,
        route: &HttpRoute<'_>,
        app_id: &str,
        project_id: &str,
    ) -> anyhow::Result<http::Response<Body>> {
        let guard = &epoch::pin();

        let visitor = |entry: crossbeam_skiplist::base::Entry<String, RwLock<Project>>| {
            let pojo = entry.value().read();
            let pojo = pojo.deref();

            let wrapper = skiplist_serde::SerdeListWrapper(&pojo.experiments);

            HttpResponse::binary_or_json(route, &wrapper)
        };

        let x = self.visit_project(app_id, project_id, guard, visitor);

        match x {
            Ok(result) => result,
            Err(err) => err.into(),
        }
    }

    pub fn visit_experiment<'g, F, R>(
        &self,
        app_id: &str,
        project_id: &str,
        experiment_id: &str,
        guard: &'g Guard,
        visitor: F,
    ) -> Result<R, HttpError>
    where
        F: FnOnce(crossbeam_skiplist::base::Entry<String, RwLock<Experiment>>) -> R,
    {
        let proj_visitor = |entry: crossbeam_skiplist::base::Entry<String, RwLock<Project>>| {
            let project_guard = entry.value().read();
            let experiment_entry = project_guard.experiments.get(experiment_id, guard);
            match experiment_entry {
                None => {
                    // insert here
                    Err(HttpError::NotFound(format!(
                        "Experiment not found for id: {}, project id: {} and app id: {}",
                        experiment_id, project_id, app_id
                    )))
                }
                Some(experiment_entry) => Ok(visitor(experiment_entry)),
            }
        };

        let x = self.visit_project(app_id, project_id, guard, proj_visitor);
        match x {
            Ok(result) => result,
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    // use anyhow::anyhow;
    // use anyhow::Context;
    use pyo3::prelude::*;
    use pyo3::types::IntoPyDict;
    use pythonize::pythonize;
    use serde_json::json;

    // use pyo3::types::IntoPyDict;

    #[test]
    fn test_pyo3() -> anyhow::Result<()> {
        for _i in 0..10 {
            let instance = Instant::now();

            Python::with_gil(|py| -> anyhow::Result<bool> {
                let ctx = Some(json!({
                    "new_user": true,
                    "app_version": "4.8.3",
                }));

                let ctx = pythonize(py, &ctx)?;

                let locals = [("ctx", ctx)].into_py_dict(py);

                // let sys = PyModule::import(py, "sys")?;
                let fns = PyModule::from_code(
                    py,
                    r#"
from packaging.version import parse as parse_version
                "#,
                    "fns.py",
                    "fns",
                )?;
                let globals = [("fns", fns)].into_py_dict(py);

                let result = py.eval(
                    // "sys.version",
                    "fns.parse_version(ctx['app_version'])",
                    // "ctx['new_user'] == False and ctx['app_version'] >= '4.7.3'",
                    Some(&globals),
                    Some(&locals),
                )?;

                println!("Result: {:?}", result);
                println!("Total time taken: {:?}", instance.elapsed());

                // match result {
                //     Ok(result) => {
                //         // let res: bool = result.extract().unwrap();
                //
                //         println!("Result: {:?}", result);
                //         println!("Total time taken: {:?}", instance.elapsed());
                //     }
                //     Err(err) => {
                //         println!("Error in executing python function: {}", err)
                //     }
                // }
                // .map_err(|err| Err(anyhow!("Error in evaluating expression: {}", err)))?;
                // .with_context(|| format!("Error in evaluating expression"))?;
                // .map_err(|e| {
                //     e.print_and_set_sys_last_vars(py);
                // })?;

                Ok(true)
            })?;
        }

        Ok(())
    }
}
