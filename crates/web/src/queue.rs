use crate::job::{Job, JobStatus};

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

    pub fn jobs_left(&self) -> u32 {
        self.jobs.len() as u32
    }

    pub fn list_jobs(&self) {
        for job in self.jobs.iter() {
            job.print_status();
        }
    }
    
    pub fn get_job_statuses(&self) -> Vec<&JobStatus> {
        self.jobs.iter().map(|job| job.status( )).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::queue::Queue;

    // empty queue
    #[test]
    fn test_empty_queue() {
        let mut queue = Queue::new(vec![]);
        assert!(!queue.is_any_job_queued());
    }

    // adding jobs to queue
    #[test]
    fn test_adding_jobs_to_queue() {
        let mut queue = Queue::new(vec!["Job1", "Job2", "Job3"]);
        queue.add_job("Job4");
        assert_eq!(queue.jobs_left(), 4);
    }

    // checking if job is queued after adding
    #[test]
    fn test_checking_next_queued_job() {
        let mut queue = Queue::new(vec![]);
        queue.add_job("Job1");
        assert!(queue.is_any_job_queued());
    }
}