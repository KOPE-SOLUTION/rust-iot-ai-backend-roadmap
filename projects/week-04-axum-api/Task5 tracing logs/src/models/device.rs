use serde::{Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct Device {
    pub device_id: String,
    pub site_id: String,
    pub name: String,
    pub online: bool,
}