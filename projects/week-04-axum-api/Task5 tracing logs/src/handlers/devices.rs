use axum::Json;
use std::fs;
use tracing::info;

use crate::models::device::Device;

pub async fn get_devices() -> Json<Vec<Device>> {
    info!("Fetching devices...");
    
    let content =
        fs::read_to_string("devices.json")
        .expect("failed to read devices.json");

    let devices: Vec<Device> =
        serde_json::from_str(&content)
        .expect("failed to parse devices.json");

    Json(devices)
}