use log::{info, trace, warn};
use reqwest::{header, Method};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::pin::Pin;
use std::time::Duration;
use tokio::time;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub address: String,
}

pub struct Checker {
    pinger: tokio_ping::Pinger,
}

impl Checker {
    pub async fn new() -> Result<Self, tokio_ping::Error> {
        Ok(Check {
            pinger: tokio_ping::Pinger::new().await?,
        })
    }
}

pub async fn check(
    config: &Config,
    status: &crate::status::Status,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut interval = time::interval(Duration::from_millis(1000));
    loop {
        interval.tick().await;
        info!("ping");
        status.handle();
    }
}
