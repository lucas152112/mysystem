pub mod functions;
pub mod status;
pub mod websocket;

use axum::{
    Router,
    routing::{get, post, put, delete},
};
use crate::database::DatabasePool;

pub fn routes() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .nest("/functions", functions::routes())
        .nest("/status", status::routes())
        .route("/ws", get(websocket::handler))
}

async fn health_check() -> &'static str {
    "OK"
}