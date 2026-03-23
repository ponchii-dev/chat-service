// /src/handlers.rs

use axum::{Json, response::IntoResponse};

pub async fn health() -> impl IntoResponse {
    Json(serde_json::json!({"status":"ok"}))
}
