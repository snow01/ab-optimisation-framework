//#![allow(unused_imports)]
//#![allow(unused_variables)]
#![allow(dead_code)]
#![recursion_limit = "8192"]

#[macro_use]
extern crate clap;
// #[macro_use]
pub extern crate derive_more;
// #[macro_use]
extern crate diesel;
#[macro_use]
extern crate enumset;
extern crate gethostname;
// #[macro_use]
extern crate itertools;
extern crate jemallocator;
#[macro_use]
pub extern crate lazy_static;
#[macro_use]
extern crate pest_derive;
// #[macro_use]
pub extern crate serde_json;

use std::path::Path;

use anyhow::Context;
#[allow(unused_imports)]
use log::{debug, error, info, warn};
use log4rs;

// crate specific imports
use crate::config::load_global_config;
use crate::server::{start_http_server, ServiceBuilder};
use crate::service::{AbOptimisationService, AbOptimisationServiceBuilder};

mod api;
mod config;
mod core;
mod experiment_store;
mod server;
mod service;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), anyhow::Error> {
    run_main(AbOptimisationServiceBuilder {}).await
}

pub async fn run_main<AppBuilder>(app_builder: AppBuilder) -> anyhow::Result<()>
where
    AppBuilder: 'static + ServiceBuilder<AbOptimisationService>,
{
    // build command line
    let yaml = load_yaml!("cli.yml");
    let clap = clap::App::from_yaml(yaml).version(crate_version!());
    let matches = clap.get_matches();

    // let _postgres_address: &str = &value_t!(matches, "postgres", String).unwrap();
    let log4rs_prop_file: &str = &value_t!(matches, "log4rs_prop_file", String).unwrap();
    let config_dir: &str = &value_t!(matches, "config_dir", String).unwrap();
    let env: &str = &value_t!(matches, "env", String).unwrap();

    // setup logging
    log4rs::init_file(Path::new(log4rs_prop_file), Default::default())
        .with_context(|| format!("Error in opening log file: {}", log4rs_prop_file))
        .unwrap();

    load_global_config(config_dir, env)?;

    match matches.subcommand_name() {
        Some("start") => {
            let arg_match = matches.subcommand_matches("start").unwrap();
            let addr: &str = &value_t!(arg_match, "addr", String).unwrap();

            start_http_server(addr, app_builder).await
        }
        None => anyhow::bail!("No sub command match"),
        _ => anyhow::bail!("Some other sub command was used"),
    }
}
