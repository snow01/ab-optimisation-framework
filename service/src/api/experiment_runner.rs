use std::collections::{BTreeMap, HashMap};
use std::ops::Deref;
use std::sync::atomic::Ordering;

use anyhow::{anyhow, Context, Error};
use crossbeam_epoch as epoch;
use crossbeam_epoch::Guard;
use hyper::header::SET_COOKIE;
use hyper::http::HeaderValue;
use hyper::Body;
#[allow(unused_imports)]
use log::{debug, error, info, warn};
use rand::distributions::Distribution;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::api::common::merge_data;
use crate::api::experiment_tracking_data::{TrackedExperiment, TrackingData, TrackingDataParser};
use crate::core;
use crate::core::{Project, Script, TrackingMethod};
use crate::server::{ApiError, HttpRequest, HttpResponse, HttpRoute};
use crate::service::AbOptimisationService;

use super::common::ExperimentMemberKind;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentRequest {
    pub app_id: String,
    pub project_id: String,
    pub user_id: String,
    pub context: Option<JsonValue>,
    pub tracking_data: Option<String>,

    #[serde(skip)]
    #[serde(default = "current_time")]
    experiment_start_time: chrono::DateTime<chrono::Local>,

    #[serde(skip)]
    #[serde(default = "default_context")]
    script_context: jexl_eval::Value,
}

fn current_time() -> chrono::DateTime<chrono::Local> {
    chrono::Local::now()
}

