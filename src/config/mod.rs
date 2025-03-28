pub mod error;
pub mod fields;

use crate::request::request::Request;
use error::{Error, Result};

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(rename = "name")]
    pub name: fields::Name,
    pub environment: fields::environment::Configs,
    #[serde()]
    pub method: fields::method::CfgMethod,
    #[serde(rename = "headers")]
    pub headers: fields::headers::Headers,
    pub body: fields::Body,
    pub expected_status: fields::expected_status::ExpectedStatus,
}

pub fn deserialize_data(path: &Path) -> Result<Config> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let new_config: Config = serde_json::from_str(&contents)?;
    Ok(new_config)
}

impl Config {
    pub fn to_request(self, env: &str) -> Result<Request> {
        let host = self.environment.into_inner(env)?;
        Ok(Request {
            name: self.name.into_inner(),
            host,
            method: self.method,
            headers: self.headers.into_inner(),
            body: self.body.into(),
            expected_status: self.expected_status.into_inner(),
        })
    }
}
