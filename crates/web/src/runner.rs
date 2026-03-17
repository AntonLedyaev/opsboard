use crate::job::{Job, JobStatus};
use crate::queue::Queue;

pub struct Runner<'a> {
    queue: &'a mut Queue
}

fn check_is_failing_job(job: &Job) -> bool {
    job.name().contains("fail")
}

impl<'a> Runner<'a> {
    pub fn new(queue: &'_ mut Queue) -> Runner<'_> {
        Runner { queue }
    }

    pub fn get_next_queued_job(&mut self) -> Option<&mut Job> {
        self.queue.find_next_queued_job_mut()
    }

    pub fn run_next_job(&mut self) {
        let next_queued_job = self.get_next_queued_job();

        match next_queued_job {
            Some(job) => {
                job.start();
                // fail job destined to fail
                if (check_is_failing_job(job)) {
                    job.fail()
                } else {
                    job.finish()
                }
            }

            None => { }
        }
    }

    pub fn run_queue(&mut self) {
        while self.queue.is_any_job_queued() {
            self.run_next_job();
        }

        println!("Queue completed, final state is: ");
        self.queue.list_jobs();
    }

    pub fn get_job_statuses(&self) -> Vec<&JobStatus> {
        self.queue.get_job_statuses()
    }
}

#[cfg(test)]
mod tests {
    use crate::job::JobStatus;
    use crate::queue::Queue;
    use crate::runner::Runner;

    // empty queue
    #[test]
    fn test_run_empty_queue() {
        let mut queue = Queue::new(vec![]);
        let mut runner = Runner::new(&mut queue);
        runner.run_queue();
        assert!(matches!(runner.get_next_queued_job(), None));
    }

    // runs successful queue
    #[test]
    fn test_run_successful_queue() {
        let mut queue = Queue::new(vec!["Job1", "Job2", "Job3"]);
        let mut runner = Runner::new(&mut queue);
        runner.run_queue();

        let result = runner.get_job_statuses();

        for status in result {
            assert!(matches!(status, JobStatus::Done));
        }
    }

    // runs mixed queue
    #[test]
    fn test_run_mixed_queue() {
        let mut queue = Queue::new(vec!["Job1", "Job2_fail", "Job3"]);
        let mut runner = Runner::new(&mut queue);
        runner.run_queue();

        let result = runner.get_job_statuses();

        assert!(matches!(result[0], JobStatus::Done));
        assert!(matches!(result[1], JobStatus::Failed));
        assert!(matches!(result[2], JobStatus::Done));
    }
}

