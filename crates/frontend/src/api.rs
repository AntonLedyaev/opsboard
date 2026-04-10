use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

const API_BASE: &str = "http://127.0.0.1:3000";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobView {
    pub id: u32,
    pub name: String,
    pub status: String,
    pub retry_count: u64,
}

pub async fn fetch_jobs() -> Result<Vec<JobView>, gloo_net::Error> {
    Request::get(&format!("{API_BASE}/jobs"))
        .send()
        .await?
        .json::<Vec<JobView>>()
        .await
}

#[derive(Serialize)]
pub struct CreateJobRequestPayload {
    pub name: String,
}
pub async fn fetch_jobs_post(payload: CreateJobRequestPayload) -> Result<String, gloo_net::Error> {
    Request::post(&format!("{API_BASE}/jobs"))
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&payload).unwrap())?.send().await?.text().await
}

pub async fn run_jobs() -> Result<String, gloo_net::Error> {
    Request::post(&format!("{API_BASE}/run")).send().await?.text().await
}

#[derive(Serialize)]
pub struct DeleteJobRequestPayload {
    pub id: u32,
}
pub async fn delete_job(payload: DeleteJobRequestPayload) -> Result<String, gloo_net::Error> {
    Request::post(&format!("{API_BASE}/delete"))
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&payload).unwrap())?.send().await?.text().await
}