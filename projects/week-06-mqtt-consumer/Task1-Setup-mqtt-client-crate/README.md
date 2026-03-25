# Task 1 — Setup MQTT Client

## Objective

Initialize a Rust project capable of connecting to an MQTT broker using the `rumqttc` crate.

This task focuses on setting up the MQTT client environment and verifying that the application can successfully establish a connection to the broker.

---

## Concepts Covered

- MQTT protocol basics
- Rust MQTT client (`rumqttc`)
- Async networking with Tokio
- Event loop handling
- Structured logging with `tracing`

---

## Architecture

```mermaid
graph LR
    A[Rust Application] --> B[MQTT Client<br/>(rumqttc)]
    B --> C[MQTT Broker<br/>(Mosquitto)]
```

---

## Dependencies

Cargo.toml

```toml
[dependencies]
rumqttc = "0.24"
tokio = { version = "1", features = ["full"] }
tracing = "0.1"
tracing-subscriber = "0.3"
```

---

## Example Code

```rs
use rumqttc::{AsyncClient, Event, Incoming, MqttOptions, QoS};
use tokio::time::{sleep, Duration};
use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let mut mqttoptions = MqttOptions::new("rust-consumer-01", "127.0.0.1", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(10));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    client
        .subscribe("site/+/device/+/telemetry/+", QoS::AtMostOnce)
        .await
        .expect("failed to subscribe");

    info!("MQTT consumer started. Waiting for messages...");

    loop {
        match eventloop.poll().await {
            Ok(notification) => match notification {
                Event::Incoming(Incoming::Publish(publish)) => {
                    info!(
                        topic = %publish.topic,
                        payload = ?publish.payload,
                        "received publish message"
                    );
                }
                other => {
                    info!(event = ?other, "mqtt event");
                }
            },
            Err(err) => {
                error!(error = %err, "mqtt connection error");
                sleep(Duration::from_secs(2)).await;
            }
        }
    }
}
```

---

## Running the Consumer

Start the application `cargo run`

Expected output

```bash
MQTT consumer started. Waiting for messages...
mqtt event event=Incoming(ConnAck(...))
mqtt event event=Incoming(SubAck(...))
```

## Testing MQTT Publish


### Install MQTT clients

`sudo apt install mosquitto-clients`

<br>

### Publish a test message

```bash
mosquitto_pub -h 127.0.0.1 \
-t site/farm-1/device/node-01/telemetry/temp \
-m '{"device_id":"node-01","sensor":"temp","value":27.5,"unit":"C","timestamp":"2026-03-26T10:30:00Z"}'
```

### Example log

```bash
received publish message
topic=site/farm-1/device/node-01/telemetry/temp
```

## Result

The Rust MQTT consumer successfully:
- connects to the broker
- subscribes to telemetry topics
- receives publish messages
- logs events using structured tracing