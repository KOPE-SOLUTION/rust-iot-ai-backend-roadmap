# Week 1 — Sensor CLI Tool

## Objective

Build a simple Rust CLI tool that accepts sensor data from command-line arguments and evaluates whether the reading is OK, LOW, or HIGH.

## Features

- Accept sensor name, value, and unit
- Parse numeric input
- Evaluate sensor status using simple threshold rules
- Print formatted sensor output

## Supported Sensors

- temp
- ph
- ec
- humidity

## How to Run

```bash
cargo run -- temp 25.6 C
cargo run -- ph 6.8 pH
cargo run -- ec 1.45 mS/cm
cargo run -- humidity 85 %
```

## Example Output

```bash
=== Sensor CLI Tool ===
Sensor : temp
Value  : 25.6
Unit   : C
Status : OK
```

## Status

Completed