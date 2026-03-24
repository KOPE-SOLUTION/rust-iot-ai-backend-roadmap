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
    let input_file = "telemetry_batch.json";
    let output_file = "batch_report.json";

    match read_batch(input_file) {
        Ok(records) => {
            println!("Records loaded: {}", records.len());

            let reports: Vec<ValidationReport> =
                records.iter().map(build_report).collect();

            write_report(output_file, &reports).unwrap();

            println!("Report written to {}", output_file);
        }

        Err(err) => println!("Error: {}", err),
    }
}

fn read_batch(path: &str) -> Result<Vec<Telemetry>, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("failed to read file: {}", e))?;

    let records: Vec<Telemetry> =
        serde_json::from_str(&content)
        .map_err(|e| format!("JSON parse error: {}", e))?;

    Ok(records)
}

fn validate_telemetry(t: &Telemetry) -> Result<(), String> {

    if t.device_id.trim().is_empty() {
        return Err("device_id empty".to_string());
    }

    match t.sensor.as_str() {

        "temp" => {
            if t.value < -40.0 || t.value > 125.0 {
                return Err(format!("temp {} out of range", t.value));
            }
        }

        "humidity" => {
            if t.value < 0.0 || t.value > 100.0 {
                return Err(format!("humidity {} out of range", t.value));
            }
        }

        _ => {
            return Err(format!("unsupported sensor {}", t.sensor));
        }
    }

    Ok(())
}

fn build_report(t: &Telemetry) -> ValidationReport {

    match validate_telemetry(t) {

        Ok(_) => ValidationReport {
            device_id: t.device_id.clone(),
            sensor: t.sensor.clone(),
            value: t.value,
            status: "VALID".to_string(),
            message: "OK".to_string(),
        },

        Err(e) => ValidationReport {
            device_id: t.device_id.clone(),
            sensor: t.sensor.clone(),
            value: t.value,
            status: "INVALID".to_string(),
            message: e,
        },
    }
}

fn write_report(path: &str, report: &Vec<ValidationReport>) -> Result<(), String> {

    let json = serde_json::to_string_pretty(report)
        .map_err(|e| format!("serialize error: {}", e))?;

    fs::write(path, json)
        .map_err(|e| format!("write error: {}", e))?;

    Ok(())
}

#[cfg(test)]
mod tests {

use super::*;

#[test]
fn test_batch_deserialize() {

let json = r#"
[
{
"device_id":"node-01",
"sensor":"temp",
"value":25.0,
"unit":"C",
"timestamp":"2026"
}
]
"#;

let data: Vec<Telemetry> = serde_json::from_str(json).unwrap();

assert_eq!(data.len(),1);
assert_eq!(data[0].sensor,"temp");

}

}