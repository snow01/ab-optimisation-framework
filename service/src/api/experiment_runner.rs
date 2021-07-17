use std::collections::HashMap;
use std::sync::atomic::Ordering;

use anyhow::{anyhow, Context};
use crossbeam_epoch as epoch;
use crossbeam_epoch::Guard;
use hyper::header::SET_COOKIE;
use hyper::http::HeaderValue;
use hyper::Body;
#[allow(unused_imports)]
use log::{debug, error, info, warn};
use pyo3::prelude::PyModule;
use pyo3::types::IntoPyDict;
use pyo3::Python;
use pythonize::pythonize;
use rand::distributions::Distribution;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::api::common::merge_data;
use crate::api::experiment_tracking_data;
use crate::core;
use crate::server::{HttpError, HttpRequest, HttpResponse, HttpRoute};
use crate::service::AbOptimisationService;

use super::common::ExperimentMemberKind;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentRequest {
    pub app_id: String,
    pub project_id: String,
    pub user_id: String,
    // pub experiment_history: Option<ExperimentHistory>,
    pub context: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentResponse {
    pub app_id: String,
    pub project_id: String,
    pub tracking_cookie_name: String,
    pub active_experiments: Vec<ActiveExperiment>,
    // pub context_data: ContextData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveExperiment {
    pub short_name: String,
    pub variation: Option<String>,
    pub data: Option<JsonValue>,
}

impl AbOptimisationService {
    fn run(
        &self,
        route: &HttpRoute<'_>,
        req: &ExperimentRequest,
    ) -> Result<(experiment_tracking_data::TrackingData, ExperimentResponse), HttpError> {
        let guard = &epoch::pin();

        // check app
        self.apps
            .get(&req.app_id, guard)
            .ok_or_else(|| HttpError::NotFound(format!("App:{} not found", req.app_id)))
            .and_then(|app_entry| Ok(app_entry.value()))
            // check project
            .and_then(|app| {
                app.projects
                    .get(&req.project_id, guard)
                    .ok_or_else(|| {
                        HttpError::NotFound(format!("Project:{} not found", req.project_id))
                    })
                    .and_then(|proj_entry| Ok(proj_entry.value()))
                    .and_then(|proj| {
                        AbOptimisationService::run_for_project(route, &req, app, proj, guard)
                    })
            })
    }

    fn run_for_project(
        route: &HttpRoute<'_>,
        req: &ExperimentRequest,
        app: &core::App,
        proj: &core::Project,
        guard: &Guard,
    ) -> Result<(experiment_tracking_data::TrackingData, ExperimentResponse), HttpError> {
        let tracking_cookie_name = AbOptimisationService::tracking_cookie_name(app, proj);

        let mut tracked_experiments = Vec::<experiment_tracking_data::Experiment>::new();

        // info!("Request headers: {:?}", route.req.headers());

        // identify existing experiments if any - parse cookies for this
        let existing_experiments =
            AbOptimisationService::parse_tracking_cookie(route, &tracking_cookie_name)?;

        // LATER: parse cookie / header / context into final context object - not needed immediately...

        let mut active_experiments = Vec::<ActiveExperiment>::new();

        // go over all experiments for the project (later to be done in experiment group order)
        for exp_entry in proj.experiments.iter(guard) {
            let experiment = exp_entry.value();

            if experiment.inactive {
                continue;
            }

            // todo: check for schedule

            match existing_experiments.get(&experiment.short_name) {
                None => {}
                Some(existing_experiment) => {
                    if existing_experiment.version == experiment.version {
                        tracked_experiments.push(existing_experiment.clone());

                        if existing_experiment.member_kind == ExperimentMemberKind::Control {
                            continue;
                        }

                        // if existing experiment and same version - keep this experiment
                        let mut data = experiment.data.clone();

                        let mut selected_variation = None;
                        if let Some(existing_variation) = existing_experiment.variation.as_ref() {
                            for variation in experiment.variations.iter() {
                                if existing_variation == &variation.short_name {
                                    selected_variation = Some(variation.short_name.to_string());

                                    data = merge_data(data, variation.data.clone());

                                    break;
                                }
                            }
                        }

                        active_experiments.push(ActiveExperiment {
                            short_name: experiment.short_name.to_string(),
                            variation: selected_variation,
                            data,
                        });

                        continue;
                    } else {
                        // todo: if existing experiment and version change - do something extra ?
                        // version change can be because of
                        //      - change in audience spec or size
                        //      - change in variations or size
                        // if audience spec changed -- user may not be anymore part of the eligible set.
                        // if audience size increased -- it is fine to keep user
                        // if audience size decreased -- we may need to re-evaluate user's eligibility
                        // if change in variations -- experiment is kept, but user may need to be assigned to different variant
                        // if variant size increased -- experiment is kept, variation is kept
                        // if variant size decreased -- experiment is kept, reassign variant
                    }
                }
            }

            // else go over all target audience for the experiment, evaluate if user is part and pick for target size
            let (eligible, picked) =
                AbOptimisationService::sample_experiment(&req, proj, experiment, guard)?;

            // if not eligible for experiment, skip
            if !eligible {
                continue;
            }

            // if matches audience but not picked, set user in control group
            let tracking_experiment;
            if picked {
                // if matches and picked, set user in test group
                let (data, picked_variation) = AbOptimisationService::sample_variation(experiment);

                active_experiments.push(ActiveExperiment {
                    short_name: experiment.short_name.to_string(),
                    variation: picked_variation.clone(),
                    data,
                });

                experiment.test_size.fetch_add(1, Ordering::Relaxed);

                tracking_experiment = experiment_tracking_data::Experiment {
                    short_name: experiment.short_name.to_string(),
                    version: experiment.version,
                    member_kind: ExperimentMemberKind::Test,
                    variation: picked_variation,
                };
            } else {
                experiment.control_size.fetch_add(1, Ordering::Relaxed);

                tracking_experiment = experiment_tracking_data::Experiment {
                    short_name: experiment.short_name.to_string(),
                    version: experiment.version,
                    member_kind: ExperimentMemberKind::Control,
                    variation: None,
                };
            }

            tracked_experiments.push(tracking_experiment);
        }

        let tracking_data = experiment_tracking_data::TrackingData {
            experiments: tracked_experiments,
        };

        let experiment_response = ExperimentResponse {
            app_id: proj.app.to_string(),
            project_id: proj.id.to_string(),
            tracking_cookie_name,
            active_experiments,
            // context_data: ExperimentHistory {},
        };

        Ok((tracking_data, experiment_response))
    }

    fn sample_variation(
        experiment: &core::Experiment,
    ) -> (Option<serde_json::Value>, Option<String>) {
        let mut picked_variation: Option<String> = None;

        let mut data: Option<serde_json::Value> = experiment.data.clone();

        let mut rng = rand::thread_rng();
        let variation_sample_value = experiment.variation_sampler.sample(&mut rng);

        let mut cumulative_index = 0;

        // if picked and variations exist, pick a variation
        if experiment.variations.len() > 0 {
            for variation in experiment.variations.iter() {
                cumulative_index += variation.size * 100;

                if variation_sample_value < cumulative_index {
                    // we pick this variant
                    picked_variation = Some(variation.short_name.to_string());

                    data = merge_data(data, variation.data.clone());

                    break;
                }
            }
        }

        (data, picked_variation)
    }

    fn sample_experiment(
        req: &ExperimentRequest,
        proj: &core::Project,
        experiment: &core::Experiment,
        guard: &Guard,
    ) -> anyhow::Result<(bool, bool)> {
        let mut eligible: bool = false;
        let mut picked: bool = false;

        for core::Audience {
            audience,
            size,
            picked_size,
            ..
        } in experiment.audiences.iter()
        {
            let mut candidate = true;

            match audience {
                core::AudienceSpec::Script { script_src } => {
                    match script_src {
                        None => {
                            // matches all
                            candidate = true;
                        }
                        Some(condition) => {
                            // evaluate expression
                            candidate = AbOptimisationService::evaluate_condition(
                                req.context.as_ref(),
                                condition,
                            )
                            .with_context(|| {
                                format!(
                                    "script_src=\"{}\" ctx={:?}, experiment={}",
                                    condition,
                                    req.context.as_ref(),
                                    experiment.short_name
                                )
                            })?;
                        }
                    }
                }
                core::AudienceSpec::List { list_id: id } => {
                    let audience_entry = proj.audience_lists.get(id, guard).ok_or_else(|| {
                        HttpError::BadRequest(anyhow!("Audience Spec not found for id: {}", id))
                    })?;

                    if audience_entry.value().list.contains(&req.user_id) {
                        candidate = true;
                    }
                }
            }

            if candidate {
                eligible = true;

                // pick as per size spec
                match size {
                    core::SizeSpec::Absolute { value } => {
                        if *value > picked_size.fetch_or(0, Ordering::Relaxed) {
                            picked = true;
                        }
                    }
                    core::SizeSpec::Percent { value, sampler } => {
                        let mut rng = rand::thread_rng();
                        if sampler.sample(&mut rng) < value * 100 {
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

        Ok((eligible, picked))
    }

    fn evaluate_condition(ctx: Option<&JsonValue>, script_src: &str) -> anyhow::Result<bool> {
        Python::with_gil(|py| -> anyhow::Result<bool> {
            let python_ctx = pythonize(py, &ctx)
                .with_context(|| format!("Error in converting context into python object"))?;
            let locals = [("ctx", python_ctx)].into_py_dict(py);

            let fns = AbOptimisationService::build_common_script_module(py)?;

            let globals = [("fns", fns)].into_py_dict(py);

            let result = py
                .eval(script_src, Some(&globals), Some(&locals))
                .with_context(|| format!("Error in evaluating script"))?;

            result
                .extract()
                .with_context(|| format!("Error in extracting boolean result"))
        })
    }

    fn build_common_script_module(py: Python) -> anyhow::Result<&PyModule> {
        PyModule::from_code(
            py,
            r#"
from packaging.version import parse as parse_version
                "#,
            "fns.py",
            "fns",
        )
        .with_context(|| "Error in building common python modules")
    }

    fn tracking_cookie_name(app: &core::App, proj: &core::Project) -> String {
        format!("X-abof-{}-{}", app.short_name, proj.short_name)
    }

    fn parse_tracking_cookie(
        route: &HttpRoute,
        tracking_cookie_name: &String,
    ) -> anyhow::Result<HashMap<String, experiment_tracking_data::Experiment>> {
        let mut existing_experiments =
            HashMap::<String, experiment_tracking_data::Experiment>::new();

        match route.req.headers().get(hyper::header::COOKIE) {
            None => {}
            Some(cookie_header) => {
                let cookie_header = cookie_header
                    .to_str()
                    .with_context(|| format!("Error in converting cookie header to string"))?;

                for cookie_part in cookie_header.split(";") {
                    let cookie = cookie::Cookie::parse(cookie_part)
                        .with_context(|| format!("Error in parsing cookie"))?;

                    // info!("Cookie name we got: {}", cookie.name());

                    if cookie.name() == tracking_cookie_name {
                        let cookie_value = /*base64::decode(*/cookie.value()/*)
                        .with_context(|| format!("Error in decoding cookie data from base64"))?*/;
                        let experiment_cookie: experiment_tracking_data::TrackingData =
                            experiment_tracking_data::ExperimentTrackingCookieParser::parse_str(
                                cookie_value,
                            )
                            .with_context(|| {
                                format!("Error in deserializing to Experiment Cookie")
                            })?;

                        // info!("Experiment Cookie: {:?}", experiment_cookie);

                        for exp in experiment_cookie.experiments {
                            existing_experiments.insert(exp.short_name.to_string(), exp);
                        }
                    }
                }
            }
        }

        Ok(existing_experiments)
    }
}

pub async fn run(
    app: &AbOptimisationService,
    route: &HttpRoute<'_>,
    body: Body,
) -> anyhow::Result<http::Response<Body>> {
    let req = HttpRequest::value::<ExperimentRequest>(route, body).await?;

    match app.run(route, &req) {
        Ok((tracking_data, experiment_response)) => {
            HttpResponse::binary_or_json(route, &experiment_response).and_then(|mut response| {
                let cookie_value = tracking_data.to_string();
                let cookie =
                    cookie::Cookie::build(experiment_response.tracking_cookie_name, cookie_value)
                        .path("/")
                        .permanent()
                        // .secure(true)
                        .http_only(true)
                        .finish();

                response
                    .headers_mut()
                    .append(SET_COOKIE, HeaderValue::from_str(&cookie.to_string())?);

                Ok(response)
            })
        }
        Err(error) => error.into(),
    }
}
