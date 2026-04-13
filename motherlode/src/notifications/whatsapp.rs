use crate::{config::Config, errors::AppResult};

pub async fn send(phone: &str, message: &str, config: &Config) -> AppResult<()> {
    // TODO: implement WhatsApp Business API delivery
    tracing::info!(phone = %phone, "WhatsApp stub: {}", message);
    Ok(())
}
