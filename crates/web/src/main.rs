mod job;
mod queue;

use job::{Job};
use crate::queue::Queue;

fn main() {
    let first_job = Job::create("Job1");
    let second_job = Job::create("Job2");
    let third_job = Job::create("Job3");

    let mut queue = Queue::new(vec![first_job, second_job, third_job]);

    let fourth_job = Job::create("Job4");

    queue.add_job(fourth_job);
    queue.list_jobs();
    queue.start_job("Job1");
    queue.start_job("Job2");
    queue.start_job("Job3");
    queue.start_job("Job4");
    queue.get_next_job();
    queue.retry_job("Job1");
    queue.retry_job("Job1");
    queue.retry_job("Job1");
    queue.retry_job("Job1");
    queue.list_jobs();
}
