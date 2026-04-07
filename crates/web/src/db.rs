use sqlx::{Sqlite, migrate::MigrateDatabase, SqlitePool, FromRow};
use crate::job::Job;
use std::time::{SystemTime, UNIX_EPOCH};

pub const DB_URL: &str = "sqlite://jobs.db";

#[derive(FromRow)]
pub struct JobRow {
    pub id: u32,
    pub name: String,
    pub status: String,
    pub retry_count: u32,
    pub max_retry_count: u32,
    pub started_at: Option<u64>,
    pub finished_at: Option<u64>,
}

pub async fn init_db() -> Result<SqlitePool, sqlx::Error> {

    if !Sqlite::database_exists(DB_URL).await? {
        Sqlite::create_database(DB_URL).await?;
        println!("Database created");
    }

    let pool = SqlitePool::connect(DB_URL).await?;
    create_db(&pool).await?;

    Ok((pool))
}

pub async fn create_db(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query("CREATE TABLE IF NOT EXISTS jobs (
        id INTEGER PRIMARY KEY,
        name TEXT,
        status TEXT,
        retry_count INTEGER DEFAULT 0,
        max_retry_count INTEGER DEFAULT 3,
        started_at INTEGER DEFAULT null,
        finished_at INTEGER DEFAULT null
    )").execute(pool).await?;

    println!("Database created");

    Ok(())
}

fn to_timestamp(opt: Option<SystemTime>) -> Option<i64> {
    opt.map(|t| {
        t.duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    })
}


pub async fn insert_job(pool: &SqlitePool, job_name: String) -> Result<(), sqlx::Error> {
    let job = Job::create(job_name);

    let result = sqlx::query("INSERT INTO jobs (
                  id,
                  name,
                  status,
                  retry_count,
                  max_retry_count,
                  started_at,
                  finished_at
                  ) VALUES(?, ?, ?, ?, ?, ?, ?) ")
        .bind(job.id())
        .bind(job.name())
        .bind(job.status().to_string())
        .bind(job.retry_count())
        .bind(job.max_retry_count())
        .bind(to_timestamp(job.started_at()))
        .bind(to_timestamp(job.finished_at())).execute(pool).await?;

    println!("rows_affected = {}", result.rows_affected());

    Ok(())
}

pub async fn get_jobs_from_db(pool: &SqlitePool) -> Result<Vec<JobRow>, sqlx::Error> {
    let rows: Vec<JobRow> = sqlx::query_as("SELECT * FROM jobs").fetch_all(pool).await?;

    Ok(rows)
}

pub async fn get_queued_jobs_from_db(pool: &SqlitePool) -> Result<Vec<JobRow>, sqlx::Error> {
    let rows: Vec<JobRow> = sqlx::query_as("SELECT * FROM jobs WHERE status = 'Queued'").fetch_all(pool).await?;

    Ok(rows)
}

pub async fn get_next_queued_job(pool: &SqlitePool) -> Result<Option<JobRow>, sqlx::Error> {
    sqlx::query_as("SELECT * FROM jobs WHERE status = 'Queued' ORDER BY id LIMIT 1")
        .fetch_optional(pool)
        .await
}
pub async fn update_job(pool: &SqlitePool, runned_job: &Job) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE jobs SET retry_count = ?, status = ?, started_at = ?, finished_at = ? WHERE id = ?")
        .bind(runned_job.retry_count())
        .bind(runned_job.status().to_string())
        .bind(to_timestamp(runned_job.started_at()))
        .bind(to_timestamp(runned_job.finished_at()))
        .bind(runned_job.id()).execute(pool).await?;

    Ok(())
}