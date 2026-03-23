use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        print_usage(&args[0]);
        process::exit(1);
    }

    let sensor_name = args[1].to_lowercase();
    let value: f64 = match args[2].parse() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Error: value must be a number");
            process::exit(1);
        }
    };
    let unit = &args[3];

    let status = evaluate_sensor(&sensor_name, value);

    println!("=== Sensor CLI Tool ===");
    println!("Sensor : {}", sensor_name);
    println!("Value  : {}", value);
    println!("Unit   : {}", unit);
    println!("Status : {}", status);
}

fn print_usage(program_name: &str) {
    eprintln!("Usage:");
    eprintln!("  {} <sensor_name> <value> <unit>", program_name);
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  {} temp 25.6 C", program_name);
    eprintln!("  {} ph 6.8 pH", program_name);
    eprintln!("  {} ec 1.45 mS/cm", program_name);
}

fn evaluate_sensor(sensor_name: &str, value: f64) -> &'static str {
    match sensor_name {
        "temp" => {
            if value < 18.0 {
                "LOW"
            } else if value > 30.0 {
                "HIGH"
            } else {
                "OK"
            }
        }
        "ph" => {
            if value < 5.5 {
                "LOW"
            } else if value > 7.5 {
                "HIGH"
            } else {
                "OK"
            }
        }
        "ec" => {
            if value < 0.8 {
                "LOW"
            } else if value > 2.5 {
                "HIGH"
            } else {
                "OK"
            }
        }
        "humidity" => {
            if value < 40.0 {
                "LOW"
            } else if value > 80.0 {
                "HIGH"
            } else {
                "OK"
            }
        }
        _ => "UNKNOWN SENSOR",
    }
}