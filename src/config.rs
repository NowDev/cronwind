use serde::{ Deserialize, Serialize };

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

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
#[serde(untagged)]
pub enum JobConfig {
    Command {
        command: String,
    },
    Request {
        url: String,
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
    let config = serde_json::from_reader(reader)?;

    Ok(config)
}
