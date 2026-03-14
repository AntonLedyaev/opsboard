use std::fmt;
use rand::{RngExt};
use chrono::{DateTime, Utc};

pub struct Job {
    pub id: u32,
    pub name: &'static str,
    pub retry_count: u32,
    pub status: JobStatus,
    pub started_at: Option<String>,
    pub finished_at: Option<String>,
}

#[derive(PartialEq, Eq)]
pub enum JobStatus {
    Created,
    Queued,
    Running,
    Done,
    Failed
}

impl fmt::Display for JobStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JobStatus::Created => write!(f, "Created"),
            JobStatus::Queued => write!(f, "Queued"),
            JobStatus::Done => write!(f, "Done"),
            JobStatus::Failed => write!(f, "Failed"),
            JobStatus::Running => write!(f, "Running")
        }
    }
}

impl Job {
    pub fn create(name: &'static str) -> Job {
        let id = rand::rng().random();
        println!("Job {} created, Job Id: {}", name, id);
        Job { name, id, retry_count: 0, status: JobStatus::Created, started_at: None, finished_at: None }
    }
    pub fn start(&mut self) {
        let now = std::time::SystemTime::now();
        let datetime: DateTime<Utc> = now.into();

        self.started_at = Some(datetime.format("%Y-%m-%d %H:%M:%S").to_string());
        self.status = JobStatus::Running;
        self.print_status();
    }
    pub fn check_is_failed(&mut self) {
        if (self.retry_count > 3) {
            self.status = JobStatus::Failed;
            self.print_status();
        }
    }
    pub fn retry(&mut self) {
        self.retry_count += 1;
        println!("Job {} retried, Job Id: {}, Retry Count: {}", self.name, self.id, self.retry_count);
        self.check_is_failed();
    }
    pub fn print_status(&self) {
        println!("Job Id: {}, Job Name: {}, Job Status: {}",self.id, self.name, self.status);
    }
}

pub fn print_job_name(job: &Job) {
    println!("{} {} {} {} ", job.name, job.id, job.retry_count, job.status);
}
