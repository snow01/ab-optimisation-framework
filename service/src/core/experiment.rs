use std::ops::Deref;
use std::sync::atomic::{AtomicI64, AtomicU64};

use anyhow::{anyhow, Context};
use crossbeam_epoch as epoch;
use crossbeam_epoch::Guard;
use hyper::Body;
use itertools::Itertools;
use nanoid::nanoid;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use validator::{Validate, ValidationError};

use crate::core::{skiplist_serde, AddResponse, HasId, Project};
use crate::server::{ApiError, ApiResult, HttpRequest, HttpResponse, HttpResult, HttpRoute};
use crate::service::AbOptimisationService;

use super::variation::Variation;

#[derive(Serialize, Deserialize, Validate)]
pub struct Experiment {
    #[serde(skip_deserializing)]
    pub id: String,

    #[validate(length(min = 1))]
    pub name: String,

    #[validate(length(min = 1, max = 5))]
    pub short_name: String,

    #[serde(default)]
    pub version: i64,

    #[serde(default)]
    pub kind: ExperimentKind,

    #[serde(default)]
    pub inactive: bool,

    pub start_time: Option<String>,
    pub end_time: Option<String>,

    #[validate]
    #[validate(length(min = 1))]
    #[validate(custom = "validate_audiences")]
    pub audiences: Vec<Audience>,

    pub frequency_constraint: Option<String>,

    #[validate]
    #[validate(custom = "validate_variations")]
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

impl PartialEq for Experiment {
    fn eq(&self, other: &Self) -> bool {
        (
            &self.id,
            &self.name,
            &self.short_name,
            &self.version,
            &self.kind,
            &self.inactive,
            &self.start_time,
            &self.end_time,
            &self.audiences,
            &self.variations,
            &self.data,
        ) == (
            &other.id,
            &other.name,
            &other.short_name,
            &other.version,
            &other.kind,
            &other.inactive,
            &other.start_time,
            &other.end_time,
            &other.audiences,
            &other.variations,
            &other.data,
        )
    }
}

impl HasId for Experiment {
    fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq)]
pub enum ExperimentKind {
    Feature,
    Experiment,
}

impl Default for ExperimentKind {
    fn default() -> Self {
        Self::Experiment
    }
}

#[derive(Serialize, Deserialize, Validate)]
#[serde(tag = "kind")]
pub struct Audience {
    #[validate(length(min = 1))]
    pub name: String,

    pub script_src: Option<String>,

    pub list_id: Option<String>,

    #[serde(flatten)]
    #[validate(custom = "validate_size_spec")]
    pub size: SizeSpec,

    #[serde(default)]
    #[serde(skip)]
    pub picked_size: AtomicI64,
}

impl PartialEq for Audience {
    fn eq(&self, other: &Self) -> bool {
        (&self.name, &self.script_src, &self.list_id, &self.size)
            == (&other.name, &other.script_src, &self.list_id, &other.size)
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "size_kind")]
pub enum SizeSpec {
    Absolute {
        #[serde(alias = "size_value")]
        value: i64,
    },
    Percent {
        #[serde(alias = "size_value")]
        value: i64,
    },
}

