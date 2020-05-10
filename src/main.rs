use crate::common::*;
use futures::future;
use std::pin::Pin;

mod config;
mod http;
// mod ping;
mod status;

mod common {
    pub(crate) use crate::config::CheckConfig;
    pub(crate) use log::{debug, error, info, trace};
    pub(crate) use serde::{Deserialize, Serialize};
    pub(crate) use std::error::Error;
}

#[derive(Debug)]
pub struct Holder<T> {
    pub name: String,
    pub check_config: CheckConfig,
    pub config: T,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let config_file = "rustymon.yaml";
    let config = config::read_from_file(config_file)
        .map_err(|e| format!("Failed to load config {}: {}", config_file, e))?;
    let s = serde_yaml::to_string(&config)?;
    info!("Config {}", s);

    let status = &status::Status::new();

    let http_client = http::Client::new()?;
    // let ping_checker = ping::Checker::new();

    use future::Future;
    use std::error::Error;

    let work: Vec<_> = {
        macro_rules! as_dyn {
            ($f:expr) => {
                Box::pin($f) as Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>>>>
            };
        }

        config
            .checks
            .iter()
            .map(|(name, check_config)| {
                use CheckConfig::*;
                match check_config {
                    Http(config) => as_dyn!(http_client.check(
                        Holder {
                            name: name.into(),
                            check_config: check_config.clone(),
                            config: config.clone(),
                        },
                        status
                    )),
                    // Ping(c) => as_dyn!(ping::check(c, &status)),
                }
            })
            .collect()
    };

    let (done, _idx, _remaining) = future::select_all(work).await;

    match done {
        Ok(_) => {}
        Err(err) => {
            error!("processing failed; error = {}", err);
        }
    }

    Ok(())
}
