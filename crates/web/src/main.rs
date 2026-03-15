mod job;
mod queue;

use crate::queue::Queue;

fn main() {
    let mut queue = Queue::new(vec!["Job1", "Job2", "Job3"]);
    queue.add_job("Job4");
    queue.list_jobs();

    // success scenario
    queue.start_job("Job1");
    queue.finish_job("Job1");
    queue.list_jobs();

    // fail + requeue scenario
    queue.start_job("Job2");
    queue.fail("Job2");
    queue.start_job("Job2");
    queue.finish_job("Job2");
    queue.list_jobs();

    // final fail scenario;

    queue.start_job("Job3");
    queue.fail("Job3");
    queue.start_job("Job3");
    queue.fail("Job3");
    queue.start_job("Job3");
    queue.fail("Job3");
    queue.start_job("Job3");
    queue.list_jobs();

    // empty queue scenario
    queue.start_job("Job4");
    queue.finish_job("Job4");
    queue.find_next_queued_job();
    queue.list_jobs();
}
