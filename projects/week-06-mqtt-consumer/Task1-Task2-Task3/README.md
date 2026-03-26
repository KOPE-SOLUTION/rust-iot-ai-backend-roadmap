# Week 6 — MQTT Consumer Service

## Overview

This module introduces how to build a Rust-based MQTT consumer for IoT telemetry ingestion.

The goal of this week is to understand how an IoT backend service connects to an MQTT broker, subscribes to telemetry topics, and receives messages from distributed sensor devices.

This forms the foundation for event-driven IoT platforms where devices publish data and backend services consume, process, and store telemetry.

---

## Architecture

```text
IoT Device
   ↓
MQTT Broker
   ↓
Rust MQTT Consumer
   ↓
Backend Processing
```

## Task 1 — Setup MQTT Client Crate

### Objective

Install and configure an MQTT client library in Rust.

We use the rumqttc crate which provides an async MQTT client suitable for backend services and IoT platforms.

### Dependencies

Added to `Cargo.toml`:

```toml
rumqttc = "0.24"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tracing = "0.1"
tracing-subscriber = "0.3"
```

### Concepts Learned
- Async MQTT client
- Tokio runtime
- Structured logging with tracing
- IoT message ingestion patterns

## Task 2 — Connect to MQTT Broker

### Objective

Establish a connection between the Rust consumer service and the MQTT broker.

Example configuration:

```rs
let mut mqttoptions = MqttOptions::new(
    "rust-consumer-01",
    "127.0.0.1",
    1883
);
```

### Connection Flow

```bash
Rust Client
    ↓
MQTT CONNECT
    ↓
Broker
    ↓
CONNACK
```

Example log: `Incoming(ConnAck { code: Success })`

This confirms that the Rust application successfully connected to the MQTT broker.

## Task 3 — Subscribe to Telemetry Topics

### Objective

Subscribe to IoT telemetry topics so the consumer can receive messages published by devices.

### Topic Pattern

`site/+/device/+/telemetry/+`

This wildcard pattern allows the consumer to receive telemetry from any site, device, and sensor.

### Example Topic

`site/farm-1/device/node-01/telemetry/temp`

### Subscribe Code

```rs
client
    .subscribe(
        "site/+/device/+/telemetry/+",
        QoS::AtMostOnce
    )
    .await?;
```

### Subscription Flow

```sh
Rust Client
   ↓
SUBSCRIBE
   ↓
Broker
   ↓
SUBACK
```

Example log: `Incoming(SubAck { return_codes: [Success] })`

## Example Telemetry Message

Example payload sent by an IoT device:

```js
{
  "device_id": "node-01",
  "sensor": "temp",
  "value": 27.5,
  "unit": "C",
  "timestamp": "2026-03-26T10:30:00Z"
}
```

Example publish command:

```bash
mosquitto_pub \
-h 127.0.0.1 \
-p 1883 \
-t site/farm-1/device/node-01/telemetry/temp \
-m '{"device_id":"node-01","sensor":"temp","value":27.5,"unit":"C","timestamp":"2026-03-26T10:30:00Z"}'
```

## Example Runtime Logs

Example output from the Rust consumer:

```sh
INFO MQTT consumer started. Waiting for messages...
INFO Incoming(ConnAck { code: Success })
INFO Incoming(SubAck { return_codes: [Success] })
INFO received publish message topic=site/farm-1/device/node-01/telemetry/temp
```

This confirms:
- broker connection works
- topic subscription works
- telemetry messages are received

## Summary

By completing Tasks 1–3 we successfully built the core MQTT ingestion layer of the IoT backend system.

Key capabilities implemented:

- MQTT client initialization
- Broker connection
- Topic subscription
- Message reception
- Structured logging