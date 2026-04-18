use bigfinance::{config::Config, server};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_max_level(tracing::Level::INFO)
        .init();

    let config = Config::from_env()?;

    tracing::info!(
        "bigfinance starting on {}:{}",
        config.host, config.port
    );

    server::run(config).await
}
