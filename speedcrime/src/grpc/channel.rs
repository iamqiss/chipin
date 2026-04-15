//! gRPC channel management.
//! Handles connection, reconnection, and token injection.

use std::sync::Arc;
use tokio::sync::RwLock;
use tonic::transport::{Channel, Endpoint};
use tonic::metadata::MetadataValue;
use tonic::service::interceptor::InterceptedService;
use tonic::{Request, Status};

use crate::state::auth::AuthState;

/// The gRPC endpoint — motherlode's native port.
/// Override with MOTHERLODE_GRPC_URL env var.
pub fn motherlode_endpoint() -> String {
    std::env::var("MOTHERLODE_GRPC_URL")
        .unwrap_or_else(|_| "http://localhost:50051".to_string())
}

/// Build a channel to motherlode.
pub async fn connect() -> anyhow::Result<Channel> {
    let endpoint = Endpoint::from_shared(motherlode_endpoint())?
        .tcp_keepalive(Some(std::time::Duration::from_secs(30)))
        .connect_timeout(std::time::Duration::from_secs(10))
        .timeout(std::time::Duration::from_secs(30));

    Ok(endpoint.connect().await?)
}

/// Build a channel with lazy (on-demand) connection.
pub fn connect_lazy() -> anyhow::Result<Channel> {
    let endpoint = Endpoint::from_shared(motherlode_endpoint())?
        .tcp_keepalive(Some(std::time::Duration::from_secs(30)));

    Ok(endpoint.connect_lazy())
}

/// Interceptor that injects JWT access token into every request.
#[derive(Clone)]
pub struct AuthInterceptor {
    pub auth: Arc<RwLock<AuthState>>,
}

impl tonic::service::Interceptor for AuthInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        // Try to get token synchronously from a cached copy
        // Real implementation: use try_read() and cache token on login
        // For now: placeholder — token injection handled per-call in clients
        Ok(request)
    }
}

/// Inject a bearer token into request metadata.
pub fn with_token<T>(mut req: Request<T>, token: &str) -> Request<T> {
    if let Ok(val) = MetadataValue::try_from(format!("Bearer {}", token)) {
        req.metadata_mut().insert("authorization", val);
    }
    req
}
