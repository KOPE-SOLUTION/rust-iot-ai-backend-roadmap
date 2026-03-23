#[derive(Debug)]
enum Severity {
    Low,
    Warning,
    Critical,
}

#[derive(Debug)]
enum EventStatus {
    Active,
    Resolved,
}

#[derive(Debug)]
struct AlertEvent {
    event_id: String,
    device_id: String,
    alert_type: String,
    severity: Severity,
    message: String,
    timestamp: String,
    status: EventStatus,
}

impl AlertEvent {
    fn new(
        event_id: &str,
        device_id: &str,
        alert_type: &str,
        severity: Severity,
        message: &str,
        timestamp: &str,
        status: EventStatus,
    ) -> Self {
        Self {
            event_id: event_id.to_string(),
            device_id: device_id.to_string(),
            alert_type: alert_type.to_string(),
            severity,
            message: message.to_string(),
            timestamp: timestamp.to_string(),
            status,
        }
    }

    fn is_critical(&self) -> bool {
        matches!(self.severity, Severity::Critical)
    }

    fn print_summary(&self) {
        println!("=== Alert Event Summary ===");
        println!("Event ID    : {}", self.event_id);
        println!("Device ID   : {}", self.device_id);
        println!("Alert Type  : {}", self.alert_type);
        println!("Severity    : {}", self.severity_string());
        println!("Message     : {}", self.message);
        println!("Timestamp   : {}", self.timestamp);
        println!("Status      : {}", self.status_string());
    }

    fn severity_string(&self) -> &str {
        match self.severity {
            Severity::Low => "LOW",
            Severity::Warning => "WARNING",
            Severity::Critical => "CRITICAL",
        }
    }

    fn status_string(&self) -> &str {
        match self.status {
            EventStatus::Active => "ACTIVE",
            EventStatus::Resolved => "RESOLVED",
        }
    }
}

fn main() {
    let alert = AlertEvent::new(
        "evt-001",
        "node-01",
        "temperature_high",
        Severity::Critical,
        "Temperature exceeded threshold",
        "2026-03-24T12:00:00Z",
        EventStatus::Active,
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
    fn test_severity_enum() {
        let severity = Severity::Critical;

        match severity {
            Severity::Critical => assert!(true),
            _ => panic!("Wrong severity"),
        }
    }

    #[test]
    fn test_status_enum() {
        let status = EventStatus::Resolved;

        match status {
            EventStatus::Resolved => assert!(true),
            _ => panic!("Wrong status"),
        }
    }

    #[test]
    fn test_is_critical() {
        let alert = AlertEvent::new(
            "evt-001",
            "node-01",
            "temperature_high",
            Severity::Critical,
            "msg",
            "time",
            EventStatus::Active,
        );

        assert!(alert.is_critical());
    }
}