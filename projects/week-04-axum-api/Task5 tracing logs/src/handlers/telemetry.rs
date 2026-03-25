use axum::Json;
use std::fs;
use tracing::info;

use crate::models::telemetry::Telemetry;

pub async fn latest_telemetry() -> Json<Vec<Telemetry>> {

    info!("Fetching latest telemetry...");

    let content =
        fs::read_to_string("telemetry_latest.json")
        .expect("failed to read telemetry_latest.json");

    let data: Vec<Telemetry> =
        serde_json::from_str(&content)
        .expect("failed to parse telemetry_latest.json");

    Json(data)
}