//! gRPC module — tonic server wiring.
//!
//! Motherlode runs two servers:
//!   :8080 — existing REST/HTTP (Axum) for backwards compat + health checks
//!   :50051 — gRPC (tonic) for speedcrime native client
//!   :50052 — gRPC-Web (tonic + grpc-web) for WASM/browser clients

pub mod proto;
pub mod server;
pub mod interceptors;
pub mod services;
