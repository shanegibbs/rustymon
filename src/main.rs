// use config::Config;
use log::{debug, info, warn, error};
use futures::future;

mod http;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    debug!("Setting up");

    let client = http::Client::new()?;

    let work = vec![
        Box::pin(client.check("https://www.vividseats.com")),
        Box::pin(client.check("https://skybox.vividseats.com/health")),
        Box::pin(client.check("https://skybox.vividseats.com/services/health")),
        ];

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
