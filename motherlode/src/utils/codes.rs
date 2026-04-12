//! OTP and collection code generation.
//!
//! OTPs:
//!   - 6-digit numeric
//!   - Stored in Redis with TTL
//!   - Rate limited: max 3 requests per phone per 10 minutes
//!   - Single-use: deleted from Redis on successful verification
//!   - Delivered via SMS and WhatsApp
//!
//! Collection codes:
//!   - Unique alphanumeric (no ambiguous chars)
//!   - Used for Market bulk order pickups

use rand::Rng;
use redis::aio::ConnectionManager;
use crate::errors::{AppError, AppResult};

// ── Constants ─────────────────────────────────────────────────────────────────

const OTP_LENGTH: usize = 6;
const OTP_TTL_SECONDS: u64 = 300;        // 5 minutes
const OTP_RATE_WINDOW_SECONDS: u64 = 600; // 10 minutes
const OTP_RATE_MAX_REQUESTS: u32 = 3;
const PHONE_VERIFIED_TTL_SECONDS: u64 = 600; // 10 min window to complete registration
const OTP_TOKEN_TTL_SECONDS: u64 = 600;  // 10 min token issued after OTP verify (for password reset)
const COLLECTION_CODE_LENGTH: usize = 8;

// ── OTP Purposes ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum OtpPurpose {
    Register,
    ResetPassword,
    Withdraw,
}

impl OtpPurpose {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Register => "register",
            Self::ResetPassword => "reset_password",
            Self::Withdraw => "withdraw",
        }
    }
}

// ── OTP Generation ────────────────────────────────────────────────────────────

/// Generate a 6-digit OTP and store it in Redis.
/// Returns the OTP string — caller is responsible for delivering it via SMS/WhatsApp.
///
/// Key format: `otp:{purpose}:{phone}`
pub async fn generate_and_store_otp(
    redis: &mut ConnectionManager,
    phone: &str,
    purpose: &OtpPurpose,
) -> AppResult<String> {
    let otp: String = (0..OTP_LENGTH)
        .map(|_| rand::thread_rng().gen_range(0..10).to_string())
        .collect();

    let key = otp_key(phone, purpose.as_str());

    redis::cmd("SETEX")
        .arg(&key)
        .arg(OTP_TTL_SECONDS)
        .arg(&otp)
        .query_async::<_, ()>(redis)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis OTP store failed: {}", e)))?;

    tracing::info!(
        phone = %phone,
        purpose = %purpose.as_str(),
        "OTP generated and stored"
    );

    Ok(otp)
}

// ── OTP Verification ──────────────────────────────────────────────────────────

/// Verify an OTP. Deletes it on success (single-use).
/// Returns true if valid, false if wrong or expired.
pub async fn verify_otp(
    redis: &mut ConnectionManager,
    phone: &str,
    purpose: &OtpPurpose,
    submitted: &str,
) -> AppResult<bool> {
    let key = otp_key(phone, purpose.as_str());

    let stored: Option<String> = redis::cmd("GET")
        .arg(&key)
        .query_async(redis)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis OTP get failed: {}", e)))?;

    match stored {
        None => {
            tracing::warn!(phone = %phone, purpose = %purpose.as_str(), "OTP not found or expired");
            Ok(false)
        }
        Some(value) => {
            if value == submitted {
                // Delete immediately — single use
                redis::cmd("DEL")
                    .arg(&key)
                    .query_async::<_, ()>(redis)
                    .await
                    .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis OTP delete failed: {}", e)))?;

                tracing::info!(phone = %phone, purpose = %purpose.as_str(), "OTP verified successfully");
                Ok(true)
            } else {
                tracing::warn!(phone = %phone, purpose = %purpose.as_str(), "OTP mismatch");
                Ok(false)
            }
        }
    }
}

// ── Phone Verification State ──────────────────────────────────────────────────

/// Mark a phone as OTP-verified.
/// Gives a 10-minute window to complete the registration flow.
/// Key format: `phone_verified:{phone}`
pub async fn mark_phone_verified(
    redis: &mut ConnectionManager,
    phone: &str,
) -> AppResult<()> {
    redis::cmd("SETEX")
        .arg(format!("phone_verified:{}", phone))
        .arg(PHONE_VERIFIED_TTL_SECONDS)
        .arg("1")
        .query_async::<_, ()>(redis)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis phone verify mark failed: {}", e)))?;
    Ok(())
}

/// Check if a phone has been OTP-verified within the window.
pub async fn is_phone_verified(
    redis: &mut ConnectionManager,
    phone: &str,
) -> AppResult<bool> {
    let result: Option<String> = redis::cmd("GET")
        .arg(format!("phone_verified:{}", phone))
        .query_async(redis)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis phone verified check failed: {}", e)))?;
    Ok(result.is_some())
}

/// Clear the phone verified marker — called after registration completes.
pub async fn clear_phone_verified(
    redis: &mut ConnectionManager,
    phone: &str,
) -> AppResult<()> {
    redis::cmd("DEL")
        .arg(format!("phone_verified:{}", phone))
        .query_async::<_, ()>(redis)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis phone verified clear failed: {}", e)))?;
    Ok(())
}

// ── OTP Token (post-verification short-lived token) ───────────────────────────

