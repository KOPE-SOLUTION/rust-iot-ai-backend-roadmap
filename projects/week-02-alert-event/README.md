# Week 2 — Create AlertEvent struct

## Objective

Create a Rust `AlertEvent` struct to represent alert and abnormal events in an IoT backend system.

## Fields

- `event_id`
- `device_id`
- `alert_type`
- `severity`
- `message`
- `timestamp`
- `active`

## Features

- Construct new alert events with `AlertEvent::new`
- Return current event status (`ACTIVE` / `RESOLVED`)
- Check whether an event is critical
- Print a formatted alert summary
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

This struct can be used for:
- threshold alerts
- sensor disconnect events
- pump or relay fault events
- instrusion detection events
- AI-generated abnormal event records

## Status

Completed