fn default_context() -> jexl_eval::Value {
    jexl_eval::Value::Null
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentResponse<'a> {
    pub app_id: &'a str,
    pub project_id: &'a str,
    pub active_experiments: Vec<ActiveExperiment>,
    pub tracking_cookie_name: Option<String>,
    pub tracking_data: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveExperiment {
    pub short_name: String,
    pub variation: Option<String>,
    pub data: Option<JsonValue>,
}

impl AbOptimisationService {
    fn run_internal<'a, F, R>(&'a self, route: &HttpRoute<'a>, req: &'a ExperimentRequest, result_visitor: F) -> R
    where
        F: FnOnce(Result<ExperimentResponse, ApiError>) -> R,
    {
        let guard = &epoch::pin();

        // check app
        let app_entry = self.apps.get(&req.app_id, guard);
        match app_entry {
            None => result_visitor(Err(ApiError::NotFound(format!("App:{} not found", req.app_id)))),
            Some(app_entry) => {
                let app_lock = app_entry.value();
                let app = app_lock.read();

                let proj_entry = app.projects.get(&req.project_id, guard);
                match proj_entry {
                    None => result_visitor(Err(ApiError::NotFound(format!("Project:{} not found", req.project_id)))),
                    Some(proj_entry) => {
                        let proj_lock = proj_entry.value();
                        let proj = proj_lock.read();

                        let result = self.run_for_project(route, &req, &app, &proj, guard);
                        result_visitor(result)
                    }
                }
            }
        }
    }

    fn run_for_project<'a>(
        &self,
        route: &HttpRoute<'_>,
        req: &'a ExperimentRequest,
        app: &'a core::App,
        proj: &'a core::Project,
        guard: &'a Guard,
    ) -> Result<ExperimentResponse<'a>, ApiError> {
        let tracking_cookie_name = Self::tracking_cookie_name(&app, &proj);

        // get tracking history
        let tracking_history = Self::parse_tracking_history(route, req, &proj, &tracking_cookie_name)?;

        // build map from tracking history to the version
        let tracking_history = Self::build_tracking_history_map(tracking_history);

        info!("Tracking History: {:#?}", tracking_history);

        // LATER: parse cookie / header / context into final context object - not needed immediately...

        let mut tracked_experiments = Vec::<TrackedExperiment>::new();
        let mut active_experiments = Vec::<ActiveExperiment>::new();

        // go over all experiments for the project (later to be done in experiment group order)
        for exp_entry in proj.experiments.iter(guard) {
            let exp_lock = exp_entry.value();
            let experiment = exp_lock.read();

            // todo: check for schedule
            if experiment.inactive {
                continue;
            }

            let existing_experiment = tracking_history.get(&experiment.short_name);

            // sample user for the experiment
            let (targeting_eligible, frequency_eligible, mut picked) = self.sample_experiment(&req, &proj, experiment.deref(), existing_experiment, guard)?;

            info!(
                "Experiment={}@{} targeting_eligible={}, frequency_eligible={}, picked={}",
                experiment.id, experiment.name, targeting_eligible, frequency_eligible, picked
            );

            if !targeting_eligible {
                // identify if experiment was in tracking history... copy as it is
                if let Some(existing_experiment) = existing_experiment {
                    tracked_experiments.push(existing_experiment.clone());
                }

                continue;
            }

            // targeting eligible, but not frequency eligible... we just increment invocation for maintaining frequency
            if !frequency_eligible {
                if let Some(existing_experiment) = existing_experiment {
                    let mut existing_experiment = existing_experiment.clone();
                    existing_experiment.total_invocation_count += 1;
                    existing_experiment.invocation_date = req.experiment_start_time;
                    existing_experiment.invocation_version = experiment.version;

                    tracked_experiments.push(existing_experiment);
                }

                // it doesn't make sense to add an experiment in tracked one, if it was not invoked ever
                /*else {
                    experiment_tracking_data::Experiment {
                        short_name: experiment.short_name.to_string(),
                        last_selection_version: 0,
                        last_selection_member_kind: None,
                        last_selection_variation: None,
                        last_selection_date: 0,
                        total_selection_count: 0,
                        last_invocation_date: req.experiment_start_time,
                        last_invocation_version: experiment.version,
                        total_invocation_count: 1,
                    }
                };*/

                continue;
            }

            // user is both targeting and frequency eligible
            let mut existing_variation = None;
            let mut total_selection_count = 0;
            let mut total_invocation_count = 0;

            if let Some(existing_experiment) = existing_experiment {
                total_selection_count = existing_experiment.total_selection_count;
                total_invocation_count = existing_experiment.total_invocation_count;

                // we reduce count, because we will again increment as per new logic
                match existing_experiment.selected_member_kind {
                    ExperimentMemberKind::Control => {
                        experiment.control_size.fetch_sub(1, Ordering::Relaxed);
                    }
                    ExperimentMemberKind::Test => {
                        experiment.test_size.fetch_sub(1, Ordering::Relaxed);
                    }
                }

                // no change in experiment config
                // pick as per existing experiment data
                if existing_experiment.invocation_version == experiment.version {
                    existing_variation = existing_experiment.selected_variation.as_ref();

                    match existing_experiment.selected_member_kind {
                        ExperimentMemberKind::Control => {
                            picked = false;
                        }
                        ExperimentMemberKind::Test => {
                            picked = true;
                        }
                    }
                }
            }

            // make variation selection
            let mut selected_variation = None;
            let selected_member_kind;
            if picked {
                let (data, picked_variation) = AbOptimisationService::sample_variation(experiment.deref(), existing_variation);

                if let Some(picked_variation) = picked_variation.as_ref() {
                    selected_variation = Some(picked_variation.to_string())
                }

                active_experiments.push(ActiveExperiment {
                    short_name: experiment.short_name.to_string(),
                    variation: picked_variation.clone(),
                    data,
                });

                selected_member_kind = ExperimentMemberKind::Test;
                experiment.test_size.fetch_add(1, Ordering::Relaxed);
            } else {
                selected_member_kind = ExperimentMemberKind::Control;
                experiment.control_size.fetch_add(1, Ordering::Relaxed);
            }

            let tracking_experiment = TrackedExperiment {
                short_name: experiment.short_name.to_string(),
                invocation_version: experiment.version,
                selected_member_kind,
                selected_variation,
                selection_date: req.experiment_start_time,
                total_selection_count: total_selection_count + 1,
                invocation_date: req.experiment_start_time,
                total_invocation_count: total_invocation_count + 1,
                selected_version: experiment.version,
            };

            tracked_experiments.push(tracking_experiment);
        }

        let tracking_data = TrackingData {
            experiments: tracked_experiments,
        };

        let experiment_response = ExperimentResponse {
            app_id: &app.id,
            project_id: &proj.id,
            tracking_cookie_name: Some(tracking_cookie_name),
            active_experiments,
            tracking_data: Some(tracking_data.to_string()),
        };

        Ok(experiment_response)
    }

    fn build_tracking_history_map(tracking_history: Option<TrackingData>) -> HashMap<String, TrackedExperiment> {
        match tracking_history {
            None => HashMap::new(),
            Some(tracking_history) => {
                let mut map: HashMap<String, TrackedExperiment> = HashMap::new();

                // experiments would be reverse sorted by version number
                for experiment in tracking_history.experiments.into_iter() {
                    match map.get(&experiment.short_name) {
                        None => {
                            map.insert(experiment.short_name.to_string(), experiment);
                        }
                        Some(existing_value) => {
                            if experiment.invocation_version > existing_value.invocation_version {
                                map.insert(experiment.short_name.to_string(), experiment);
                            }
                        }
                    }
                }

                map
            }
        }
    }

    fn parse_tracking_history(
        route: &HttpRoute,
        req: &ExperimentRequest,
        proj: &Project,
        tracking_cookie_name: &String,
    ) -> Result<Option<TrackingData>, Error> {
        match proj.tracking_method {
            TrackingMethod::Both => {
                // first try with response...
                if let Some(tracking_data) = req.tracking_data.as_ref() {
                    let tracking_data = TrackingDataParser::parse_tracking_data(tracking_data)
                        .with_context(|| format!("Error in deserializing tracking_data in request to TrackingData"))?;

                    Ok(Some(tracking_data))
                } else {
                    // if none, then cookie...
                    AbOptimisationService::parse_tracking_cookie(route, &tracking_cookie_name)
                }
            }
            TrackingMethod::Cookie => {
                // parse with cookie
                AbOptimisationService::parse_tracking_cookie(route, &tracking_cookie_name)
            }
            TrackingMethod::Data => {
                // parse with response
                if let Some(tracking_data) = req.tracking_data.as_ref() {
                    let tracking_data = TrackingDataParser::parse_tracking_data(tracking_data)
                        .with_context(|| format!("Error in deserializing tracking_data in request to TrackingData"))?;

                    Ok(Some(tracking_data))
                } else {
                    Ok(None)
                }
            }
        }
    }

    fn sample_variation(experiment: &core::Experiment, existing_variation_name: Option<&String>) -> (Option<serde_json::Value>, Option<String>) {
        let mut picked_variation: Option<String> = None;

        let mut data: Option<serde_json::Value> = experiment.data.clone();

        // if picked and variations exist, pick a variation
        if let Some(variations) = experiment.variations.as_ref() {
            if variations.len() > 0 {
                match existing_variation_name {
                    None => {
                        let mut rng = rand::thread_rng();
                        let variation_sample_value = experiment.variation_sampler.sample(&mut rng);

                        let mut cumulative_index = 0;

                        for variation in variations.iter() {
                            cumulative_index += variation.size * 100;

                            if variation_sample_value < cumulative_index {
                                // we pick this variant
                                picked_variation = Some(variation.short_name.to_string());

                                data = merge_data(data, variation.data.clone());

                                break;
                            }
                        }
                    }
                    Some(existing_variation_name) => {
                        if let Some(variation) = variations.iter().find(|variation| &variation.short_name == existing_variation_name) {
                            picked_variation = Some(variation.short_name.to_string());

                            data = merge_data(data, variation.data.clone());
                        }
                    }
                }
            }
        }

        (data, picked_variation)
    }

    fn sample_experiment(
        &self,
        req: &ExperimentRequest,
        proj: &core::Project,
        experiment: &core::Experiment,
        tracked_experiment: Option<&TrackedExperiment>,
        guard: &Guard,
    ) -> anyhow::Result<(bool, bool, bool)> {
        // TODO: calculate these seed once
        let experiment_seed = fasthash::murmur3::hash32(&format!("{}/{}", proj.id, experiment.id));

        let mut targeting_eligible: bool = false;
        let mut picked: bool = false;

        // match frequency constraint
        let frequency_eligible;
        match (experiment.frequency_constraint.as_ref(), tracked_experiment) {
            (Some(frequency_constraint), Some(tracked_experiment)) => {
                frequency_eligible = self
                    .evaluate_frequency_constraint(tracked_experiment, &req.script_context, Some(frequency_constraint))
                    .with_context(|| {
                        format!(
                            "frequency_constraint=\"{}\" tracked_experiment={:?} ctx={:?}, experiment={}",
                            frequency_constraint,
                            tracked_experiment,
                            req.context.as_ref(),
                            experiment.short_name
                        )
                    })?;

                info!("Frequency eligible: {}", frequency_eligible);
            }
            _ => {
                frequency_eligible = true;
            }
        }

        for core::Audience {
            name,
            list_id,
            size,
            picked_size,
            script_src,
            ..
        } in experiment.audiences.iter()
        {
            let matches_script = self.evaluate_audience_condition(&req.script_context, script_src.as_ref()).with_context(|| {
                format!(
                    "script_src=\"{:?}\" ctx={:?}, experiment={}",
                    script_src,
                    req.context.as_ref(),
                    experiment.short_name
                )
            })?;

            let mut matches_list = true;
            if let Some(list_id) = list_id {
                let audience_entry = proj
                    .audience_lists
                    .get(list_id, guard)
                    .ok_or_else(|| ApiError::BadRequest(anyhow!("Audience list not found for id: {}", list_id)))?;

                let audience_list = audience_entry.value();
                let audience_list = audience_list.read();

                if audience_list.list.contains(&req.user_id) {
                    matches_list = true;
                } else {
                    matches_list = false;
                }
            }

            let audience_candidate = matches_script && matches_list;

            if audience_candidate {
                targeting_eligible = true;

                // pick as per size spec
                match size {
                    core::SizeSpec::Absolute { value } => {
                        // TODO: better manage absolute
                        if *value > picked_size.fetch_or(0, Ordering::Relaxed) {
                            picked = true;
                        }
                    }
                    core::SizeSpec::Percent { value, ../*, sampler*/ } => {
                        // TODO: calculate these seed once
                        let audience_seed = fasthash::murmur3::hash32(name);
                        let user_hash_bucket = fasthash::murmur3::hash32_with_seed(
                            &req.user_id,
                            experiment_seed + audience_seed,
                        ) % 10000;

                        let user_hash_bucket = user_hash_bucket as i64;

                        // let mut rng = rand::thread_rng();
                        if
                        /*sampler.sample(&mut rng)*/
                        user_hash_bucket < (value * 100) {
                            picked = true;
                        }
                    }
                }

                if picked {
                    picked_size.fetch_add(1, Ordering::Relaxed);
                    break;
                }
            }
        }

        Ok((targeting_eligible, frequency_eligible, picked))
    }

    fn evaluate_frequency_constraint(&self, experiment: &TrackedExperiment, ctx: &jexl_eval::Value, script: Option<&Script>) -> anyhow::Result<bool> {
        if let jexl_eval::Value::Object(context_map) = ctx {
            let mut context_map = context_map.clone();
            let experiment = jexl_eval::to_value(experiment).with_context(|| format!("Error in converting experiment to EvaluationContext"))?;
            context_map.insert("experiment".to_string(), experiment);

            let context = jexl_eval::Value::from(context_map);

            self.script_evaluator.evaluate(script, &context)
        } else {
            Err(anyhow::anyhow!("Context is not a valid Object"))
        }
    }

    fn evaluate_audience_condition(&self, ctx: &jexl_eval::Value, script: Option<&Script>) -> anyhow::Result<bool> {
        self.script_evaluator.evaluate(script, ctx)
    }

    fn tracking_cookie_name(app: &core::App, proj: &core::Project) -> String {
        format!("X-abof-{}-{}", app.short_name, proj.short_name)
    }

    fn parse_tracking_cookie(route: &HttpRoute, tracking_cookie_name: &String) -> anyhow::Result<Option<TrackingData>> {
        match route.req.headers().get(hyper::header::COOKIE) {
            None => {}
            Some(cookie_header) => {
                let cookie_header = cookie_header.to_str().with_context(|| format!("Error in converting cookie header to string"))?;

                for cookie_part in cookie_header.split(";") {
                    let cookie = cookie::Cookie::parse(cookie_part).with_context(|| format!("Error in parsing cookie"))?;

                    if cookie.name() == tracking_cookie_name {
                        let cookie_value = cookie.value();
                        let tracking_data = TrackingDataParser::parse_tracking_data(cookie_value)
                            .with_context(|| format!("Error in deserializing tracking cookie to TrackingData"))?;

                        return Ok(Some(tracking_data));
                    }
                }
            }
        }

        Ok(None)
    }

    pub async fn run<'a>(&'a self, route: &HttpRoute<'a>, body: Body) -> Result<http::Response<Body>, ApiError> {
        let mut req = HttpRequest::value::<ExperimentRequest>(route, body).await?;

        let mut context_map = BTreeMap::new();

        if let Some(ref context) = req.context {
            context_map.insert("ctx".to_string(), jexl_eval::Value::from(context));
        };

        req.script_context = jexl_eval::Value::from(context_map);

        let process_result = |result: Result<ExperimentResponse, ApiError>| {
            result.and_then(|experiment_response| {
                HttpResponse::binary_or_json(route, &experiment_response).and_then(|mut response| {
                    // TODO: depends on whether we are doing cookie tracking
                    if let Some(tracking_cookie_name) = experiment_response.tracking_cookie_name.as_ref() {
                        let cookie_value = match experiment_response.tracking_data.as_ref() {
                            None => "",
                            Some(tracking_data) => tracking_data,
                        };

                        let cookie = cookie::Cookie::build(tracking_cookie_name, cookie_value)
                            .path("/")
                            .permanent()
                            // .secure(true) // TODO: depends on installation setting
                            .http_only(true)
                            .finish();

                        response.headers_mut().append(
                            SET_COOKIE,
                            HeaderValue::from_str(&cookie.to_string()).with_context(|| format!("Error in building header value for cookie"))?,
                        );
                    }

                    Ok(response)
                })
            })
        };

        self.run_internal(route, &req, process_result)
    }
}
