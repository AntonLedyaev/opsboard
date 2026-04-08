use gloo_net::http::Request;

const API_BASE: &str = "http://127.0.0.1:3000";

pub async fn fetch_health() -> Result<String, gloo_net::Error> {
    Request::get(&format!("{API_BASE}/health"))
        .send()
        .await?
        .text()
        .await
}

pub async fn fetch_jobs() -> Result<String, gloo_net::Error> {
    Request::get(&format!("{API_BASE}/jobs"))
        .send()
        .await?
        .text()
        .await
}