use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use serde::{ Deserialize, Serialize };

use crate::utils;

// Config entry
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub jobs: Vec<Job>,
}

// Job Inputs
#[derive(Serialize, Deserialize, Debug)]
pub struct Job {
    pub name: String,
    pub schedule: String,
    pub kind: JobKind,
    pub config: JobConfig,
    pub outputs: Vec<Output>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum JobConfig {
    Command {
        command: String,
    },
    Request {
        method: HttpMethod,
        url: String,
        body: Option<String>,
        headers: Option<HashMap<String, String>>,
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum JobKind {
    Command,
    Request,
}

// Job Outputs
#[derive(Serialize, Deserialize, Debug)]
pub struct Output {
    #[serde(flatten)]
    pub kind: OutputKind,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "kind", content = "config")]
pub enum OutputKind {
    #[serde(rename = "file")]
    File { path: String },
    #[serde(rename = "request")]
    Request { url: String },
}

pub fn load_config(path: &str) -> Result<Config, Box<dyn Error>> {
    if !Path::new(path).exists() {
        let file = File::create(path)?;
        let config = Config { jobs: vec![] };
        serde_json::to_writer(&file, &config)?;
    }

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut config: Config = serde_json::from_reader(reader)?;

    let env_vars = HashMap::from_iter(std::env::vars());

    // Expand env ${VARS} in body and header data.
    for job in &mut config.jobs {
        if let JobConfig::Request { body, headers, .. } = &mut job.config {
            if let Some(body) = body {
                *body = utils::expand_env_vars(&body, &env_vars);
            }
            if let Some(headers) = headers {
                for (_key, value) in headers {
                    *value = utils::expand_env_vars(&value, &env_vars);
                }
            }
        }
    }

    Ok(config)
}
