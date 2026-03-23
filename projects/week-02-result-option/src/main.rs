#[derive(Debug)]
struct SensorReading {
    sensor_name: String,
    value: f64,
    unit: String,
}

fn main() {
    println!("=== Result and Option Demo ===");

    // -------------------------
    // Option example
    // -------------------------
    let sensor_name = "temp";
    match find_sensor_unit(sensor_name) {
        Some(unit) => println!("Sensor '{}' uses unit '{}'", sensor_name, unit),
        None => println!("Sensor '{}' not found", sensor_name),
    }

    let unknown_sensor = "co2";
    let unit = find_sensor_unit(unknown_sensor).unwrap_or("UNKNOWN");
    println!("Sensor '{}' uses unit '{}'", unknown_sensor, unit);

    // -------------------------
    // Result example
    // -------------------------
    let input_value = "27.5";
    match parse_sensor_value(input_value) {
        Ok(value) => println!("Parsed value = {}", value),
        Err(err) => println!("Parse error: {}", err),
    }

    let bad_input = "abc";
    match parse_sensor_value(bad_input) {
        Ok(value) => println!("Parsed value = {}", value),
        Err(err) => println!("Parse error: {}", err),
    }

    // -------------------------
    // Combined example
    // -------------------------
    match build_sensor_reading("ph", "6.8") {
        Ok(reading) => {
            println!("\nBuilt sensor reading successfully:");
            print_sensor_reading(&reading);
        }
        Err(err) => println!("Error building sensor reading: {}", err),
    }

    match build_sensor_reading("co2", "500") {
        Ok(reading) => {
            println!("\nBuilt sensor reading successfully:");
            print_sensor_reading(&reading);
        }
        Err(err) => println!("Error building sensor reading: {}", err),
    }
}

fn find_sensor_unit(sensor_name: &str) -> Option<&'static str> {
    match sensor_name {
        "temp" => Some("C"),
        "humidity" => Some("%"),
        "ph" => Some("pH"),
        "ec" => Some("mS/cm"),
        _ => None,
    }
}

fn parse_sensor_value(input: &str) -> Result<f64, String> {
    match input.parse::<f64>() {
        Ok(value) => Ok(value),
        Err(_) => Err(format!("'{}' is not a valid number", input)),
    }
}

fn build_sensor_reading(sensor_name: &str, raw_value: &str) -> Result<SensorReading, String> {
    let unit = find_sensor_unit(sensor_name)
        .ok_or_else(|| format!("sensor '{}' is not supported", sensor_name))?;

    let value = parse_sensor_value(raw_value)?;

    Ok(SensorReading {
        sensor_name: sensor_name.to_string(),
        value,
        unit: unit.to_string(),
    })
}

fn print_sensor_reading(reading: &SensorReading) {
    println!("Sensor Name : {}", reading.sensor_name);
    println!("Value       : {:.2}", reading.value);
    println!("Unit        : {}", reading.unit);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_sensor_unit_some() {
        let unit = find_sensor_unit("temp");
        assert_eq!(unit, Some("C"));
    }

    #[test]
    fn test_find_sensor_unit_none() {
        let unit = find_sensor_unit("co2");
        assert_eq!(unit, None);
    }

    #[test]
    fn test_parse_sensor_value_ok() {
        let value = parse_sensor_value("25.5");
        assert_eq!(value.unwrap(), 25.5);
    }

    #[test]
    fn test_parse_sensor_value_err() {
        let value = parse_sensor_value("abc");
        assert!(value.is_err());
    }

    #[test]
    fn test_build_sensor_reading_ok() {
        let reading = build_sensor_reading("ph", "6.8").unwrap();
        assert_eq!(reading.sensor_name, "ph");
        assert_eq!(reading.value, 6.8);
        assert_eq!(reading.unit, "pH");
    }

    #[test]
    fn test_build_sensor_reading_invalid_sensor() {
        let reading = build_sensor_reading("co2", "500");
        assert!(reading.is_err());
    }

    #[test]
    fn test_build_sensor_reading_invalid_value() {
        let reading = build_sensor_reading("temp", "bad");
        assert!(reading.is_err());
    }
}