use std::sync::atomic::AtomicI64;

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::core::HasId;

use super::variation::Variation;

#[derive(Serialize, Deserialize)]
pub struct Experiment {
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
    pub variations: Vec<Variation>,
    pub data: Option<JsonValue>,

    #[serde(default = "default_sampler")]
    #[serde(skip)]
    pub variation_sampler: rand::distributions::Uniform<i64>,

    #[serde(default)]
    #[serde(skip)]
    pub control_size: AtomicI64,

    #[serde(default)]
    #[serde(skip)]
    pub test_size: AtomicI64,
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
    pub picked_size: AtomicI64,
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
        value: i64,
    },
    Percent {
        #[serde(alias = "size_value")]
        value: i64,

        #[serde(default = "default_sampler")]
        #[serde(skip)]
        sampler: rand::distributions::Uniform<i64>,
    },
}

fn default_sampler() -> rand::distributions::Uniform<i64> {
    rand::distributions::Uniform::<i64>::from(0..10000)
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
