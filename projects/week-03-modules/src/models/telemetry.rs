#[derive(Debug)]
pub struct Telemetry {
    pub device_id: String,
    pub sensor: String,
    pub value: f64,
}

impl Telemetry {
    pub fn new(device_id: &str, sensor: &str, value: f64) -> Self {
        Self {
            device_id: device_id.to_string(),
            sensor: sensor.to_string(),
            value,
        }
    }
}