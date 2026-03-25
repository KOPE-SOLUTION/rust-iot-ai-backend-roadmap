# Week 5 — Async Rust and Tokio

This module introduces asynchronous programming in Rust using the Tokio runtime.

The goal of this week is to understand how Rust handles concurrent workloads in backend and distributed systems.

---

# Learning Objectives

After completing this module you should understand:

- async/await in Rust
- Tokio runtime
- task spawning
- concurrent event processing
- message passing with channels
- timeout and retry strategies

---

# Task Overview

| Task | Topic | Description |
|-----|------|-------------|
| Task 1 | Async / Await | Basic asynchronous functions |
| Task 2 | Tokio Spawn | Running concurrent tasks |
| Task 3 | Concurrent Sources | Simulating multiple event producers |
| Task 4 | Channels | Message passing using Tokio mpsc |
| Task 5 | Timeout / Retry | Handling timeouts and implementing retry logic |

---

# Architecture Evolution

During this week we gradually build a simple event pipeline.

### Step 1

Async Function

### Step 2

Concurrent Tasks

### Step 3

Multipe Event Surces

### Step 4

```mermaid
graph LR
    A[Event Sources] --> B[Channel]
    B --> C[Receiver]
```

### Step 5

```mermaid
graph LR
    A[Async Operation] --> B[Timeout Guard]
    B --> C[Retry Logic]
```

---

# Real-World Backend Mapping

The concepts learned here directly apply to real backend systems.

| Concept | Real System Example |
|-------|--------------------|
| async tasks | web server requests |
| channels | message queues |
| spawn | worker pools |
| timeout | API calls |
| retry | network resilience |

---

# Technologies Used

- Rust
- Tokio
- tracing

---

# How to Run

```bash
cargo run
```

# Status

Completed