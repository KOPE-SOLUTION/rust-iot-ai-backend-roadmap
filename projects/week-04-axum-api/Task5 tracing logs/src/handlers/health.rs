use axum::Json;
use serde::Serialize;
use tracing::info;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
}

pub async fn health() -> Json<HealthResponse> {
    info!("Checking health...");

    Json(HealthResponse {
        status: "ok".to_string(),
    })
}