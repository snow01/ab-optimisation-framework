use std::sync::Arc;

use anyhow::{Context, Error};
use crossbeam_epoch as epoch;
use crossbeam_skiplist::SkipList;
#[allow(unused_imports)]
use log::{debug, error, info, warn};
use parking_lot::RwLock;
use regex::Regex;
use serde::Serialize;
use walkdir::WalkDir;

use crate::config::settings;
use crate::core::{App, AudienceList, Experiment, Project, ScriptEvaluator};
use crate::server::ServiceBuilder;

pub struct AbOptimisationService {
    pub apps: SkipList<String, RwLock<App>>,
    pub script_evaluator: Arc<ScriptEvaluator>,
}

pub struct AbOptimisationServiceBuilder {}

impl ServiceBuilder<AbOptimisationService> for AbOptimisationServiceBuilder {
    fn build(self) -> anyhow::Result<AbOptimisationService> {
        let service = AbOptimisationService {
            apps: SkipList::new(epoch::default_collector().clone()),
            script_evaluator: Arc::new(ScriptEvaluator::new()),
        };

        load_all_apps(&service)?;
        load_all_projects(&service)?;
        load_all_experiments(&service)?;
        load_all_audience_lists(&service)?;

        Ok(service)
    }
}
