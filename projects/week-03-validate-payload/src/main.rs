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
            println!("=== Payload Validation ===");
            println!("Telemetry: {:#?}", telemetry);

            match validate_telemetry(&telemetry) {
                Ok(()) => println!("Status   : VALID"),
                Err(err) => println!("Status   : INVALID\nReason   : {}", err),
            }
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

fn validate_telemetry(t: &Telemetry) -> Result<(), String> {
    if t.device_id.trim().is_empty() {
        return Err("device_id cannot be empty".to_string());
    }

    if t.sensor.trim().is_empty() {
        return Err("sensor cannot be empty".to_string());
    }

    if t.unit.trim().is_empty() {
        return Err("unit cannot be empty".to_string());
    }

    if t.timestamp.trim().is_empty() {
        return Err("timestamp cannot be empty".to_string());
    }

    match t.sensor.as_str() {
        "temp" => {
            if t.value < -40.0 || t.value > 125.0 {
                return Err(format!("temp value {} is out of valid range", t.value));
            }
        }
        "humidity" => {
            if t.value < 0.0 || t.value > 100.0 {
                return Err(format!("humidity value {} is out of valid range", t.value));
            }
        }
        "ph" => {
            if t.value < 0.0 || t.value > 14.0 {
                return Err(format!("ph value {} is out of valid range", t.value));
            }
        }
        "ec" => {
            if t.value < 0.0 || t.value > 20.0 {
                return Err(format!("ec value {} is out of valid range", t.value));
            }
        }
        _ => {
            return Err(format!("unsupported sensor type '{}'", t.sensor));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_telemetry(sensor: &str, value: f64) -> Telemetry {
        Telemetry {
            device_id: "node-01".to_string(),
            sensor: sensor.to_string(),
            value,
            unit: "unit".to_string(),
            timestamp: "2026-03-24T14:00:00Z".to_string(),
        }
    }

    #[test]
    fn test_valid_temp() {
        let t = Telemetry {
            device_id: "node-01".to_string(),
            sensor: "temp".to_string(),
            value: 27.5,
            unit: "C".to_string(),
            timestamp: "2026-03-24T14:00:00Z".to_string(),
        };

        assert!(validate_telemetry(&t).is_ok());
    }

    #[test]
    fn test_invalid_temp_range() {
        let t = make_telemetry("temp", 200.0);
        assert!(validate_telemetry(&t).is_err());
    }

    #[test]
    fn test_valid_humidity() {
        let t = Telemetry {
            device_id: "node-01".to_string(),
            sensor: "humidity".to_string(),
            value: 65.0,
            unit: "%".to_string(),
            timestamp: "2026-03-24T14:00:00Z".to_string(),
        };

        assert!(validate_telemetry(&t).is_ok());
    }

    #[test]
    fn test_invalid_humidity_range() {
        let t = make_telemetry("humidity", 120.0);
        assert!(validate_telemetry(&t).is_err());
    }

    #[test]
    fn test_invalid_empty_device_id() {
        let t = Telemetry {
            device_id: "".to_string(),
            sensor: "temp".to_string(),
            value: 25.0,
            unit: "C".to_string(),
            timestamp: "2026-03-24T14:00:00Z".to_string(),
        };

        assert!(validate_telemetry(&t).is_err());
    }

    #[test]
    fn test_invalid_sensor_type() {
        let t = make_telemetry("co2", 500.0);
        assert!(validate_telemetry(&t).is_err());
    }
}