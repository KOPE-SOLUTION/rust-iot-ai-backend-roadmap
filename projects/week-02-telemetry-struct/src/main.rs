#[derive(Debug)]
struct Telemetry {
    device_id: String,
    sensor_type: String,
    value: f64,
    unit: String,
    timestamp: String,
}

impl Telemetry {
    fn new(
        device_id: &str,
        sensor_type: &str,
        value: f64,
        unit: &str,
        timestamp: &str,
    ) -> Self {
        Self {
            device_id: device_id.to_string(),
            sensor_type: sensor_type.to_string(),
            value,
            unit: unit.to_string(),
            timestamp: timestamp.to_string(),
        }
    }

    fn is_above(&self, threshold: f64) -> bool {
        self.value > threshold
    }

    fn print_summary(&self) {
        println!("=== Telemetry Summary ===");
        println!("Device ID   : {}", self.device_id);
        println!("Sensor Type : {}", self.sensor_type);
        println!("Value       : {:.2}", self.value);
        println!("Unit        : {}", self.unit);
        println!("Timestamp   : {}", self.timestamp);
    }
}

fn main() {
    let telemetry = Telemetry::new(
        "node-01",
        "temperature",
        27.5,
        "C",
        "2026-03-24T10:30:00Z",
    );

    println!("{:#?}", telemetry);
    println!();

    telemetry.print_summary();

    println!();
    println!("Above 25.0? {}", telemetry.is_above(25.0));
    println!("Above 30.0? {}", telemetry.is_above(30.0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_telemetry() {
        let telemetry = Telemetry::new(
            "node-01",
            "temperature",
            27.5,
            "C",
            "2026-03-24T10:30:00Z",
        );

        assert_eq!(telemetry.device_id, "node-01");
        assert_eq!(telemetry.sensor_type, "temperature");
        assert_eq!(telemetry.value, 27.5);
        assert_eq!(telemetry.unit, "C");
        assert_eq!(telemetry.timestamp, "2026-03-24T10:30:00Z");
    }

    #[test]
    fn test_is_above_true() {
        let telemetry = Telemetry::new(
            "node-01",
            "temperature",
            27.5,
            "C",
            "2026-03-24T10:30:00Z",
        );

        assert!(telemetry.is_above(25.0));
    }

    #[test]
    fn test_is_above_false() {
        let telemetry = Telemetry::new(
            "node-01",
            "temperature",
            27.5,
            "C",
            "2026-03-24T10:30:00Z",
        );

        assert!(!telemetry.is_above(30.0));
    }
}