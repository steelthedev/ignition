# Ignition

Ignition is a distributed Solana transaction execution and orchestration engine built in Rust.

The project focuses on reliable transaction execution infrastructure for Solana applications through:
- async job orchestration
- transaction simulation
- retries
- realtime event streaming
- worker systems
- RPC abstraction
- resilient backend architecture

---

# Vision

Most Solana applications rely on fragile transaction execution pipelines that break under:
- RPC instability
- congestion
- blockhash expiration
- unreliable retries
- websocket failures
- poor observability

Ignition aims to provide a production-grade execution layer that abstracts transaction reliability into scalable infrastructure.

---

# Philosophy

Ignition is designed as both:
- a serious systems/backend engineering project
- a production-grade Solana infrastructure platform

The project heavily emphasizes:
- async architecture
- distributed systems thinking
- resilient backend design
- event-driven systems
- reliability engineering
- strong Rust abstractions
- infrastructure-oriented architecture

The goal is to deeply understand and build the systems behind reliable blockchain infrastructure.

---

# Current Features

## API Layer
- Axum HTTP API
- Transaction submission endpoint
- Transaction status querying
- WebSocket realtime updates

---

## Queue + Worker System
- Async in-memory queue engine
- Background worker runtime
- Retry orchestration
- Exponential backoff handling
- Stateful transaction lifecycle management

---

## Execution Architecture
- Trait-based execution abstraction
- Pluggable executor system
- Dependency-injected infrastructure services

---

## Solana Infrastructure
- Async Solana RPC integration
- RPC abstraction layer
- Transaction deserialization
- Base64 transaction decoding
- Transaction simulation pipeline

---

## Event System
- Internal event bus
- Pub/sub architecture
- WebSocket broadcasting
- Realtime transaction lifecycle events

---

## Storage
- PostgreSQL persistence
- SQLx integration
- Transaction lifecycle tracking
- Retry/error persistence

---

# High-Level Architecture

```text
Client
  ↓
API Layer (Axum)
  ↓
Queue Engine
  ↓
Worker Runtime
  ↓
Executor Layer
  ↓
RPC Manager
  ↓
Solana RPC
