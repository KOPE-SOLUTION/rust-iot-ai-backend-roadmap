# Week 5 — Async / Tokio

## Task 3 — Simulate Concurrent Event Sources

### Objective

Simulate multiple IoT devices sending telemetry events concurrently using Tokio asynchronous tasks.

This task demonstrates how backend systems process multiple event sources in parallel.

---

## Key Concepts

- concurrent async tasks
- event source simulation
- asynchronous delays
- parallel execution with Tokio

---

## Example Code

```rust
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
```

## Example Output

```bash
INFO Simulating concurrent event sources...
INFO node-01 -> temp starting...
INFO node-02 -> humidity starting...
INFO node-03 -> ph starting...
INFO node-02 -> humidity = 68.2
INFO node-01 -> temp = 27.5
INFO node-03 -> ph = 6.8
INFO All sensor tasks completed.
```

The events complete in different order depending on the delay.

## Why This Matters

This pattern represents how real systems handle:

- IoT device telemetry
- distributed sensor networks
- background processing pipelines
- event-driven systems

<br>

In production environments, these events would typically arrive from:

- MQTT brokers
- message queues
- Kafka streams
- HTTP ingestion endpoints

## Status

Completed