use serde::{Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct Telemetry {
    pub device_id: String,
    pub sensor: String,
    pub value: f64,
    pub unit: String,
    pub timestamp: String,
}