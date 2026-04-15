//! gRPC server builder.
//! Called from main.rs alongside the existing Axum HTTP server.

use std::net::SocketAddr;
use tonic::transport::Server;
use tonic_web::GrpcWebLayer;
use tower_http::cors::CorsLayer;

use crate::config::Config;
use crate::grpc::interceptors::auth_interceptor;
use crate::grpc::services::{
    auth::AuthServiceImpl,
    stokvels::StokvelServiceImpl,
    contributions::ContributionServiceImpl,
    wallet::WalletServiceImpl,
    market::MarketServiceImpl,
    fair_score::FairScoreServiceImpl,
    messages::MessageServiceImpl,
    fraud::FraudServiceImpl,
};
use crate::grpc::proto::{
    auth::auth_service_server::AuthServiceServer,
    stokvels::stokvel_service_server::StokvelServiceServer,
    contributions::contribution_service_server::ContributionServiceServer,
    wallet::wallet_service_server::WalletServiceServer,
    market::market_service_server::MarketServiceServer,
    fair_score::fair_score_service_server::FairScoreServiceServer,
    messages::message_service_server::MessageServiceServer,
    fraud::fraud_service_server::FraudServiceServer,
};

pub struct GrpcServers {
    pub db:     sqlx::PgPool,
    pub redis:  redis::aio::ConnectionManager,
    pub config: Config,
}

impl GrpcServers {
    /// Start the native gRPC server on port 50051.
    pub async fn serve_native(self) -> anyhow::Result<()> {
        let addr: SocketAddr = format!("{}:50051", self.config.app_host).parse()?;

        tracing::info!("motherlode gRPC listening on {}", addr);

        let auth_interceptor = auth_interceptor(self.config.clone());

        Server::builder()
            .layer(tower::ServiceBuilder::new()
                .layer(tonic::service::interceptor(crate::grpc::interceptors::logging_interceptor))
            )
            // Auth service — no auth required (handles its own auth)
            .add_service(AuthServiceServer::new(
                AuthServiceImpl::new(self.db.clone(), self.redis.clone(), self.config.clone())
            ))
            // All other services — JWT auth required
            .add_service(StokvelServiceServer::with_interceptor(
                StokvelServiceImpl::new(self.db.clone(), self.redis.clone()),
                auth_interceptor.clone(),
            ))
            .add_service(ContributionServiceServer::with_interceptor(
                ContributionServiceImpl::new(self.db.clone(), self.redis.clone()),
                auth_interceptor.clone(),
            ))
            .add_service(WalletServiceServer::with_interceptor(
                WalletServiceImpl::new(self.db.clone(), self.redis.clone(), self.config.clone()),
                auth_interceptor.clone(),
            ))
            .add_service(MarketServiceServer::with_interceptor(
                MarketServiceImpl::new(self.db.clone()),
                auth_interceptor.clone(),
            ))
            .add_service(FairScoreServiceServer::with_interceptor(
                FairScoreServiceImpl::new(self.db.clone()),
                auth_interceptor.clone(),
            ))
            .add_service(MessageServiceServer::with_interceptor(
                MessageServiceImpl::new(self.db.clone(), self.redis.clone()),
                auth_interceptor.clone(),
            ))
            .add_service(FraudServiceServer::with_interceptor(
                FraudServiceImpl::new(self.db.clone()),
                auth_interceptor.clone(),
            ))
            .serve(addr)
            .await?;

        Ok(())
    }

    /// Start the gRPC-Web server on port 50052.
    /// Wraps the same services with GrpcWebLayer + CORS for browser/WASM clients.
    pub async fn serve_web(self) -> anyhow::Result<()> {
        let addr: SocketAddr = format!("{}:50052", self.config.app_host).parse()?;

        tracing::info!("motherlode gRPC-Web listening on {}", addr);

        let auth_interceptor = auth_interceptor(self.config.clone());

        Server::builder()
            .accept_http1(true) // required for gRPC-Web
            .layer(GrpcWebLayer::new())
            .layer(CorsLayer::permissive()) // restrict in production
            .add_service(AuthServiceServer::new(
                AuthServiceImpl::new(self.db.clone(), self.redis.clone(), self.config.clone())
            ))
            .add_service(StokvelServiceServer::with_interceptor(
                StokvelServiceImpl::new(self.db.clone(), self.redis.clone()),
                auth_interceptor.clone(),
            ))
            .add_service(WalletServiceServer::with_interceptor(
                WalletServiceImpl::new(self.db.clone(), self.redis.clone(), self.config.clone()),
                auth_interceptor.clone(),
            ))
            .add_service(MarketServiceServer::with_interceptor(
                MarketServiceImpl::new(self.db.clone()),
                auth_interceptor.clone(),
            ))
            .serve(addr)
            .await?;

        Ok(())
    }
}
