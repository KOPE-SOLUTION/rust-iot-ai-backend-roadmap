use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Telemetry {
    pub device_id: String,
    pub sensor: String,
    pub value: f64,
    pub unit: String,
    pub timestamp: String,
}