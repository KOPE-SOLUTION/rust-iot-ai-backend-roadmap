# Task 4 — Parse Telemetry JSON Payload

## Objective

Convert raw MQTT payload data into strongly typed Rust structures.

When an MQTT message is received, the payload arrives as raw bytes.  
In this task we decode the payload into a UTF-8 string and then parse it as JSON using `serde`.

This allows the backend service to work with structured telemetry data instead of raw message bytes.

---

## Message Processing Pipeline

```text
MQTT Publish
      ↓
Raw Payload (bytes)
      ↓
UTF-8 String
      ↓
JSON
      ↓
Rust Struct
```

## Telemetry Data Structure

Telemetry messages from IoT devices are mapped into the following Rust struct:

```rs
#[derive(Debug, Deserialize)]
struct TelemetryPayload {
    device_id: String,
    sensor: String,
    value: f64,
    unit: String,
    timestamp: String,
}
```

This structure defines the schema expected from incoming telemetry messages.

---

## Example Telemetry Payload

Example JSON message sent by a device:
```json
{
  "device_id": "node-01",
  "sensor": "temp",
  "value": 27.5,
  "unit": "C",
  "timestamp": "2026-03-26T10:30:00Z"
}
```

---

## Parsing Workflow

The MQTT consumer performs the following steps when a message is received:
1. Convert payload bytes into UTF-8 string
2. Deserialize JSON into Rust struct
3. Log structured telemetry information

Example code:

```rs
let payload_str = std::str::from_utf8(&publish.payload)?;

let telemetry: TelemetryPayload =
    serde_json::from_str(payload_str)?;

info!(
    device_id = %telemetry.device_id,
    sensor = %telemetry.sensor,
    value = telemetry.value,
    unit = %telemetry.unit,
    timestamp = %telemetry.timestamp,
    "parsed telemetry payload"
);
```

---

## Error Handling

Two types of errors are handled:

### Invalid UTF-8 Payload

If the payload cannot be converted to UTF-8 text.

Example log: `invalid UTF-8 payload`

### Invalid JSON Format

If the payload does not match the expected JSON schema.

Example log: `failed to parse telemetry JSON`

This prevents malformed messages from crashing the ingestion service.

---

## Example Runtime Log

Example output when a valid telemetry message is received:

```sh
INFO parsed telemetry payload
device_id=node-01
sensor=temp
value=27.5
unit=C
timestamp=2026-03-26T10:30:00Z
```

## Why This Step Matters

Parsing JSON payloads enables the backend to:
- Validate telemetry schema
- Process structured data
- Store data in databases
- Trigger alerts or analytics pipelines

This step converts raw MQTT events into meaningful application data.