mod job;
mod runner;
mod api;
mod db;
mod app_state;

use crate::app_state::AppState;
use crate::db::{init_db};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = init_db().await?;
    let state = AppState::new(pool);
    let app = api::api_router().with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await?;

    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
