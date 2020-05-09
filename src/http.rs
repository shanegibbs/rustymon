// use crate::Config;
use log::{debug, info};
use reqwest::header;
use std::time::Duration;
use tokio::time;

pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub fn new() -> Result<Self, reqwest::Error> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

        // if let Some(token) = &config.k8s_token {
        //     let auth_value = format!("Bearer {}", token);
        //     headers.insert(
        //         header::AUTHORIZATION,
        //         header::HeaderValue::from_str(&auth_value).unwrap(),
        //     );
        // }

        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

        let builder = {
            let b = reqwest::Client::builder()
                .default_headers(headers)
                .timeout(Duration::from_secs(10));
            b
        };

        Ok(Client {
            client: builder.build()?,
        })
    }

    pub async fn check(&self, url: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut interval = time::interval(Duration::from_millis(1000));

        loop {
            interval.tick().await;

            debug!("Checking {}", url);

            let result = self
                .client
                .head(url)
                .send()
                .await?
                .status();

            info!("Result: {} {}", result, url);
        }
    }
}
