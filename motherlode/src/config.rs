use anyhow::Result;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub jwt_expiry_hours: i64,
    pub refresh_token_expiry_days: i64,
    pub app_env: String,
    pub app_port: u16,
    pub app_host: String,
    pub payment_provider: String,
    pub ukheshe_base_url: String,
    pub ukheshe_client_id: String,
    pub ukheshe_client_secret: String,
    pub ukheshe_webhook_secret: String,
    pub internal_payments_enabled: bool,
    pub kyc_provider_url: String,
    pub kyc_api_key: String,
    pub sim_swap_api_url: String,
    pub sim_swap_api_key: String,
    pub fcm_server_key: String,
    pub sendgrid_api_key: String,
    pub whatsapp_api_url: String,
    pub whatsapp_token: String,
    pub tax_free_interest_limit: f64,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            database_url: std::env::var("DATABASE_URL")?,
            redis_url: std::env::var("REDIS_URL")?,
            jwt_secret: std::env::var("JWT_SECRET")?,
            jwt_expiry_hours: std::env::var("JWT_EXPIRY_HOURS").unwrap_or("24".into()).parse()?,
            refresh_token_expiry_days: std::env::var("REFRESH_TOKEN_EXPIRY_DAYS").unwrap_or("30".into()).parse()?,
            app_env: std::env::var("APP_ENV").unwrap_or("development".into()),
            app_port: std::env::var("APP_PORT").unwrap_or("8080".into()).parse()?,
            app_host: std::env::var("APP_HOST").unwrap_or("0.0.0.0".into()),
            payment_provider: std::env::var("PAYMENT_PROVIDER").unwrap_or("ukheshe".into()),
            ukheshe_base_url: std::env::var("UKHESHE_BASE_URL").unwrap_or_default(),
            ukheshe_client_id: std::env::var("UKHESHE_CLIENT_ID").unwrap_or_default(),
            ukheshe_client_secret: std::env::var("UKHESHE_CLIENT_SECRET").unwrap_or_default(),
            ukheshe_webhook_secret: std::env::var("UKHESHE_WEBHOOK_SECRET").unwrap_or_default(),
            internal_payments_enabled: std::env::var("INTERNAL_PAYMENTS_ENABLED").unwrap_or("false".into()).parse()?,
            kyc_provider_url: std::env::var("KYC_PROVIDER_URL").unwrap_or_default(),
            kyc_api_key: std::env::var("KYC_API_KEY").unwrap_or_default(),
            sim_swap_api_url: std::env::var("SIM_SWAP_API_URL").unwrap_or_default(),
            sim_swap_api_key: std::env::var("SIM_SWAP_API_KEY").unwrap_or_default(),
            fcm_server_key: std::env::var("FCM_SERVER_KEY").unwrap_or_default(),
            sendgrid_api_key: std::env::var("SENDGRID_API_KEY").unwrap_or_default(),
            whatsapp_api_url: std::env::var("WHATSAPP_API_URL").unwrap_or_default(),
            whatsapp_token: std::env::var("WHATSAPP_TOKEN").unwrap_or_default(),
            tax_free_interest_limit: std::env::var("TAX_FREE_INTEREST_LIMIT").unwrap_or("23800".into()).parse()?,
        })
    }

    pub fn is_production(&self) -> bool {
        self.app_env == "production"
    }
}
