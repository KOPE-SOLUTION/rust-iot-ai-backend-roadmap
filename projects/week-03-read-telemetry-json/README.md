# Week 3 — Read Telemetry JSON File

## Objective

Read telemetry data from a JSON file and deserialize it into a Rust struct.

## Concepts Covered

- File I/O with `std::fs`
- JSON deserialization with `serde`
- Error handling with `Result`
- Unit testing JSON parsing

## Files

- `telemetry.json` — sample telemetry input
- `src/main.rs` — JSON reader and parser

## How to Run

```bash
cargo run
```

## How to Test

```bash
cargo test
```

## Example JSON

```json
{
  "device_id": "node-01",
  "sensor": "temp",
  "value": 27.5,
  "unit": "C",
  "timestamp": "2026-03-24T13:00:00Z"
}
```

## Status

Completed