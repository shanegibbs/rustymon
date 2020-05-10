use crate::common::*;
use reqwest::{header, Method};
use std::time::Duration;
use tokio::time;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub method: String,
    pub url: String,
    // #[serde(flatten)]
    // pub common: CommonCheckConfig,
}

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct CommonCheckConfig {
//     pub interval: usize,
// }

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

    pub async fn do_check(&self, url: &str) -> Result<(), Box<dyn std::error::Error>> {
        trace!("Checking {}", url);

        let method = Method::GET;

        self.client
            .request(method.clone(), url)
            .send()
            .await
            .map_err(|e| {
                debug!("Result: {} {}", e, url);
                format!("failed: {}", e).into()
            })
            .and_then(|r| {
                if r.status() == 200 {
                    trace!("Result: {} {}", r.status(), url);
                    Ok(())
                } else {
                    debug!("Result: {} {}", r.status(), url);
                    Err("not 200".into())
                }
            })
    }

    pub async fn check(
        &self,
        holder: crate::Holder<Config>,
        status: &crate::status::Status,
    ) -> Result<(), Box<dyn Error>> {
        let mut interval = time::interval(Duration::from_millis(20000));

        loop {
            interval.tick().await;
            let result = self.do_check(holder.config.url.as_str()).await;
            status.handle(&holder, result);
        }
    }
}
