# Week 3 — Validate Payload Values

## Objective

Validate telemetry payload values after reading JSON input into Rust structs.

## Validation Rules

### Required fields
- `device_id` must not be empty
- `sensor` must not be empty
- `unit` must not be empty
- `timestamp` must not be empty

### Sensor ranges
- `temp`: -40.0 to 125.0
- `humidity`: 0.0 to 100.0
- `ph`: 0.0 to 14.0
- `ec`: 0.0 to 20.0

## Concepts Covered

- Rust struct validation
- `Result<(), String>`
- match-based rule checking
- backend payload guardrails
- unit testing validation logic

## How to Run

```bash
cargo run
```

## Status

Completed