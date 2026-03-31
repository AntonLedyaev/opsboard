mod job;
mod queue;
mod runner;
mod api;
mod app_state;

use crate::app_state::AppState;
use crate::queue::Queue;

#[tokio::main]
async fn main() {
    let queue = Queue::new(vec!["Job1", "Job2"]);
    let state = AppState::new(queue);

    let app = api::api_router().with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
