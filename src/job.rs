use cron::Schedule;
use std::error::Error;
use std::str::FromStr;
use tokio::process::Command;
use crate::config::{JobConfig, HttpMethod};
use chrono::{DateTime, Utc};
use reqwest::Method;

pub struct JobRunner {
    pub name: String,
    pub schedule: Schedule,
    pub config: JobConfig,
    pub last_run: Option<DateTime<Utc>>,
}

impl JobRunner {
    pub fn new(name: String, schedule: String, config: JobConfig) -> Result<Self, Box<dyn Error>> {
        if let Err(_) = Schedule::from_str(&schedule) {
            log::error!("Invalid cron schedule: '{}'\n-> Usage: sec min hour day_of_month month day_of_week year", schedule);
            std::process::exit(1);
        }

        let schedule = Schedule::from_str(&schedule)?;
        
        Ok(JobRunner {
            name,
            schedule,
            config,
            last_run: None,
        })
    }

    pub fn should_run(&self) -> bool {
        let now = Utc::now();
        
        // Cron match
        if self.schedule.includes(now) {
            return true;
        }
        
        false
    }

    pub async fn execute(&mut self) -> Result<String, Box<dyn Error>> {
        log::info!("Executing job: {}", self.name);

        match &self.config {
            JobConfig::Command { command } => {
                let output = if cfg!(target_os = "windows") {
                    Command::new("cmd")
                        .args(["/C", command])
                        .output()
                        .await?
                } else {
                    Command::new("sh")
                        .args(["-c", command])
                        .output()
                        .await?
                };
        
                self.last_run = Some(Utc::now());
        
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        
                if !output.status.success() {
                    return Err(format!("Command failed: {}", stderr).into());
                }
        
                Ok(stdout)
            }
            JobConfig::Request { method, url } => {
                let client = reqwest::Client::new();
                let method = match method {
                    HttpMethod::Get => Method::GET,
                    HttpMethod::Post => Method::POST,
                    HttpMethod::Put => Method::PUT,
                    HttpMethod::Delete => Method::DELETE,
                    HttpMethod::Patch => Method::PATCH,
                    HttpMethod::Head => Method::HEAD,
                };

                let response = client.request(method, url).send().await?;
                
                if !response.status().is_success() {
                    return Err(format!("Request failed with status: {}", response.status()).into());
                }

                let body = response.text().await?;
                Ok(body)
            }
        }
    }
} 