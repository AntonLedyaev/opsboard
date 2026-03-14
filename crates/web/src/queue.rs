use crate::job::{print_job_name, Job, JobStatus};

pub struct Queue {
    jobs: Vec<Job>,
}

pub fn find_queued_job(jobs: &Vec<Job>) -> Option<&Job> {
    jobs.iter().find(|job| job.status == JobStatus::Queued)
}

impl Queue {
    pub fn new(jobs: Vec<Job>) -> Queue {
        let mut queue = Queue { jobs };
        for job in queue.jobs.iter_mut() {
            job.status = JobStatus::Queued;
        }
        queue
    }

    pub fn add_job(&mut self, mut job: Job) {
        job.status = JobStatus::Queued;
        self.jobs.push(job);
    }

    pub fn get_next_job(&mut self) {
        let found_job = find_queued_job(&self.jobs);

        match found_job {
            Some(job) => {
                print_job_name(job);
            }
            None => {
                println!("No job found");
            }
        }
    }

    pub fn list_jobs(&self) {
        for job in self.jobs.iter() {
            print_job_name(job);
        }
    }

    pub fn start_job(&mut self, name: &str) {
        for job in self.jobs.iter_mut() {
            if (job.name == name) {
                job.start();
            }
        }
    }

    pub fn retry_job(&mut self, name: &str) {
        for job in self.jobs.iter_mut() {
            if (job.name == name) {
                job.retry();
            }
        }
    }
}