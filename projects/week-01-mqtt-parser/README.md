# Week 1 — MQTT Topic Parser

## Objective

Build a Rust CLI tool to parse MQTT topic strings into structured fields for backend and IoT systems.

## Supported Topic Format

```text
site/<site_id>/device/<device_id>/<category>/<sensor>
```

## Example Topics

```build
site/farm-1/device/node-01/telemetry/temp
site/farm-2/device/node-22/telemetry/humidity
site/greenhouse-a/device/ec-meter-01/alert/high_ec
```

## Parsed Output

- site_id
- device_id
- category
- sensor

## How to Run

```bash
cargo run -- site/farm-1/device/node-01/telemetry/temp
cargo run -- site/farm-2/device/node-22/telemetry/humidity
cargo run -- site/greenhouse-a/device/ec-meter-01/alert/high_ec
```

## How to Test

```bash
cargo test
```

## Status

Completed