use std::fmt;
use std::time::SystemTime;
use rand::{RngExt};

pub struct Job {
    id: u32,
    name: String,
    retry_count: u32,
    status: JobStatus,
    started_at: Option<SystemTime>,
    finished_at: Option<SystemTime>,
    max_retry_count: u32,
}

#[derive(PartialEq, Eq)]
pub enum JobStatus {
    Queued,
    Running,
    Done,
    Failed
}

const MAX_RETRY_COUNT: u32 = 3;

fn format_time_to_string(timestamp: Option<SystemTime>) -> String {
     match timestamp {
        Some(time) => time.elapsed().unwrap().as_micros().to_string(),
        None => { "No finish time".to_owned() }
    }
}

impl fmt::Display for JobStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JobStatus::Queued => write!(f, "Queued"),
            JobStatus::Running => write!(f, "Running"),
            JobStatus::Done => write!(f, "Done"),
            JobStatus::Failed => write!(f, "Failed")
        }
    }
}

impl Job {
    pub fn create(name: String) -> Job {
        let id = rand::rng().random();
        println!("Job {} created, Job Id: {}", name, id);
        Job { name, id, retry_count: 0, status: JobStatus::Queued, started_at: None, finished_at: None, max_retry_count: MAX_RETRY_COUNT }
    }

    pub fn start(&mut self) {
        if(self.status == JobStatus::Queued) {
            self.status = JobStatus::Running;
            self.started_at = Some(std::time::SystemTime::now());
            self.print_status();
        } else {
            println!("Cannot run job which is not queued");
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn retry_count(&self) -> u32 { self.retry_count }

    pub fn max_retry_count(&self) -> u32 { self.max_retry_count }

    pub fn started_at(&self) -> Option<SystemTime> { self.started_at }

    pub fn finished_at(&self) -> Option<SystemTime> { self.finished_at }

    pub fn is_queued(&self) -> bool {
        self.status == JobStatus::Queued
    }

    pub fn is_running(&self) -> bool { self.status == JobStatus::Running }

    pub fn is_done(&self) -> bool { self.status == JobStatus::Done }

    pub fn is_failed(&self) -> bool { self.status == JobStatus::Failed }

    pub fn status(&self) -> &JobStatus { &self.status}

    pub fn check_is_max_retry_count(&self) -> bool {
        self.retry_count >= self.max_retry_count
    }

    fn retry(&mut self) {
        self.status = JobStatus::Queued;
        println!("Job {} retried, Job Id: {}, Retry Count: {}", self.name, self.id, self.retry_count);
    }

    pub fn fail(&mut self) {
        if(self.status == JobStatus::Running) {
            self.status = JobStatus::Failed;
            self.retry_count += 1;

            if self.check_is_max_retry_count() {
                self.finished_at = Some(std::time::SystemTime::now());
                self.print_status();
            } else {
                self.retry();
            }

        } else {
            println!("Cannot fail job which is not running");
        }
    }

    pub fn finish (&mut self) {
        if self.status == JobStatus::Running {
            self.status = JobStatus::Done;
            self.finished_at = Some(std::time::SystemTime::now());
            self.print_status();
        } else {
            println!("Cannot finish job which is not running");
        }
    }

    pub fn print_status(&self) {
        println!("Job Id: {}, Job Name: {}, Job Status: {}, Retry Count: {}, Started At: {}, Finished At: {}",
                 self.id, self.name, self.status, self.retry_count, format_time_to_string(self.started_at), format_time_to_string(self.finished_at));
    }
}

#[cfg(test)]
mod tests {
    use crate::job::{Job};

    // creating job
    #[test]
    fn test_creating_job() {
        let job = Job::create("Job".to_string());

        assert_eq!(job.retry_count(), 0);
        assert!(job.is_queued());
        assert_eq!(job.started_at(), None);
        assert_eq!(job.finished_at(), None);
    }

    // starting job
    #[test]
    fn test_starting_job() {
        let mut job = Job::create("Job".to_string());
        assert!(job.is_queued());
        job.start();
        assert!(!job.is_queued());
        assert!(job.is_running());
        assert!(matches!(job.started_at, Some(_)));
    }

    // finishing job
    #[test]
    fn test_finishing_job() {
        let mut job = Job::create("Job".to_string());
        job.start();
        job.finish();
        assert!(job.is_done());
        assert!(matches!(job.finished_at, Some(_)));
    }

    // fail does retry
    #[test]
    fn test_fail_retry() {
        let mut job = Job::create("Job".to_string());
        job.start();
        job.fail();
        assert!(job.is_queued());
        assert_eq!(job.retry_count(), 1);
    }

    // fails job after max retry limit
    #[test]
    fn test_fail_totally() {
        let mut job = Job::create("Job".to_string());
        job.start();
        job.fail();
        job.start();
        job.fail();
        job.start();
        job.fail();
        assert!(job.is_failed());
        assert!(matches!(job.finished_at, Some(_)));
    }
}