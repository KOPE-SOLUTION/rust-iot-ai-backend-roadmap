# Rust IoT + AI Backend Roadmap

A structured 90-day roadmap for learning and building real-world backend systems with Rust, focused on IoT, data processing, and AI event integration.

## Overview

This repository documents a hands-on Rust learning journey designed around production-oriented backend engineering.  
The roadmap combines backend development, MQTT-based IoT ingestion, database integration, analytics pipelines, and AI event processing.

The goal is not only to learn Rust syntax, but to build a backend architecture that can evolve into real systems such as:

- Smart Farm
- Smart Hydroponic
- Smart Building
- Security / CCTV analytics
- Industrial IoT platforms
- AI-integrated monitoring systems

## Objectives

- Learn Rust fundamentals through practical mini-projects
- Build backend APIs with Axum and Tokio
- Process MQTT telemetry from IoT devices
- Store and query structured data using SQLx
- Build ETL and analytics workflows
- Integrate AI-generated events into backend pipelines
- Design a capstone Rust IoT event platform

## Roadmap Structure

### Month 1 — Rust Fundamentals
- Rust syntax basics
- Ownership and borrowing
- Struct / Enum / Result / Option
- Modules, JSON, and file I/O
- Intro to Axum API development

### Month 2 — Backend Systems
- Async Rust with Tokio
- MQTT consumer service
- Database integration with SQLx
- REST API + MQTT + DB integration
- Service-oriented backend structure

### Month 3 — Data + AI Integration
- CSV / JSON ETL
- Analytics with Polars
- Rule engine design
- AI event gateway
- Capstone backend platform

## Current Work Packages

This roadmap is also tracked in OpenProject using a hierarchical structure:

- Month
  - Week
    - Task

Example:

- Month 1 — Rust Fundamentals
  - Week 1 — Rust Basics + Ownership
    - Learn Rust syntax basics
    - Practice ownership and borrowing
    - Build sensor CLI tool
    - Implement 4–20mA converter
    - Create MQTT topic parser

## Planned Mini Projects

- sensor CLI tool
- telemetry JSON validator
- Axum API starter
- MQTT consumer service
- SQLx storage service
- telemetry ETL tool
- Polars analytics summary
- rule engine service
- AI event gateway
- capstone IoT event platform

## Suggested Tech Stack

- **Language:** Rust
- **Async runtime:** Tokio
- **Web framework:** Axum
- **Database layer:** SQLx
- **MQTT client:** rumqttc
- **Data processing:** Polars
- **Serialization:** serde / serde_json

## Repository Goals

This repository is intended to become:

1. A personal Rust learning roadmap
2. A portfolio project for backend / IoT / AI engineering
3. A foundation for future real-world product development

## Project Status

Active development.

The roadmap has been imported into OpenProject and is being executed week by week.

## Future Direction

This project may evolve into a larger Rust-based backend platform for:

- IoT telemetry ingestion
- event-driven alerting
- AI-generated event processing
- analytics dashboards
- smart infrastructure systems

## License

MIT
