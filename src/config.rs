use serde::{ Deserialize, Serialize };

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub jobs: Vec<Job>,
    pub outputs: Vec<Output>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Job {
    pub name: String,
    pub schedule: String,
    pub command: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Output {
    pub name: String,
    #[serde(flatten)]
    pub kind: OutputKind,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "config")]
pub enum OutputKind {
    #[serde(rename = "file")]
    File { path: String },
    #[serde(rename = "discord")]
    Discord { url: String },
}

pub fn load_config(path: &str) -> Result<Config, Box<dyn Error>> {
    // Check if the file exists
    if !Path::new(path).exists() {
    // Create a new file
    let file = File::create(path)?;

    // Write the default config to the file
    let config = Config { jobs: vec![], outputs: vec![] };
        serde_json::to_writer(&file, &config)?;
    }

    let file = File::open(path)?;

    // Create a buffered reader for the file
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `Config`.
    let config = serde_json::from_reader(reader)?;

    // Return the config
    Ok(config)
}
