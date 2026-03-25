use tokio::{
    sync::mpsc,
    time::{sleep, Duration},
};
use tracing::info;

#[derive(Debug)]
struct SensorMessage {
    device_id: String,
    sensor: String,
    value: f64,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    info!("Starting channel demo...");

    let (tx, mut rx) = mpsc::channel::<SensorMessage>(32);

    let tx1 = tx.clone();
    let tx2 = tx.clone();
    let tx3 = tx.clone();

    let task1 = tokio::spawn(async move {
        simulate_sensor(tx1, "node-01", "temp", 27.5, 2).await;
    });

    let task2 = tokio::spawn(async move {
        simulate_sensor(tx2, "node-02", "humidity", 68.2, 1).await;
    });

    let task3 = tokio::spawn(async move {
        simulate_sensor(tx3, "node-03", "ph", 6.8, 3).await;
    });

    drop(tx);

    while let Some(msg) = rx.recv().await {
        info!(
            "Received => device: {}, sensor: {}, value: {}",
            msg.device_id, msg.sensor, msg.value
        );
    }

    task1.await.unwrap();
    task2.await.unwrap();
    task3.await.unwrap();

    info!("All messages processed.");
}

async fn simulate_sensor(
    tx: mpsc::Sender<SensorMessage>,
    device_id: &str,
    sensor: &str,
    value: f64,
    delay_secs: u64,
) {
    info!("{} -> {} starting...", device_id, sensor);

    sleep(Duration::from_secs(delay_secs)).await;

    let message = SensorMessage {
        device_id: device_id.to_string(),
        sensor: sensor.to_string(),
        value,
    };

    if let Err(err) = tx.send(message).await {
        info!("Failed to send message: {}", err);
    }
}