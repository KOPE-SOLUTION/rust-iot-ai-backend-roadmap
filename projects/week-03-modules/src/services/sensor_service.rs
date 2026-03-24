use crate::models::telemetry::Telemetry;

pub fn evaluate_telemetry(t: &Telemetry) -> &'static str {
    match t.sensor.as_str() {
        "temp" => {
            if t.value > 30.0 {
                "HIGH"
            } else if t.value < 18.0 {
                "LOW"
            } else {
                "OK"
            }
        }
        "ph" => {
            if t.value > 7.5 {
                "HIGH"
            } else if t.value < 5.5 {
                "LOW"
            } else {
                "OK"
            }
        }
        _ => "UNKNOWN",
    }
}