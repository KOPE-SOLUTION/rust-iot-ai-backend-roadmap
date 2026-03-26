# Task 5 — Structured Logging with Tracing

## Objective

Add structured logging to the MQTT consumer service using the `tracing` crate.

Structured logging improves observability by providing machine-readable logs that contain contextual information such as topic names, device identifiers, and telemetry values.

Instead of printing plain text logs, the application emits structured log events that can later be consumed by log aggregation systems or monitoring platforms.

---

## Logging Architecture

```text
MQTT Event
    ↓
Rust Consumer
    ↓
Tracing Logger
    ↓
Structured Logs
    ↓
Monitoring / Debugging
```

## Logging Crates Used

The following dependencies were added to enable structured logging:

```toml
tracing = "0.1"
tracing-subscriber = "0.3"
```

## These crates provide:

- structured log events
- contextual metadata
- async-safe logging
- integration with observability systems

## Logger Initialization

The tracing subscriber must be initialized at application startup.

`tracing_subscriber::fmt::init();`

This enables formatted log output for the entire application.

## Logging MQTT Events

The MQTT consumer logs different types of events:

## Connection Events

Example log when connecting to the broker:

```bash
INFO MQTT consumer started. Waiting for messages...
INFO Incoming(ConnAck { code: Success })
```

## Subscription Events

Example log when subscribing to topics:

`INFO Incoming(SubAck { return_codes: [Success] })`

## Telemetry Message Events

When a telemetry message is received:

```bash
info!(
    topic = %publish.topic,
    payload = ?publish.payload,
    "received publish message"
);
```

Example output: `INFO received publish message topic=site/farm-1/device/node-01/telemetry/temp`

## Structured Telemetry Logging

After parsing the payload JSON, telemetry data is logged as structured fields.

Example code:

```rs
info!(
    device_id = %data.device_id,
    sensor = %data.sensor,
    value = data.value,
    unit = %data.unit,
    timestamp = %data.timestamp,
    "parsed telemetry payload"
);
```

Example output:

```bash
INFO parsed telemetry payload
device_id=node-01
sensor=temp
value=27.5
unit=C
timestamp=2026-03-26T10:30:00Z
```

## Why Structured Logging Matters

Structured logs allow backend services to:
- debug production systems
- trace telemetry ingestion
- monitor system health
- detect malformed messages
- analyze device activity

In large IoT platforms, structured logging is essential for diagnosing issues across thousands of devices.

## Example Runtime Log

Example runtime output of the MQTT consumer:

```bash
INFO MQTT consumer started. Waiting for messages...
INFO Incoming(ConnAck { code: Success })
INFO Incoming(SubAck { return_codes: [Success] })
INFO received publish message topic=site/farm-1/device/node-01/telemetry/temp
INFO parsed telemetry payload device_id=node-01 sensor=temp value=27.5 unit=C
Summary
```

By completing Task 5, the MQTT consumer service now includes structured logging capabilities.

Key improvements:
- MQTT connection events logged
- topic subscription events logged
- telemetry ingestion events logged
- structured fields for telemetry data
- improved debugging and observability

## Next Steps

In the following weeks, the MQTT consumer will be extended to:
- store telemetry data in SQLite
- expose data through REST APIs
- integrate with analytics and rule engines