# Rust IoT + AI Backend Roadmap

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)
![Platform](https://img.shields.io/badge/platform-IoT%20%7C%20Backend%20%7C%20AI-red.svg)

A structured 90-day roadmap for learning and building real-world backend systems with Rust, focused on IoT, data processing, and AI event integration.

---

## Overview

This repository documents a hands-on Rust learning journey designed around production-oriented backend engineering.

The roadmap combines:
- Backend development (Rust + Axum)
- MQTT-based IoT ingestion
- Database integration (SQLx)
- Data processing pipelines (ETL + Polars)
- AI event integration

---

## Objectives

- Master Rust for backend systems
- Build async services with Tokio
- Integrate MQTT for IoT telemetry
- Store and process data efficiently
- Build analytics and rule engine systems
- Connect AI events into backend pipelines

---

## Roadmap Structure

### Month 1 — Rust Fundamentals
- Syntax, ownership, borrowing
- Struct / Enum / Result / Option
- JSON & File I/O
- Axum API basics

### Month 2 — Backend Systems
- Async Rust (Tokio)
- MQTT consumer service
- Database integration (SQLx)
- API + MQTT + DB pipeline

### Month 3 — Data + AI Integration
- ETL pipeline
- Polars analytics
- Rule engine
- AI event gateway
- Capstone IoT platform

---

## Architecture Overview

```text
IoT Devices → MQTT Broker → Rust Backend → Database → API → Dashboard
```

---

## Planned Mini Projects

- Sensor CLI tool
- Telemetry JSON validator
- Axum API starter
- MQTT consumer service
- SQLx storage layer
- Polars analytics engine
- Rule-based alert system
- AI event ingestion service
- Capstone IoT event platform

---

## Tech Stack
- Language: Rust
- Async runtime: Tokio
- Web framework: Axum
- Database: PostgreSQL + SQLx
- MQTT: rumqttc
- Data processing: Polars
- Serialization: serde / serde_json

---

## Project Statatus

In Progress — Actively building week-by-week based on roadmap.

---

## Future Direction

This project will evolve into a scalable backend platform for:
- Smart Farm
- Smart Hydroponic
- Smart Building
- Industrial IoT
- AI-driven monitoring systems

---

## License

This project is licensed under the MIT License – see the LICENSE file for details.

---

## Author

Kittisak Hanheam
Public Release Date: 2026-03-22
