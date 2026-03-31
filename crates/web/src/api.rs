use axum::{
    routing::get,
    extract::State,
    http::StatusCode,
    Json,
    Router,
};
use axum::routing::post;
use crate::app_state::AppState;
use crate::job::{Job, JobStatus};
use serde::{Deserialize, Serialize};
use crate::runner::Runner;

#[derive(Serialize)]
struct JobDto {
    id: u32,
    name: String,
    status: JobStatus,
    retry_count: u32,
}

impl From<&Job> for JobDto {
    fn from(job: &Job) -> Self {
        JobDto {
            id: job.id(),
            name: job.name().to_string(),
            status: job.status().to_owned(),
            retry_count: job.retry_count(),
        }
    }
}

async fn health() -> StatusCode {
    StatusCode::OK
}

async fn get_jobs(State(state): State<AppState>) -> (StatusCode, Json<Vec<JobDto>>) {
    let jobs = state.queue.lock().unwrap();

    (StatusCode::OK, Json(jobs.get_jobs().iter().map(|job| JobDto::from(job)).collect()))
}

    #[derive(Deserialize)]
struct CreateJobRequest {
    name: String,
}

async fn create_job(State(state): State<AppState>, Json(payload): Json<CreateJobRequest>) -> (StatusCode, Json<Vec<JobDto>>) {
    let mut queue = state.queue.lock().unwrap();
    queue.add_job(&payload.name);

    let jobs = queue.get_jobs().iter().map(|job| JobDto::from(job)).collect();

    (StatusCode::CREATED, Json(jobs))
}

async fn run_queue(State(state): State<AppState>) -> (StatusCode, Json<Vec<JobStatus>>) {
    let mut queue = state.queue.lock().unwrap();
    let mut runner = Runner::new(&mut queue);
    runner.run_queue();

    (StatusCode::OK, Json(runner.get_job_statuses().to_owned()))
}

pub fn api_router() -> Router<AppState> {
    Router::new().route("/health", get(health)).route("/jobs", get(get_jobs).post(create_job)).route("/run", post(run_queue))
}