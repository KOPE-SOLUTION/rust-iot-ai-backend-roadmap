use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize)]
struct Telemetry {
    device_id: String,
    sensor: String,
    value: f64,
    unit: String,
    timestamp: String,
}

#[derive(Debug, Serialize)]
struct ValidationReport {
    device_id: String,
    sensor: String,
    value: f64,
    status: String,
    message: String,
}

fn main() {
    let input_file = "telemetry.json";
    let output_file = "report.json";

    match read_telemetry_from_file(input_file) {
        Ok(telemetry) => {
            let report = build_report(&telemetry);

            match write_report_to_file(output_file, &report) {
                Ok(()) => {
                    println!("=== Report Writer ===");
                    println!("Input  : {}", input_file);
                    println!("Output : {}", output_file);
                    println!("Report written successfully.");
                    println!("{:#?}", report);
                }
                Err(err) => {
                    eprintln!("Failed to write report: {}", err);
                }
            }
        }
        Err(err) => {
            eprintln!("Failed to read telemetry: {}", err);
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

fn build_report(t: &Telemetry) -> ValidationReport {
    match validate_telemetry(t) {
        Ok(()) => ValidationReport {
            device_id: t.device_id.clone(),
            sensor: t.sensor.clone(),
            value: t.value,
            status: "VALID".to_string(),
            message: "Payload is valid".to_string(),
        },
        Err(err) => ValidationReport {
            device_id: t.device_id.clone(),
            sensor: t.sensor.clone(),
            value: t.value,
            status: "INVALID".to_string(),
            message: err,
        },
    }
}

fn write_report_to_file(path: &str, report: &ValidationReport) -> Result<(), String> {
    let json = serde_json::to_string_pretty(report)
        .map_err(|e| format!("failed to serialize report: {}", e))?;

    fs::write(path, json)
        .map_err(|e| format!("failed to write report file '{}': {}", path, e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_valid_temp() -> Telemetry {
        Telemetry {
            device_id: "node-01".to_string(),
            sensor: "temp".to_string(),
            value: 27.5,
            unit: "C".to_string(),
            timestamp: "2026-03-24T15:00:00Z".to_string(),
        }
    }

    #[test]
    fn test_build_valid_report() {
        let telemetry = make_valid_temp();
        let report = build_report(&telemetry);

        assert_eq!(report.status, "VALID");
        assert_eq!(report.message, "Payload is valid");
    }

    #[test]
    fn test_build_invalid_report() {
        let telemetry = Telemetry {
            device_id: "node-01".to_string(),
            sensor: "humidity".to_string(),
            value: 120.0,
            unit: "%".to_string(),
            timestamp: "2026-03-24T15:00:00Z".to_string(),
        };

        let report = build_report(&telemetry);

        assert_eq!(report.status, "INVALID");
        assert!(report.message.contains("out of valid range"));
    }

    #[test]
    fn test_serialize_report_json() {
        let telemetry = make_valid_temp();
        let report = build_report(&telemetry);

        let json = serde_json::to_string_pretty(&report).unwrap();
        assert!(json.contains("\"status\": \"VALID\""));
        assert!(json.contains("\"sensor\": \"temp\""));
    }
}