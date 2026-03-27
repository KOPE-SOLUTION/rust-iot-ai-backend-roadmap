# MQTT → SQLite → API Integration

## Objective

Integrate Rust MQTT consumer, SQLite storage, and Axum API into one working IoT backend service.

## Flow

MQTT Broker → Rust Consumer → SQLite → Axum API

## Features

- MQTT telemetry subscription
- JSON payload parsing
- SQLite telemetry storage
- API endpoints for devices and latest telemetry
- tracing-based structured logs

## Endpoints

- `GET /health`
- `GET /devices`
- `GET /telemetry/latest`

## MQTT Topic

```text
site/+/device/+/telemetry/+
```

## Example Payload

```json
{
  "device_id": "node-01",
  "sensor": "temp",
  "value": 31.2,
  "unit": "C",
  "timestamp": "2026-03-26T11:30:00Z"
}
```

## Run

```bash
cargo run
```

## Status

In Progress