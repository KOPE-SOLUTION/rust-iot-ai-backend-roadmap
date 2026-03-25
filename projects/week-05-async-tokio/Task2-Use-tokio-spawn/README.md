# Week 5 — Async / Tokio

## Task 2 — Using `tokio::spawn`

### Objective

Demonstrate how to run asynchronous tasks concurrently using `tokio::spawn`.  
This allows multiple operations to run in parallel without blocking the main thread.

This concept is essential for:

- backend worker tasks
- MQTT consumers
- concurrent API processing
- IoT telemetry ingestion

---

## Key Concepts

- `tokio::spawn`
- asynchronous tasks
- `JoinHandle`
- concurrent execution

---

## Example Code

```rust
use tokio::time::{sleep, Duration};
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    info!("Starting tokio::spawn demo...");

    let temp_task = tokio::spawn(async {
        read_temperature().await
    });

    let humidity_task = tokio::spawn(async {
        read_humidity().await
    });

    let temp = temp_task.await.unwrap();
    let humidity = humidity_task.await.unwrap();

    info!("Temperature = {}", temp);
    info!("Humidity = {}", humidity);
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
```

## Expected Output

```bash
INFO Starting tokio::spawn demo...
INFO Reading temperature...
INFO Reading humidity...
INFO Temperature = 27.5
INFO Humidity = 68.2
```

The tasks execute concurrently instead of sequentially.

## Why This Matters

Using `tokio::spawn` allows your Rust backend to:
- process multiple sensor inputs
- handle API requests concurrently
- run background workers
- scale efficiently for IoT event processing

## Status

Completed