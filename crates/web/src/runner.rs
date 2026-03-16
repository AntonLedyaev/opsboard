use crate::job::Job;
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

    fn get_next_queued_job(&mut self) -> Option<&mut Job> {
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
}