#[derive(Debug)]
pub struct Device {
    pub device_id: String,
    pub site_id: String,
    pub name: String,
    pub online: bool,
}

impl Device {
    pub fn new(device_id: &str, site_id: &str, name: &str, online: bool) -> Self {
        Self {
            device_id: device_id.to_string(),
            site_id: site_id.to_string(),
            name: name.to_string(),
            online,
        }
    }
}