use tokio::time::{sleep, Duration};
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    info!("Starting async/await basics demo...");

    let temp = read_temperature().await;
    let humidity = read_humidity().await;

    info!("Temperature = {}", temp);
    info!("Humidity = {}", humidity);

    info!("Done.");
}

async fn read_temperature() -> f64 {
    info!("Reading temperature...");
    sleep(Duration::from_secs(2)).await;
    27.5
}

async fn read_humidity() -> f64 {
    info!("Reading humidity...");
    sleep(Duration::from_secs(1)).await;
    68.2
}