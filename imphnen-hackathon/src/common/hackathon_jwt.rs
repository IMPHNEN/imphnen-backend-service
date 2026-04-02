use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use imphnen_utils::errors::AppError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HackathonClaims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
    pub jti: String,
    #[serde(default)]
    pub token_type: String,
}

#[derive(Clone)]
pub struct HackathonJwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    expiry_hours: i64,
}

impl HackathonJwtService {
    pub fn new(secret: &str, expiry_hours: i64) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            expiry_hours,
        }
    }

    pub fn generate_token(&self, user_id: Uuid) -> Result<String, AppError> {
        self.generate_token_with_type(user_id, "access", self.expiry_hours)
    }

    pub fn generate_refresh_token(&self, user_id: Uuid) -> Result<String, AppError> {
        self.generate_token_with_type(user_id, "refresh", self.expiry_hours * 7)
    }

    fn generate_token_with_type(&self, user_id: Uuid, token_type: &str, expiry_hours: i64) -> Result<String, AppError> {
        let now = Utc::now();
        let claims = HackathonClaims {
            sub: user_id.to_string(),
            exp: (now + Duration::hours(expiry_hours)).timestamp(),
            iat: now.timestamp(),
            jti: Uuid::new_v4().to_string(),
            token_type: token_type.to_string(),
        };
        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    pub fn verify_token(&self, token: &str) -> Result<HackathonClaims, AppError> {
        decode::<HackathonClaims>(token, &self.decoding_key, &Validation::default())
            .map(|d| d.claims)
            .map_err(|_| AppError::AuthenticationError("Invalid or expired token".to_string()))
    }
}
