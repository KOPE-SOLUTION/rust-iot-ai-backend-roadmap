use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    pub device_id: String,
    pub site_id: String,
    pub name: String,
    pub online: bool,
}