use axum::{
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::fs;
use tokio::net::TcpListener;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Device {
    device_id: String,
    site_id: String,
    name: String,
    online: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Telemetry {
    device_id: String,
    sensor: String,
    value: f64,
    unit: String,
    timestamp: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health))
        .route("/devices", get(get_devices))
        .route("/telemetry/latest", get(get_latest_telemetry));

    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
    })
}

async fn get_devices() -> Json<Vec<Device>> {
    let content = fs::read_to_string("devices.json")
        .expect("failed to read devices.json");

    let devices: Vec<Device> = serde_json::from_str(&content)
        .expect("failed to parse devices.json");

    Json(devices)
}

async fn get_latest_telemetry() -> Json<Vec<Telemetry>> {
    let content = fs::read_to_string("telemetry_latest.json")
        .expect("failed to read telemetry_latest.json");

    let telemetry: Vec<Telemetry> = serde_json::from_str(&content)
        .expect("failed to parse telemetry_latest.json");

    Json(telemetry)
}