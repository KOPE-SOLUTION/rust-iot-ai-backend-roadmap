# Week 2 — Create Device struct

## Objective

Create a Rust `Device` struct to represent an IoT device in a backend system.

## Fields

- `device_id`
- `site_id`
- `name`
- `firmware_version`
- `online`

## Features

- Construct a new device with `Device::new`
- Return online/offline status
- Print a formatted device summary
- Validate basic behavior using unit tests

## How to Run

```bash
cargo run
```

## How to Test

```bash
cargo test
```

## Example Use Case

This struc can be used as the base model for:
- IoT device registry
- backend API responses
- telemetry ownership
- alert/event relation mapping

## Status

Completed