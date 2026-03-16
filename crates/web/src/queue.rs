use crate::job::{Job};

pub struct Queue {
    jobs: Vec<Job>,
}

impl Queue {
    pub fn new(job_names: Vec<&str>) -> Queue {
        let mut queue = Queue { jobs: Vec::new() };
        for job_name in job_names {
            queue.jobs.push(Job::create(job_name.to_owned()));
        }

        queue
    }

    pub fn add_job(&mut self, job_name: &str) {
        self.jobs.push(Job::create(job_name.to_owned()));
    }

    pub fn is_any_job_queued (&self) -> bool {
        self.jobs.iter().any(|job| job.is_queued())
    }

    pub fn find_next_queued_job_mut(&mut self) -> Option<&mut Job> {
        self.jobs.iter_mut().find(|job| job.is_queued())
    }

    pub fn list_jobs(&self) {
        for job in self.jobs.iter() {
            job.print_status();
        }
    }
}