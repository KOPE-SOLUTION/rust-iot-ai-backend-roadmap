# Week 2 — Create Telemetry struct

## Objective

Create a Rust `Telemetry` struct to represent sensor readings from IoT devices in a backend system.

## Fields

- `device_id`
- `sensor_type`
- `value`
- `unit`
- `timestamp`

## Features

- Construct new telemetry data with `Telemetry::new`
- Print a telemetry summary
- Compare value against a threshold using `is_above`
- Validate behavior with unit tests

## How to Run

```bash
cargo run
```

## How to Test

```bash
cargo test
```

## Example Use Case

This struct can be used as the base model for:
- MQTT payload mapping
- database storage design
- telemetry APIs
- alert threshold evaluation

## Status

Completed