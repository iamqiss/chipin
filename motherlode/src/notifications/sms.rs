use crate::{config::Config, errors::AppResult};

pub async fn send(phone: &str, message: &str, config: &Config) -> AppResult<()> {
    // TODO: implement Vonage/Infobip SMS delivery
    tracing::info!(phone = %phone, "SMS stub: {}", message);
    Ok(())
}
