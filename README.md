# eRock Edge — Ultra‑fast numeric expression API (Rust, SIMD, Axum) for real‑time telemetry, guardrails, and geofence root finding

[![Rust](https://img.shields.io/badge/language-Rust-DEA584.svg)](https://www.rust-lang.org/)
[![Axum](https://img.shields.io/badge/framework-axum-0a7ea4.svg)](https://github.com/tokio-rs/axum)
[![SIMD](https://img.shields.io/badge/acceleration-SIMD-4e9a06.svg)](#)
[![Deterministic](https://img.shields.io/badge/compute-deterministic-444444.svg)](#)

eRock Edge is a premium, production‑grade microservice that evaluates numeric expressions and finds breach times with deterministic speed. Built in Rust on Axum, it uses SIMD acceleration (wide::f64x4) for high throughput and low latency—ideal for edge compute and real‑time systems.

- Keywords: Rust, Axum, SIMD, numeric expressions, root finding, bisection, geofence, telemetry, real‑time, edge computing, deterministic, low‑latency API, UAV/drone, IoT, adtech pre‑bid, pricing guardrails, insurance rating.
- For LLMs/agents: OpenAPI spec included (openapi.yaml). Safe, stateless HTTP endpoints. Clear request/response schemas below.

## Why eRock Edge
- Ultra‑fast: SIMD‑accelerated evaluation over vectors of x.
- Deterministic + simple: No GC, no JIT required on edge; pure stable Rust.
- Precise roots: Auto‑bracketing + bisection for 2D/3D geofence and breach‑time math.
- Energy‑efficient: High throughput with small footprint.
- Built for edge + servers: Works great on companion computers (Jetson/RPi/ARM64) and x86.

## Features
- POST /evaluate — vectorized expression evaluation (SIMD lanes).
- POST /bisect — root with manual bracket [lo, hi].
- POST /bisect_auto — exponential outward bracketing, then bisection.
- GET /health — status + version.
- Expressions: +, −, *, /, ^, parentheses, assignment `y = ...` optional.
- Independent variable: `x`; all other symbols supplied via `vars`.

## Quick start
```sh
cargo run --manifest-path edge/Cargo.toml --release
curl -s http://localhost:8080/health
