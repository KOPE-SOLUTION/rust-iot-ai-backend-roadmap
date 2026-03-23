use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        print_usage(&args[0]);
        process::exit(1);
    }

    let current_ma: f64 = parse_f64(&args[1], "current_mA");
    let eu_min: f64 = parse_f64(&args[2], "eu_min");
    let eu_max: f64 = parse_f64(&args[3], "eu_max");
    let unit = &args[4];

    if eu_max <= eu_min{
        eprintln!("Error: eu_max must be greater than eu_min.");
        process::exit(1);
    }

    let result = convert_4_20ma_to_eu(current_ma, eu_min, eu_max);

    println!("=== 4-20mA Converter ===");
    println!("Input Current: {:.2} mA", current_ma);
    println!("EU range     : {:.2} to {:.2} {}", eu_min, eu_max, unit);
    println!("Output value : {:.2} {}", result, unit);

    if current_ma < 4.0{
        println!("Status     : BELOW RANGE");
    }else if current_ma > 20.0{
        println!("Status     : ABOVE RANGE");
    }else{
        println!("Status     : OK");
    }
}

fn parse_f64(input: &str, field_name: &str) -> f64{
    match input.parse::<f64>(){
        Ok(value) => value,
        Err(_) =>{
            eprintln!("Error: {} must be a valid number", field_name);
            process::exit(1);
        }
    }
}

fn convert_4_20ma_to_eu(current_ma: f64, eu_min: f64, eu_max: f64) -> f64{
    ((current_ma - 4.0) / 16.0) * (eu_max - eu_min) + eu_min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4ma_to_min() {
        let result = convert_4_20ma_to_eu(4.0, 0.0, 2000.0);
        assert!((result - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_12ma_to_mid() {
        let result = convert_4_20ma_to_eu(12.0, 0.0, 2000.0);
        assert!((result - 1000.0).abs() < 0.0001);
    }

    #[test]
    fn test_20ma_to_max() {
        let result = convert_4_20ma_to_eu(20.0, 0.0, 2000.0);
        assert!((result - 2000.0).abs() < 0.0001);
    }

    #[test]
    fn test_pressure_range() {
        let result = convert_4_20ma_to_eu(8.0, 0.0, 10.0);
        assert!((result - 2.5).abs() < 0.0001);
    }
}

fn print_usage(program_name: &str) {
    eprintln!("Usage:");
    eprintln!("  {} <current_mA> <eu_min> <eu_max> <unit>", program_name);
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  {} 4 0 2000 W/m2", program_name);
    eprintln!("  {} 12 0 2000 W/m2", program_name);
    eprintln!("  {} 20 0 2000 W/m2", program_name);
    eprintln!("  {} 8 0 10 bar", program_name);
}