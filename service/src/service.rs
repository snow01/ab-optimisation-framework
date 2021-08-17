use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use crossbeam_epoch as epoch;
use crossbeam_skiplist::SkipList;
#[allow(unused_imports)]
use log::{debug, error, info, warn};
use parking_lot::RwLock;

use crate::core::{App, ScriptEvaluator};
use crate::experiment_store::ExperimentStore;
use crate::server::{ServiceBuilder, ServiceDaemon, SHUTDOWN};
use crate::settings;

pub struct AbOptimisationService {
    pub apps: SkipList<String, RwLock<App>>,
    pub script_evaluator: Arc<ScriptEvaluator>,
    pub experiment_store: ExperimentStore,
}

pub struct AbOptimisationServiceDaemon {}

pub struct AbOptimisationServiceBuilder {}

#[async_trait]
impl ServiceDaemon<AbOptimisationService> for AbOptimisationServiceDaemon {
    async fn start(&self, service: Arc<AbOptimisationService>) {
        let mut interval = tokio::time::interval(Duration::from_secs(settings::s3_store_config().refresh_rate));

        while !SHUTDOWN.load(Ordering::Relaxed) {
            interval.tick().await;
            info!("Refreshing data: {:?}", chrono::Local::now());
            if let Err(err) = service.load_data() {
                error!("Error in loading data: {:?}", err);
            }
        }
    }
}

impl ServiceBuilder<AbOptimisationService, AbOptimisationServiceDaemon> for AbOptimisationServiceBuilder {
    fn build(self) -> anyhow::Result<(AbOptimisationService, Option<AbOptimisationServiceDaemon>)> {
        let apps = SkipList::new(epoch::default_collector().clone());
        let script_evaluator = Arc::new(ScriptEvaluator::new());

        let experiment_store = match crate::settings::store_kind().as_str() {
            "local" => ExperimentStore::new_local_store(settings::local_store_config()),
            "s3" => ExperimentStore::new_s3_store(settings::s3_store_config()),
            _ => Err(anyhow::anyhow!("Unknown store kind: {}", settings::store_kind())),
        }?;

        let service = AbOptimisationService {
            apps,
            script_evaluator,
            experiment_store,
        };

        service.load_data()?;

        Ok((service, Some(AbOptimisationServiceDaemon {})))
    }
}

impl AbOptimisationService {
    pub fn load_data(&self) -> anyhow::Result<()> {
        self.experiment_store.load_apps(&self)?;
        self.experiment_store.load_projects(&self)?;
        self.experiment_store.load_audience_lists(&self)?;
        self.experiment_store.load_experiments(&self)?;

        Ok(())
    }
}