fn validate_size_spec(size_spec: &SizeSpec) -> Result<(), ValidationError> {
    match size_spec {
        SizeSpec::Absolute { value } => {
            if 0.ge(value) {
                return Err(ValidationError::new("invalid absolute size value"));
            }
        }
        SizeSpec::Percent { value } => {
            if 0.ge(value) || 100.lt(value) {
                return Err(ValidationError::new(
                    "invalid percent size value - should be min=1 and max=100",
                ));
            }
        }
    }

    Ok(())
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
    ) -> HttpResult {
        let mut req_data = HttpRequest::value::<Experiment>(route, body).await?;

        let guard = &epoch::pin();

        let visitor = |entry: crossbeam_skiplist::base::Entry<String, RwLock<Project>>| {
            let project = entry.value().read();

            self.validate_experiment_data(&project, &req_data, None, guard)?;

            let id = nanoid!();

            req_data.id = id.to_string();
            req_data.version = 1; // start with version # 1
            self.write_experiment_data(app_id, project_id, &req_data)?;

            project
                .experiments
                .insert(id.to_string(), RwLock::new(req_data), guard);

            HttpResponse::binary_or_json(route, &AddResponse { id })
        };

        self.visit_project(app_id, project_id, guard, visitor)
    }

    pub async fn update_experiment(
        &self,
        route: &HttpRoute<'_>,
        app_id: &str,
        project_id: &str,
        experiment_id: &str,
        body: Body,
    ) -> HttpResult {
        let req_data = HttpRequest::value::<Experiment>(route, body).await?;

        let guard = &epoch::pin();

        let validation_visitor =
            |entry: crossbeam_skiplist::base::Entry<String, RwLock<Project>>| {
                let project = entry.value().read();

                self.validate_experiment_data(&project, &req_data, Some(experiment_id), guard)
            };

        self.visit_project(app_id, project_id, guard, validation_visitor)?;

        let visitor = |entry: crossbeam_skiplist::base::Entry<String, RwLock<Experiment>>| {
            let mut existing_data = entry.value().write();

            let mut changed = false;
            if existing_data.name != req_data.name {
                existing_data.name = req_data.name;
                changed = true;
            }

            if existing_data.short_name != req_data.short_name {
                existing_data.short_name = req_data.short_name;
                changed = true;
            }

            if existing_data.kind != req_data.kind {
                existing_data.kind = req_data.kind;
                changed = true;
            }

            if existing_data.inactive != req_data.inactive {
                existing_data.inactive = req_data.inactive;
                changed = true;
            }

            if existing_data.start_time != req_data.start_time {
                existing_data.start_time = req_data.start_time;
                changed = true;
            }

            if existing_data.end_time != req_data.end_time {
                existing_data.end_time = req_data.end_time;
                changed = true;
            }

            if existing_data.audiences != req_data.audiences {
                existing_data.audiences = req_data.audiences;
                changed = true;
            }

            if existing_data.variations != req_data.variations {
                existing_data.variations = req_data.variations;
                changed = true;
            }

            if existing_data.frequency_constraint != req_data.frequency_constraint {
                existing_data.frequency_constraint = req_data.frequency_constraint;
                changed = true;
            }

            if existing_data.data != req_data.data {
                existing_data.data = req_data.data;
                changed = true;
            }

            // version change can be because of
            //      - change in audience spec or size
            //      - change in variations or size
            // if audience spec changed -- user may not be anymore part of the eligible set.
            // if audience size increased -- it is fine to keep user
            // if audience size decreased -- we may need to re-evaluate user's eligibility
            // if change in variations -- experiment is kept, but user may need to be assigned to different variant
            // if variant size increased -- experiment is kept, variation is kept
            // if variant size decreased -- experiment is kept, reassign variant
            if changed {
                // increment the version #
                existing_data.version += 1;
            }

            self.write_experiment_data(app_id, project_id, &existing_data)?;

            HttpResponse::str(route, "SUCCESS")
        };

        self.visit_experiment(app_id, project_id, experiment_id, guard, visitor)
    }

    fn validate_experiment_data(
        &self,
        project: &Project,
        data_to_validate: &Experiment,
        update_id: Option<&str>,
        guard: &Guard,
    ) -> ApiResult<()> {
        data_to_validate
            .validate()
            .with_context(|| format!("Error in validating experiment data"))?;

        if let Some(variations) = data_to_validate.variations.as_ref() {
            if variations.len() > 0 && data_to_validate.kind == ExperimentKind::Feature {
                return Err(ApiError::BadRequest(anyhow!(
                    "For experiment of kind=Feature no variations are allowed"
                )));
            }
        }

        if data_to_validate.version > 0 {
            return Err(ApiError::BadRequest(anyhow!(
                "Version # is automatically calculated and is not allowed"
            )));
        }

        for entry in project.experiments.iter(guard) {
            let value = entry.value();
            let experiment = value.read();

            if let Some(update_id) = update_id {
                if experiment.id.eq(update_id) {
                    continue;
                }
            }

            if experiment.short_name.eq(&data_to_validate.short_name) {
                return Err(ApiError::BadRequest(anyhow!(
                    "Experiment with same short_name={} already exists",
                    experiment.short_name
                )));
            }

            if experiment.name.eq(&data_to_validate.name) {
                return Err(ApiError::BadRequest(anyhow!(
                    "Experiment with same name={} already exists",
                    experiment.name
                )));
            }
        }

        Ok(())
    }

    pub async fn get_experiment(
        &self,
        route: &HttpRoute<'_>,
        app_id: &str,
        project_id: &str,
        experiment_id: &str,
    ) -> HttpResult {
        let guard = &epoch::pin();

        let visitor = |entry: crossbeam_skiplist::base::Entry<String, RwLock<Experiment>>| {
            let pojo = entry.value().read();
            let pojo = pojo.deref();

            HttpResponse::binary_or_json(route, pojo)
        };

        self.visit_experiment(app_id, project_id, experiment_id, guard, visitor)
    }

    pub async fn list_experiments(
        &self,
        route: &HttpRoute<'_>,
        app_id: &str,
        project_id: &str,
    ) -> HttpResult {
        let guard = &epoch::pin();

        let visitor = |entry: crossbeam_skiplist::base::Entry<String, RwLock<Project>>| {
            let pojo = entry.value().read();
            let pojo = pojo.deref();

            let wrapper = skiplist_serde::SerdeListWrapper(&pojo.experiments);

            HttpResponse::binary_or_json(route, &wrapper)
        };

        self.visit_project(app_id, project_id, guard, visitor)
    }

    pub fn visit_experiment<'g, F, R>(
        &self,
        app_id: &str,
        project_id: &str,
        experiment_id: &str,
        guard: &'g Guard,
        visitor: F,
    ) -> ApiResult<R>
    where
        F: FnOnce(crossbeam_skiplist::base::Entry<String, RwLock<Experiment>>) -> ApiResult<R>,
    {
        let proj_visitor = |entry: crossbeam_skiplist::base::Entry<String, RwLock<Project>>| {
            let project_guard = entry.value().read();
            let experiment_entry = project_guard.experiments.get(experiment_id, guard);
            match experiment_entry {
                None => {
                    // insert here
                    Err(ApiError::NotFound(format!(
                        "Experiment not found for id: {}, project id: {} and app id: {}",
                        experiment_id, project_id, app_id
                    )))
                }
                Some(experiment_entry) => visitor(experiment_entry),
            }
        };

        self.visit_project(app_id, project_id, guard, proj_visitor)
    }
}

fn validate_variations(variations: &Vec<Variation>) -> Result<(), ValidationError> {
    // check for unique variations by name
    if !variations
        .iter()
        .map(|variation| &variation.name)
        .all_unique()
    {
        return Err(ValidationError::new("Duplicate variation name found"));
    }

    // check for unique variations by short_name
    if !variations
        .iter()
        .map(|variation| &variation.short_name)
        .all_unique()
    {
        return Err(ValidationError::new("Duplicate variation short_name found"));
    }

    // check variation size adds upto 100
    if variations
        .iter()
        .map(|variation| variation.size)
        .sum::<u64>()
        != 100
    {
        return Err(ValidationError::new(
            "All variations total size should sum to 100",
        ));
    }

    Ok(())
}

fn validate_audiences(audiences: &Vec<Audience>) -> Result<(), ValidationError> {
    // check for unique variations by name
    if !audiences.iter().map(|audience| &audience.name).all_unique() {
        return Err(ValidationError::new("Duplicate audience name found"));
    }

    Ok(())
}
