# Ignition

Ignition is a distributed Solana transaction execution and orchestration engine built in Rust.

The goal of the project is to provide reliable transaction execution infrastructure for Solana applications through:

- transaction queuing
- execution orchestration
- retries
- RPC failover
- realtime status streaming
- observability
- distributed workers

---

# Vision

Most Solana applications rely on fragile transaction pipelines that fail under congestion, RPC instability, or blockhash expiration.

Ignition aims to provide a production-grade execution layer that abstracts transaction reliability into a scalable backend infrastructure service.

---

# Current Features

- Rust workspace architecture
- Axum API server
- PostgreSQL integration
- Shared application state
- Transaction submission endpoint
- In-memory queue system
- Async worker runtime
- Background transaction processing

---

# Workspace Structure

```text
crates/
├── api         # HTTP API server
├── common      # Shared utilities/config/errors
├── queue       # Queue engine + worker orchestration
├── storage     # Database abstractions
└── types       # Shared DTOs/types