/// After OTP is verified for password reset, issue a short-lived token
/// that authorises the actual password reset endpoint.
/// This prevents the reset endpoint from being called without OTP verification.
///
/// Key format: `otp_token:{token}:{phone}`
pub async fn issue_otp_token(
    redis: &mut ConnectionManager,
    phone: &str,
) -> AppResult<String> {
    let token = generate_secure_token();
    let key = format!("otp_token:{}:{}", token, phone);

    redis::cmd("SETEX")
        .arg(&key)
        .arg(OTP_TOKEN_TTL_SECONDS)
        .arg(phone)
        .query_async::<_, ()>(redis)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis OTP token store failed: {}", e)))?;

    Ok(token)
}

/// Validate and consume an OTP token for password reset.
/// Returns the phone number it was issued for, or None if invalid/expired.
pub async fn consume_otp_token(
    redis: &mut ConnectionManager,
    token: &str,
    phone: &str,
) -> AppResult<bool> {
    let key = format!("otp_token:{}:{}", token, phone);

    let stored_phone: Option<String> = redis::cmd("GET")
        .arg(&key)
        .query_async(redis)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis OTP token get failed: {}", e)))?;

    match stored_phone {
        None => Ok(false),
        Some(stored) => {
            if stored == phone {
                // Consume — single use
                redis::cmd("DEL")
                    .arg(&key)
                    .query_async::<_, ()>(redis)
                    .await
                    .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis OTP token delete failed: {}", e)))?;
                Ok(true)
            } else {
                Ok(false)
            }
        }
    }
}

// ── Rate Limiting ─────────────────────────────────────────────────────────────

/// Check and increment OTP request rate limit.
/// Returns Ok(true) if within limit, Ok(false) if exceeded.
/// Max 3 OTP requests per phone per 10 minutes per purpose.
///
/// Key format: `otp_rate:{purpose}:{phone}`
pub async fn check_otp_rate_limit(
    redis: &mut ConnectionManager,
    phone: &str,
    purpose: &OtpPurpose,
) -> AppResult<bool> {
    let key = format!("otp_rate:{}:{}", purpose.as_str(), phone);

    let count: u32 = redis::cmd("INCR")
        .arg(&key)
        .query_async(redis)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis rate limit incr failed: {}", e)))?;

    if count == 1 {
        // First request in window — set TTL
        redis::cmd("EXPIRE")
            .arg(&key)
            .arg(OTP_RATE_WINDOW_SECONDS)
            .query_async::<_, ()>(redis)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis rate limit expire failed: {}", e)))?;
    }

    if count > OTP_RATE_MAX_REQUESTS {
        tracing::warn!(
            phone = %phone,
            purpose = %purpose.as_str(),
            count = %count,
            "OTP rate limit exceeded"
        );
        return Ok(false);
    }

    Ok(true)
}

/// How many seconds until the rate limit window resets.
pub async fn otp_rate_limit_ttl(
    redis: &mut ConnectionManager,
    phone: &str,
    purpose: &OtpPurpose,
) -> AppResult<i64> {
    let key = format!("otp_rate:{}:{}", purpose.as_str(), phone);
    let ttl: i64 = redis::cmd("TTL")
        .arg(&key)
        .query_async(redis)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis TTL check failed: {}", e)))?;
    Ok(ttl.max(0))
}

// ── Collection Codes ──────────────────────────────────────────────────────────

/// Generate a unique alphanumeric collection code for Market order pickups.
/// Excludes ambiguous characters: 0, O, 1, I
pub fn generate_collection_code() -> String {
    let charset = "ABCDEFGHJKLMNPQRSTUVWXYZ23456789";
    let chars: Vec<char> = charset.chars().collect();
    let mut rng = rand::thread_rng();
    (0..COLLECTION_CODE_LENGTH)
        .map(|_| chars[rng.gen_range(0..chars.len())])
        .collect()
}

// ── Internal helpers ──────────────────────────────────────────────────────────

fn otp_key(phone: &str, purpose: &str) -> String {
    format!("otp:{}:{}", purpose, phone)
}

/// Generate a cryptographically random URL-safe token string.
fn generate_secure_token() -> String {
    let bytes: Vec<u8> = (0..32)
        .map(|_| rand::thread_rng().gen::<u8>())
        .collect();
    hex::encode(bytes)
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collection_code_length() {
        let code = generate_collection_code();
        assert_eq!(code.len(), COLLECTION_CODE_LENGTH);
    }

    #[test]
    fn test_collection_code_no_ambiguous_chars() {
        for _ in 0..100 {
            let code = generate_collection_code();
            assert!(!code.contains('0'));
            assert!(!code.contains('O'));
            assert!(!code.contains('1'));
            assert!(!code.contains('I'));
        }
    }

    #[test]
    fn test_collection_codes_unique() {
        let codes: std::collections::HashSet<String> =
            (0..100).map(|_| generate_collection_code()).collect();
        // All 100 should be unique
        assert_eq!(codes.len(), 100);
    }

    #[test]
    fn test_otp_key_format() {
        let key = otp_key("+27821234567", "register");
        assert_eq!(key, "otp:register:+27821234567");
    }

    #[test]
    fn test_secure_token_length() {
        let token = generate_secure_token();
        assert_eq!(token.len(), 64); // 32 bytes hex encoded
    }
}
