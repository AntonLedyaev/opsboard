use crate::job::{Job};

pub struct Queue {
    jobs: Vec<Job>,
}

pub fn find_queued_job(jobs: &[Job]) -> Option<&Job> {
    jobs.iter().find(|job| job.is_queued())
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

    pub fn find_next_queued_job(&self) -> Option<&Job> {
        let found_job = find_queued_job(&self.jobs);

        match found_job {
            Some(job) => {
                job.print_status();
            }
            None => {
                println!("No job found");
            }
        }

        found_job
    }

    pub fn list_jobs(&self) {
        for job in self.jobs.iter() {
            job.print_status();
        }
    }

    pub fn start_job(&mut self, name: &str) {
        for job in self.jobs.iter_mut() {
            if (job.get_name() == name) {
                job.start();
            }
        }
    }

    pub fn fail(&mut self, name: &str) {
        for job in self.jobs.iter_mut() {
            if (job.get_name() == name) {
                job.fail();
            }
        }
    }
    
    pub fn finish_job(&mut self, job_name: &str) {
        for job in self.jobs.iter_mut() {
            if (job.get_name() == job_name) {
                job.finish();
            }
        }
    }
}