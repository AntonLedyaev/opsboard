use sqlx::SqlitePool;
use crate::db::{get_next_queued_job, update_job};
use crate::job::{Job};
use tokio::time::{sleep, Duration};
use rand::{RngExt};

pub struct Runner {}

fn check_is_failing_job(job: &Job) -> bool {
    job.name().contains("fail")
}

impl Runner {
    pub fn new() -> Runner<> {
        Runner {}
    }

    pub async fn run_next_job(pool: &SqlitePool) -> Result<bool, sqlx::Error> {
        let next_queued_job = get_next_queued_job(pool).await?;

        match next_queued_job {
            Some(job_row) => {
                let mut job = Job::from(job_row);

                job.start();
                update_job(pool, &job).await?;

                let secs = {
                    let mut rng = rand::rng();
                    rng.random_range(0..=5)
                };

                sleep(Duration::from_secs(secs)).await;

                if check_is_failing_job(&job) {
                    job.fail();
                } else {
                    job.finish();
                }

                update_job(pool, &job).await?;
                Ok(true)
            }
            None => Ok(false),
        }
    }

    pub async fn run_queue(&mut self, pool: &SqlitePool) -> Result<(), sqlx::Error> {
        while Runner::run_next_job(pool).await? {}
        Ok(())
    }
}
