// use config::Config;
use futures::future;
use log::{error, info};

mod config;
mod http;
mod status;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let config_file = "rustymon.yaml";
    let config = config::read_from_file(config_file)
        .map_err(|e| format!("Failed to load config {}: {}", config_file, e))?;
    let s = serde_yaml::to_string(&config)?;
    info!("Config {}", s);

    let status = status::Status::new();
    let http_client = http::Client::new()?;

    let work: Vec<_> = config
        .checks
        .iter()
        .map(|c| {
            Box::pin(match c {
                config::CheckConfig::Http(c) => http_client.check(&status, c.url.clone()),
            })
        })
        .collect();

    let (done, _idx, _remaining) = future::select_all(work).await;

    match done {
        Ok(_) => {
            // do something with the values
        }
        Err(err) => {
            error!("processing failed; error = {}", err);
        }
    }

    Ok(())
}
