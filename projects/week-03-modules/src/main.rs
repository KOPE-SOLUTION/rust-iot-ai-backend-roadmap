mod models;
mod services;

use models::device::Device;
use models::telemetry::Telemetry;
use services::sensor_service::evaluate_telemetry;

fn main(){
    let device = Device::new("node-01", "farm-1", "Temp Sensor", true);

    let telemetry = Telemetry::new("node-01", "temp", 32.5);

    println!("Device: {:#?}", device);
    println!("Telemetry: {:#?}", telemetry);

    let status = evaluate_telemetry(&telemetry);

    println!("Status: {}", status);
}