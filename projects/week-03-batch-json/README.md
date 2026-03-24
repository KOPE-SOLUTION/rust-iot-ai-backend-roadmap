# Week 3 — Deserialize Batch JSON into Structs

## Objective

Read a batch JSON file containing multiple telemetry records, deserialize it into Rust structs, validate each record, and generate a JSON validation report.

## Concepts Covered

- JSON array deserialization
- `Vec<Telemetry>`
- validation pipeline
- report generation
- JSON output writing
- unit testing

## Input File

`telemetry_batch.json`

Example:

```json
[
  {
    "device_id": "node-01",
    "sensor": "temp",
    "value": 27.5,
    "unit": "C",
    "timestamp": "2026-03-24T16:00:00Z"
  },
  {
    "device_id": "node-02",
    "sensor": "humidity",
    "value": 65.0,
    "unit": "%",
    "timestamp": "2026-03-24T16:01:00Z"
  },
  {
    "device_id": "node-03",
    "sensor": "humidity",
    "value": 120.0,
    "unit": "%",
    "timestamp": "2026-03-24T16:02:00Z"
  }
]
```

## Output File

`batch_report.json`

Example:

```json
# Week 3 — Deserialize Batch JSON into Structs

## Objective

Read a batch JSON file containing multiple telemetry records, deserialize it into Rust structs, validate each record, and generate a JSON validation report.

## Concepts Covered

- JSON array deserialization
- `Vec<Telemetry>`
- validation pipeline
- report generation
- JSON output writing
- unit testing

## Input File

`telemetry_batch.json`

Example:

```json
[
  {
    "device_id": "node-01",
    "sensor": "temp",
    "value": 27.5,
    "unit": "C",
    "timestamp": "2026-03-24T16:00:00Z"
  },
  {
    "device_id": "node-02",
    "sensor": "humidity",
    "value": 65.0,
    "unit": "%",
    "timestamp": "2026-03-24T16:01:00Z"
  },
  {
    "device_id": "node-03",
    "sensor": "humidity",
    "value": 120.0,
    "unit": "%",
    "timestamp": "2026-03-24T16:02:00Z"
  }
]
```

## Pipeline
1. Read `telemetry_batch.json`
2. Deserialize JSON into `Vec<Telemetry>`
3. Validate each record
4. Build `ValidationReport`
5. Write `batch_report.json`

## How to Run

```bash
cargo run
```

## How to Test

```bash
cargo test
```

## Project Files

- `src/main.rs`
- `telemetry_batch.json`
- `batch_report.json`
- `README.md`

## Status

Completed