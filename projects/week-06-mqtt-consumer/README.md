# Week 6 — MQTT Consumer Service

## Objective

Build a Rust-based MQTT consumer service for IoT telemetry ingestion.

## Features

- Connect to MQTT broker
- Subscribe to telemetry topics
- Receive publish messages
- Parse JSON payload into Rust structs
- Log structured events using tracing

## Topic Pattern

```bash
site/+/device/+/telemetry/+
```

## Example Topic

```bash
site/farm-1/device/node-01/telemetry/temp
```

## Example Payload

```sh
{
  "device_id": "node-01",
  "sensor": "temp",
  "value": 27.5,
  "unit": "C",
  "timestamp": "2026-03-26T10:30:00Z"
}
```

## Run

`cargo run`

Example Publish Command

```bash
mosquitto_pub -h 127.0.0.1 -p 1883 \
  -t site/farm-1/device/node-01/telemetry/temp \
  -m "{\"device_id\":\"node-01\",\"sensor\":\"temp\",\"value\":27.5,\"unit\":\"C\",\"timestamp\":\"2026-03-26T10:30:00Z\"}"
```

## Status

Completed