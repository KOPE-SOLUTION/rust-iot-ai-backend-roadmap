use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Telemetry {
    pub device_id: String,
    pub sensor: String,
    pub value: f64,
    pub unit: String,
    pub timestamp: String,
}