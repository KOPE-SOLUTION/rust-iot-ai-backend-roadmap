# Week 4 — Axum API Basics

## Objective

Build the Rust backend API using Axum.

### GET api

Try:

```bash
http://localhost:3000/health
http://localhost:3000/devices
http://localhost:3000/telemetry/latest
```

logs:

```json
2026-03-25T19:52:47.081434Z  INFO week_04_axum_api::handlers::telemetry: Fetching latest telemetry...
2026-03-25T19:53:05.122971Z  INFO week_04_axum_api::handlers::health: Checking health...
2026-03-25T19:54:21.702114Z  INFO week_04_axum_api::handlers::devices: Fetching devices...
```

## How to Run

```bash
cargo run
```

Server will start at: `http://172.0.0.1:3000`

## How to Test

```bash
curl http://127.0.0.1:3000/health
curl http://127.0.0.1:3000/devices
curl http://127.0.0.1:3000/telemetry/latest
```

## Status

Completed