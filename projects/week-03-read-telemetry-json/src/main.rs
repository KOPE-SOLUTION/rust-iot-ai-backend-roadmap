use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct Telemetry {
    device_id: String,
    sensor: String,
    value: f64,
    unit: String,
    timestamp: String,
}

fn main() {
    let file_path = "telemetry.json";

    match read_telemetry_from_file(file_path) {
        Ok(telemetry) => {
            println!("=== Telemetry JSON Reader ===");
            println!("Device ID : {}", telemetry.device_id);
            println!("Sensor    : {}", telemetry.sensor);
            println!("Value     : {:.2}", telemetry.value);
            println!("Unit      : {}", telemetry.unit);
            println!("Timestamp : {}", telemetry.timestamp);
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}

fn read_telemetry_from_file(path: &str) -> Result<Telemetry, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("failed to read file '{}': {}", path, e))?;

    let telemetry: Telemetry = serde_json::from_str(&content)
        .map_err(|e| format!("failed to parse JSON: {}", e))?;

    Ok(telemetry)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_json() {
        let json = r#"
        {
            "device_id": "node-01",
            "sensor": "temp",
            "value": 27.5,
            "unit": "C",
            "timestamp": "2026-03-24T13:00:00Z"
        }
        "#;

        let telemetry: Telemetry = serde_json::from_str(json).unwrap();

        assert_eq!(telemetry.device_id, "node-01");
        assert_eq!(telemetry.sensor, "temp");
        assert_eq!(telemetry.value, 27.5);
        assert_eq!(telemetry.unit, "C");
        assert_eq!(telemetry.timestamp, "2026-03-24T13:00:00Z");
    }

    #[test]
    fn test_parse_invalid_json() {
        let json = r#"
        {
            "device_id": "node-01",
            "sensor": "temp",
            "value": "bad_value",
            "unit": "C",
            "timestamp": "2026-03-24T13:00:00Z"
        }
        "#;

        let result: Result<Telemetry, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}