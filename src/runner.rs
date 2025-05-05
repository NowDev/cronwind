use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

use crate::config::Config;
use crate::job::JobRunner;

pub struct Runner {
    jobs: Arc<Mutex<Vec<JobRunner>>>,
}

impl Runner {
    pub async fn new(config: Config) -> Result<Self, Box<dyn Error>> {
        let mut jobs = Vec::new();
        
        for job in config.jobs {
            jobs.push(JobRunner::new(
                job.name,
                job.schedule,
                job.config,
            )?);
        }
        
        Ok(Runner {
            jobs: Arc::new(Mutex::new(jobs)),
        })
    }

    pub async fn start(&self) -> Result<(), Box<dyn Error>> {
        log::info!("Starting job runner...");
        
        loop {
            let mut jobs = self.jobs.lock().await;
            
            for job in jobs.iter_mut() {
                if job.should_run() {
                    match job.execute().await {
                        Ok(output) => {
                            log::info!("Job '{}' completed successfully: {}", job.name, output);
                        }
                        Err(e) => {
                            log::error!("Job '{}' failed: {}", job.name, e);
                        }
                    }
                }
            }
            
            // Check every minute
            drop(jobs);
            sleep(Duration::from_secs(60)).await;
        }
    }
} 