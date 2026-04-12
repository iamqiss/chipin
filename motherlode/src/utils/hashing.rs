//! Argon2id password and PIN hashing.
//! Used for both account passwords and 4-digit feature lock PINs.

use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2,
};
use crate::errors::{AppError, AppResult};

/// Hash a password or PIN using Argon2id.
pub fn hash(value: &str) -> AppResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(value.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Hashing failed: {}", e)))
}

/// Verify a password or PIN against its stored hash.
pub fn verify(value: &str, hash: &str) -> AppResult<bool> {
    let parsed = PasswordHash::new(hash)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Invalid hash: {}", e)))?;
    Ok(Argon2::default()
        .verify_password(value.as_bytes(), &parsed)
        .is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify_password() {
        let password = "SecurePass123!";
        let hashed = hash(password).unwrap();
        assert!(verify(password, &hashed).unwrap());
        assert!(!verify("wrongpassword", &hashed).unwrap());
    }

    #[test]
    fn test_hash_and_verify_pin() {
        let pin = "4821";
        let hashed = hash(pin).unwrap();
        assert!(verify(pin, &hashed).unwrap());
        assert!(!verify("0000", &hashed).unwrap());
    }

    #[test]
    fn test_unique_hashes() {
        // Same value should produce different hashes (different salts)
        let password = "SamePassword1!";
        let hash1 = hash(password).unwrap();
        let hash2 = hash(password).unwrap();
        assert_ne!(hash1, hash2);
        // But both should verify correctly
        assert!(verify(password, &hash1).unwrap());
        assert!(verify(password, &hash2).unwrap());
    }
}

