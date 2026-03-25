use tokio::time::{sleep, Duration};
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    info!("Simulating concurrent event sources...");

    let sensor_1 = tokio::spawn(simulate_sensor("node-01", "temp", 27.5, 2));
    let sensor_2 = tokio::spawn(simulate_sensor("node-02", "humidity", 68.2, 1));
    let sensor_3 = tokio::spawn(simulate_sensor("node-03", "ph", 6.8, 3));

    sensor_1.await.unwrap();
    sensor_2.await.unwrap();
    sensor_3.await.unwrap();

    info!("All sensor tasks completed.");
}

async fn simulate_sensor(device_id: &str, sensor: &str, value: f64, delay_secs: u64) {
    info!("{} -> {} starting...", device_id, sensor);
    sleep(Duration::from_secs(delay_secs)).await;
    info!("{} -> {} = {}", device_id, sensor, value);
}