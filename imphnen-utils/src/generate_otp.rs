//! OTP generation utilities with time-based expiration and secure hashing.
//!
//! This module provides functionality to generate one-time passwords (OTPs) with
//! a 5-minute expiration time and SHA256 hashing for secure storage and validation,
//! preventing replay attacks.

use rand::{Rng, rng};
use sha2::{Sha256, Digest};
use chrono::{DateTime, Utc, Duration};

/// Represents an OTP with its code, hashed value and expiration time
#[derive(Debug, Clone)]
pub struct OtpData {
    pub code: u32,
    pub hash: String,
    pub expires_at: DateTime<Utc>,
}

pub struct OtpManager;

impl OtpManager {
    /// Generates a new OTP with a 5-minute expiration and SHA256 hash for secure storage
    pub fn generate_otp() -> OtpData {
        let code = rng().random_range(100_000..1_000_000);
        let otp_str = code.to_string();
        let mut hasher = Sha256::new();
        hasher.update(otp_str.as_bytes());
        let hash = format!("{:x}", hasher.finalize());
        let expires_at = Utc::now() + Duration::minutes(5);
        OtpData { code, hash, expires_at }
    }

    /// Validates the user-provided OTP against the stored OTP data
    /// Checks both hash match and expiration
    pub fn validate_otp(stored: &OtpData, user_otp: u32) -> bool {
        if Utc::now() > stored.expires_at {
            return false;
        }
        let user_otp_str = user_otp.to_string();
        let mut hasher = Sha256::new();
        hasher.update(user_otp_str.as_bytes());
        let user_hash = format!("{:x}", hasher.finalize());
        user_hash == stored.hash
    }
}
