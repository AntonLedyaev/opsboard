mod job;
mod queue;
mod runner;

use crate::queue::Queue;
use crate::runner::Runner;

fn main() {
    let mut queue = Queue::new(vec!["Job1", "Job2", "Job3"]);
    queue.add_job("Job4_fail");
    queue.list_jobs();

    let mut runner = Runner::new(&mut queue);
    runner.run_queue();
}
