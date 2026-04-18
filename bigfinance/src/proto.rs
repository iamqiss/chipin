//! bigfinance gRPC proto types.
//! bigfinance exposes its own gRPC service to motherlode.
//! motherlode calls bigfinance before writing to DB.

// TODO: define bigfinance.proto in proto/ dir
// For now: motherlode calls bigfinance via its Rust API directly (same process or IPC)
