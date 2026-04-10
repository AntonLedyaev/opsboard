use std::str::FromStr;
use axum::{
    routing::get,
    http::StatusCode,
    Json,
    Router,
};
use axum::extract::State;
use axum::http::{HeaderValue, Method};
use axum::routing::post;
use crate::job::{Job, JobStatus};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};
use crate::app_state::AppState;
use crate::db::{delete_job_from_db, get_jobs_from_db, insert_job, JobRow};
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

async fn health() -> Result<(StatusCode, Json<String>), StatusCode> {
    Ok((StatusCode::OK, Json(String::from("Healthcheck OK"))))
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

#[derive(Deserialize)]
pub struct DeleteJobRequest {
    id: u32
} 

async fn delete_job(State(state): State<AppState>, Json(payload): Json<DeleteJobRequest>) -> Result<StatusCode, StatusCode> {
    let pool = state.pool;

    delete_job_from_db(&pool, payload.id).await.map_err(|e| {
        eprintln!("delete_job error: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(StatusCode::OK)
}

pub fn api_router() -> Router<AppState> {
    let cors = CorsLayer::new()
        .allow_origin("http://127.0.0.1:8080".parse::<HeaderValue>().unwrap())
        .allow_origin("http://localhost.:8080".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers(Any);

    Router::new()
        .route("/health", get(health))
        .route("/jobs", get(get_jobs).post(create_job))
        .route("/run", post(run_queue))
        .route("/delete", post(delete_job))
        .layer(cors)
}