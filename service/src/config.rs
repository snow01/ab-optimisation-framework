use std::borrow::BorrowMut;
use std::path::Path;

use config::{Config, File};
#[allow(unused_imports)]
use log::{debug, error, info, warn};
use parking_lot::RwLock;

lazy_static! {
    static ref SETTINGS: RwLock<Config> = RwLock::new(Config::default());
    static ref HTTP_WORKERS: usize = http_workers();
    static ref JSON_PAYLOAD_LIMIT: usize = json_payload_limit();
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
