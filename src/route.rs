// /src/route.rs

use axum::{routing::get, Router};

use crate::handlers::health;
use crate::ws::ws_handler;
use crate::AppState;

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/ws", get(ws_handler))
        .with_state(state)
}
