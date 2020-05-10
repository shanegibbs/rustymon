use crate::common::*;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub checks: HashMap<String, CheckConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum CheckConfig {
    Http(crate::http::Config),
    // Ping(crate::ping::Config),
}

// impl CheckConfig {
//     pub fn as_check(
//         &self,
//     ) -> Result<Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error>>>>>, Box<dyn Error>>
//     {
//         match self {
//             CheckConfig::Http(c) => c.check(),
//         }
//     }
// }

pub fn read_from_file<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(serde_yaml::from_reader(reader)?)
}
