# Week 1 - 4-20mA Converter

## Objective

Build a Rust CLI tool to convert a 4–20mA signal into an engineering unit using linear scaling.

## Formula

```text
EU = ((mA - 4.0) / 16.0) * (EU_max - EU_min) + EU_min
```

## Features

- Accept current input in mA
- Accept engineering range min/max
- Accept output unit
- Print converted engineering value
- Show range status
- Include unit tests

## How to Run

```bash
cargo run -- 4 0 2000 W/m2
cargo run -- 12 0 2000 W/m2
cargo run -- 20 0 2000 W/m2
cargo run -- 8 0 10 bar
```

## How to Test

```bash
cargo test
```

## Example Output

```bash
=== 4–20mA Converter ===
Input current : 12.00 mA
EU range      : 0.00 to 2000.00 W/m2
Output value  : 1000.00 W/m2
Status        : OK
```

## Status

Completed