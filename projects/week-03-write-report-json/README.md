# Week 3 — Write Report JSON Output

## Objective

Read telemetry data from JSON, validate it, and write a validation report to a new JSON file.

## Pipeline

1. Read `telemetry.json`
2. Deserialize into Rust struct
3. Validate payload values
4. Build validation report
5. Write `report.json`

## Output Fields

- `device_id`
- `sensor`
- `value`
- `status`
- `message`

## How to Run

```bash
cargo run
```

## Example Files

- `telemetry.json` — input
- `report.json` — output

## Status

Completed