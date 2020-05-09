use futures::Future;
use log::{info, trace};
use reqwest::{header, Method};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::pin::Pin;
use std::time::Duration;
use tokio::time;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub method: String,
    pub url: String,
}

// impl Config {
//     pub fn check(
//         &self,
//     ) -> Result<Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error>>>>>, Box<dyn Error>>
//     {
//         async {
//             let client = Client::new().unwrap();
//             // Ok(Box::pin(client.check(self.url.clone())))
//             client.check(self.url.clone()).await?
//         }
//     }
// }

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

        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_static("rustymon"),
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

    pub async fn check(
        &self,
        status: &crate::status::Status,
        url: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut interval = time::interval(Duration::from_millis(1000));

        let method = Method::GET;

        loop {
            interval.tick().await;

            trace!("Checking {}", url);

            let result = self
                .client
                .request(method.clone(), &url)
                .send()
                .await?
                .status();

            info!("Result: {} {}", result, url);

            status.handle();
        }
    }
}
