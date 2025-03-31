pub mod error;
pub mod fields;

use crate::request::request::Request;
use error::Result;

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Endpoints(Vec<Endpoint>);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Endpoint {
    pub path: fields::path::Path,
    #[serde()]
    pub method: fields::method::CfgMethod,
    #[serde(rename = "headers")]
    pub headers: fields::headers::Headers,
    pub body: fields::Body,
    pub expected_status: fields::expected_status::ExpectedStatus,
}

#[derive(Debug)]
enum Environment {
    Dev,
    Stage,
    Prod,
}

impl Default for Environment {
    fn default() -> Self {
        Environment::Dev
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(rename = "name")]
    pub name: fields::Name,
    pub environment: fields::environment::Configs,
    pub endpoints: Endpoints,
}

pub fn deserialize_data(path: &Path) -> Result<Config> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let new_config: Config = serde_json::from_str(&contents)?;
    Ok(new_config)
}

impl Config {
    pub fn to_request(self, env: &str) -> Result<Vec<Request>> {
        let mut requests: Vec<Request> = vec![];

        let host = self.environment.into_inner(env)?;

        for entpoint in self.endpoints.0.iter() {
            requests.push(Request {
                name: self.name.clone().into_inner(),
                host: host.to_owned(),
                endpoint: entpoint.path.clone().into_inner(),
                method: entpoint.method.clone(),
                headers: entpoint.headers.clone().into_inner(),
                body: entpoint.body.clone().into(),
                expected_status: entpoint.expected_status.clone().into_inner(),
            });
        }

        Ok(requests)
    }
}
