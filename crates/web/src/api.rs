use std::str::FromStr;
use axum::{
    routing::get,
    http::StatusCode,
    Json,
    Router,
};
use axum::extract::State;
use axum::routing::post;
use crate::job::{Job, JobStatus};
use serde::{Deserialize, Serialize};
use crate::app_state::AppState;
use crate::db::{get_jobs_from_db, insert_job, JobRow};
use crate::runner::Runner;

#[derive(Serialize)]
pub struct JobDto {
    id: u32,
    name: String,
    status: JobStatus,
    retry_count: u32,
}

impl From<Job> for JobDto {
    fn from(job: Job) -> Self {
        JobDto {
            id: job.id(),
            name: job.name().to_string(),
            status: job.status().to_owned(),
            retry_count: job.retry_count(),
        }
    }
}

impl From<JobRow> for JobDto {
    fn from(job_row: JobRow) -> Self {
        JobDto {
            id: job_row.id,
            name: job_row.name,
            status: JobStatus::from_str(&job_row.status.to_string()).unwrap(),
            retry_count: job_row.retry_count,
        }
    }
}

async fn health() -> StatusCode {
    StatusCode::OK
}

async fn get_jobs(State(state): State<AppState>) -> Result<(StatusCode, Json<Vec<JobDto>>), StatusCode> {
    let pool = state.pool;

    let jobs = get_jobs_from_db(&pool).await.map_err(|e| {
        eprintln!("get_jobs_from_db error: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let jobs_dto: Vec<JobDto> = jobs.into_iter().map(JobDto::from).collect();

    Ok((StatusCode::OK, Json(jobs_dto)))
}

    #[derive(Deserialize)]
struct CreateJobRequest {
    name: String,
}

async fn create_job(State(state): State<AppState>,Json(payload): Json<CreateJobRequest>) -> Result<StatusCode, StatusCode> {
    let pool = state.pool;

    insert_job(&pool, payload.name).await.map_err(|e| {
        eprintln!("create_job error: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(StatusCode::CREATED)
}

async fn run_queue(State(state): State<AppState>) -> Result<StatusCode, StatusCode> {
    let pool = state.pool;

    let mut runner = Runner::new();

    runner.run_queue(&pool).await.map_err(|e| {
        eprintln!("run_queue error: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(StatusCode::OK)
}

pub fn api_router() -> Router<AppState> {
    Router::new().route("/health", get(health)).route("/jobs", get(get_jobs).post(create_job)).route("/run", post(run_queue))
}