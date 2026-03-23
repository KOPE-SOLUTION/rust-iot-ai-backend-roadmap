#[derive(Debug)]
struct AlertEvent {
    event_id: String,
    device_id: String,
    alert_type: String,
    severity: String,
    message: String,
    timestamp: String,
    active: bool,
}

impl AlertEvent {
    fn new(
        event_id: &str,
        device_id: &str,
        alert_type: &str,
        severity: &str,
        message: &str,
        timestamp: &str,
        active: bool,
    ) -> Self {
        Self {
            event_id: event_id.to_string(),
            device_id: device_id.to_string(),
            alert_type: alert_type.to_string(),
            severity: severity.to_string(),
            message: message.to_string(),
            timestamp: timestamp.to_string(),
            active,
        }
    }

    fn status(&self) -> &str {
        if self.active {
            "ACTIVE"
        } else {
            "RESOLVED"
        }
    }

    fn is_critical(&self) -> bool {
        self.severity.eq_ignore_ascii_case("critical")
    }

    fn print_summary(&self) {
        println!("=== Alert Event Summary ===");
        println!("Event ID    : {}", self.event_id);
        println!("Device ID   : {}", self.device_id);
        println!("Alert Type  : {}", self.alert_type);
        println!("Severity    : {}", self.severity);
        println!("Message     : {}", self.message);
        println!("Timestamp   : {}", self.timestamp);
        println!("Status      : {}", self.status());
    }
}

fn main() {
    let alert = AlertEvent::new(
        "evt-001",
        "node-01",
        "temperature_high",
        "critical",
        "Temperature exceeded threshold",
        "2026-03-24T11:00:00Z",
        true,
    );

    println!("{:#?}", alert);
    println!();

    alert.print_summary();

    println!();
    println!("Is critical? {}", alert.is_critical());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_alert_event() {
        let alert = AlertEvent::new(
            "evt-001",
            "node-01",
            "temperature_high",
            "critical",
            "Temperature exceeded threshold",
            "2026-03-24T11:00:00Z",
            true,
        );

        assert_eq!(alert.event_id, "evt-001");
        assert_eq!(alert.device_id, "node-01");
        assert_eq!(alert.alert_type, "temperature_high");
        assert_eq!(alert.severity, "critical");
        assert_eq!(alert.message, "Temperature exceeded threshold");
        assert_eq!(alert.timestamp, "2026-03-24T11:00:00Z");
        assert_eq!(alert.active, true);
    }

    #[test]
    fn test_status_active() {
        let alert = AlertEvent::new(
            "evt-002",
            "node-02",
            "water_low",
            "warning",
            "Water level is below threshold",
            "2026-03-24T11:05:00Z",
            true,
        );

        assert_eq!(alert.status(), "ACTIVE");
    }

    #[test]
    fn test_status_resolved() {
        let alert = AlertEvent::new(
            "evt-003",
            "node-03",
            "sensor_disconnect",
            "warning",
            "Sensor disconnected",
            "2026-03-24T11:10:00Z",
            false,
        );

        assert_eq!(alert.status(), "RESOLVED");
    }

    #[test]
    fn test_is_critical_true() {
        let alert = AlertEvent::new(
            "evt-004",
            "node-04",
            "intrusion_detected",
            "critical",
            "Fence intrusion detected",
            "2026-03-24T11:15:00Z",
            true,
        );

        assert!(alert.is_critical());
    }

    #[test]
    fn test_is_critical_false() {
        let alert = AlertEvent::new(
            "evt-005",
            "node-05",
            "humidity_high",
            "warning",
            "Humidity above normal range",
            "2026-03-24T11:20:00Z",
            true,
        );

        assert!(!alert.is_critical());
    }
}