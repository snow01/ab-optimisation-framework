use std::borrow::BorrowMut;
use std::path::Path;

use config::{Config, File};
#[allow(unused_imports)]
use log::{debug, error, info, warn};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref SETTINGS: RwLock<Config> = RwLock::new(Config::default());
    static ref HTTP_WORKERS: usize = http_workers();
    static ref JSON_PAYLOAD_LIMIT: usize = json_payload_limit();
    static ref SECURE_COOKIE_SETTING: bool = secure_cookie_setting();
}

pub fn settings() -> &'static RwLock<Config> {
    &*SETTINGS
}

fn http_workers() -> usize {
    settings().read().get::<usize>("http_workers").unwrap_or_else(|_| 1)
}

fn json_payload_limit() -> usize {
    settings().read().get::<usize>("json_payload_limit").unwrap_or_else(|_| 1_048_576)
}

fn secure_cookie_setting() -> bool {
    settings().read().get::<bool>("secure_cookie").unwrap_or_else(|_| false)
}

pub fn store_kind() -> String {
    settings().read().get::<String>("store_kind").unwrap_or_else(|_| "local".to_string())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalStoreConfig {
    pub path: String,
}

pub fn local_store_config() -> LocalStoreConfig {
    settings()
        .read()
        .get::<LocalStoreConfig>("store_config")
        .unwrap_or_else(|_| LocalStoreConfig { path: "data".to_string() })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct S3StoreConfig {
    pub bucket: String,
    pub region: String,
    pub path: String,

    #[serde(default = "default_refresh_rate")]
    pub refresh_rate: u64,
}

fn default_refresh_rate() -> u64 {
    10
}

pub fn s3_store_config() -> S3StoreConfig {
    settings().read().get::<S3StoreConfig>("store_config").unwrap_or_else(|_| S3StoreConfig {
        bucket: "abof-data".to_string(),
        region: "ap-south-1".to_string(),
        path: "data".to_string(),
        refresh_rate: 10,
    })
}

pub fn secure_cookie() -> bool {
    *SECURE_COOKIE_SETTING
}

pub fn load_global_config(base_dir: &str, env: &str) -> anyhow::Result<()> {
    let mut write_guard = settings().write();
    let config: &mut Config = write_guard.borrow_mut();

    // Start off by merging in the "default" configuration file
    merge_config(config, format!("{}/service-default.yml", base_dir))?;
    merge_config(config, format!("{}/service-{}.yml", base_dir, env))?;

    // if let Some(current_binary) = std::env::current_exe()?.file_name() {
    //     let current_binary = current_binary.to_string_lossy();
    //     merge_config(config, format!("{}/{}-default.yml", base_dir, current_binary))?;
    //     merge_config(config, format!("{}/{}-{}.yml", base_dir, current_binary, env))?;
    // }

    Ok(())
}

fn merge_config(config: &mut Config, path: String) -> anyhow::Result<()> {
    if Path::new(&path).exists() {
        info!("Loading config: {}", path);
        config.merge(File::with_name(path.as_str()))?;
    } else {
        warn!("No config found for: {}", path);
    }

    Ok(())
}
