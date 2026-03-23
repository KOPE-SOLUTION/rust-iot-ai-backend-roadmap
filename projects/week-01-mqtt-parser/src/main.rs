use std::env;
use std::process;

#[derive(Debug)]
struct MqttTopic {
    site_id: String,
    device_id: String,
    category: String,
    sensor: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        print_usage(&args[0]);
        process::exit(1);
    }

    let topic = &args[1];

    match parse_topic(topic) {
        Ok(parsed) => {
            println!("=== MQTT Topic Parser ===");
            println!("Topic    : {}", topic);
            println!("Site ID  : {}", parsed.site_id);
            println!("Device ID: {}", parsed.device_id);
            println!("Category : {}", parsed.category);
            println!("Sensor   : {}", parsed.sensor);
            println!("Status   : VALID");
        }
        Err(err) => {
            eprintln!("=== MQTT Topic Parser ===");
            eprintln!("Topic  : {}", topic);
            eprintln!("Status : INVALID");
            eprintln!("Error  : {}", err);
            process::exit(1);
        }
    }
}

fn parse_topic(topic: &str) -> Result<MqttTopic, String> {
    let parts: Vec<&str> = topic.split('/').collect();

    if parts.len() != 6 {
        return Err(format!(
            "expected 6 topic parts, but got {}",
            parts.len()
        ));
    }

    if parts[0] != "site" {
        return Err("topic must start with 'site'".to_string());
    }

    if parts[2] != "device" {
        return Err("third segment must be 'device'".to_string());
    }

    let site_id = parts[1].trim();
    let device_id = parts[3].trim();
    let category = parts[4].trim();
    let sensor = parts[5].trim();

    if site_id.is_empty() {
        return Err("site_id cannot be empty".to_string());
    }

    if device_id.is_empty() {
        return Err("device_id cannot be empty".to_string());
    }

    if category.is_empty() {
        return Err("category cannot be empty".to_string());
    }

    if sensor.is_empty() {
        return Err("sensor cannot be empty".to_string());
    }

    Ok(MqttTopic {
        site_id: site_id.to_string(),
        device_id: device_id.to_string(),
        category: category.to_string(),
        sensor: sensor.to_string(),
    })
}

fn print_usage(program_name: &str) {
    eprintln!("Usage:");
    eprintln!("  {} <mqtt_topic>", program_name);
    eprintln!();
    eprintln!("Examples:");
    eprintln!(
        "  {} site/farm-1/device/node-01/telemetry/temp",
        program_name
    );
    eprintln!(
        "  {} site/farm-2/device/node-22/telemetry/humidity",
        program_name
    );
    eprintln!(
        "  {} site/greenhouse-a/device/ec-meter-01/alert/high_ec",
        program_name
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_topic() {
        let topic = "site/farm-1/device/node-01/telemetry/temp";
        let parsed = parse_topic(topic).unwrap();

        assert_eq!(parsed.site_id, "farm-1");
        assert_eq!(parsed.device_id, "node-01");
        assert_eq!(parsed.category, "telemetry");
        assert_eq!(parsed.sensor, "temp");
    }

    #[test]
    fn test_valid_alert_topic() {
        let topic = "site/greenhouse-a/device/ec-meter-01/alert/high_ec";
        let parsed = parse_topic(topic).unwrap();

        assert_eq!(parsed.site_id, "greenhouse-a");
        assert_eq!(parsed.device_id, "ec-meter-01");
        assert_eq!(parsed.category, "alert");
        assert_eq!(parsed.sensor, "high_ec");
    }

    #[test]
    fn test_invalid_part_count() {
        let topic = "site/farm-1/device/node-01/temp";
        let result = parse_topic(topic);

        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_prefix() {
        let topic = "farm/farm-1/device/node-01/telemetry/temp";
        let result = parse_topic(topic);

        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_device_segment() {
        let topic = "site/farm-1/node/node-01/telemetry/temp";
        let result = parse_topic(topic);

        assert!(result.is_err());
    }
}