# StockFair Protocol Buffers

This directory is the single source of truth for all motherlode ↔ speedcrime communication.

## Services

| Proto                  | Port  | Description                          |
|------------------------|-------|--------------------------------------|
| auth.proto             | 50051 | Registration, OTP, sign in/out       |
| stokvels.proto         | 50051 | Group management, multi-sig, streaming |
| contributions.proto    | 50051 | Contributions, payouts, auto-pay     |
| wallet.proto           | 50051 | Deposits, withdrawals, history       |
| market.proto           | 50051 | Bulk deals, group orders, votes      |
| fair_score.proto       | 50051 | Trust score, history, platform stats |
| messages.proto         | 50051 | Group chat, voice notes, order votes |
| fraud.proto            | 50051 | Fraud Shield, feature lock           |

## Native vs gRPC-Web

- **speedcrime (native)** → port 50051 (tonic transport, HTTP/2)
- **WASM/browser clients** → port 50052 (tonic-web, HTTP/1.1 + CORS)

## Adding a new RPC

1. Add the message + rpc to the relevant .proto file
2. Run `cargo build` in motherlode (tonic_build regenerates server stubs)
3. Run `cargo build` in speedcrime (tonic_build regenerates client stubs)
4. Implement the method in `motherlode/src/grpc/services/{service}.rs`
5. Add the client method in `speedcrime/src/grpc/clients/{service}.rs`

## Why Protobuf?

StockFair handles real money in real-time. Protobuf gives us:
- **Type safety** across the entire stack (Rust → Rust)
- **~10x smaller** payloads than JSON (critical for mobile)
- **Streaming** for real-time contribution updates and group chat
- **Backward compatibility** via field numbers
- **Mathematical impossibility** of type mismatches at the transport layer
