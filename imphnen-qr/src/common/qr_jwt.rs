use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use imphnen_utils::errors::AppError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QrClaims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}

#[derive(Clone)]
pub struct QrJwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    expiry_minutes: i64,
    refresh_expiry_days: i64,
}

impl QrJwtService {
    pub fn new(secret: &str, expiry_minutes: i64, refresh_expiry_days: i64) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            expiry_minutes,
            refresh_expiry_days,
        }
    }

    pub fn generate_token(&self, user_id: Uuid, role: &str) -> Result<String, AppError> {
        let exp = (Utc::now() + Duration::minutes(self.expiry_minutes)).timestamp() as usize;
        let claims = QrClaims {
            sub: user_id.to_string(),
            role: role.to_string(),
            exp,
        };
        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    pub fn generate_refresh_token(&self, user_id: Uuid, role: &str) -> Result<String, AppError> {
        let exp = (Utc::now() + Duration::days(self.refresh_expiry_days)).timestamp() as usize;
        let claims = QrClaims {
            sub: user_id.to_string(),
            role: role.to_string(),
            exp,
        };
        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    pub fn verify_token(&self, token: &str) -> Result<QrClaims, AppError> {
        decode::<QrClaims>(token, &self.decoding_key, &Validation::default())
            .map(|d| d.claims)
            .map_err(|_| AppError::AuthenticationError("Invalid or expired token".to_string()))
    }
}
