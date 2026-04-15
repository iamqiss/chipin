//! gRPC client module for speedcrime.
//! Connects to motherlode on port 50051 (native) or 50052 (grpc-web/WASM).

pub mod proto;
pub mod channel;
pub mod clients;
