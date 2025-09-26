//! JWT token encoding and decoding utilities.
//!
//! This module provides functions for creating and validating JWT tokens
//! for authentication purposes, including access tokens, refresh tokens,
//! and password reset tokens.

use crate::environment::ENV;
use axum::http::StatusCode;
use chrono::{Duration, TimeDelta, Utc};
use jsonwebtoken::{
    DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode,
};
use serde::{Deserialize, Serialize};

/// JWT claims structure containing token payload information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Expiration timestamp
    pub exp: usize,
    /// Issued at timestamp
    pub iat: usize,
    /// Subject (usually user identifier)
    pub sub: String,
    /// User ID
    pub user_id: String,
}

// Token configuration constants
const ACCESS_TOKEN_DURATION_MINUTES: i64 = 15;
const REFRESH_TOKEN_DURATION_DAYS: i64 = 1;
const RESET_TOKEN_DURATION_MINUTES: i64 = 5;

// Lazy-initialized headers and keys for performance
static ACCESS_HEADER: once_cell::sync::Lazy<Header> = once_cell::sync::Lazy::new(Header::default);
static ACCESS_KEY: once_cell::sync::Lazy<EncodingKey> = once_cell::sync::Lazy::new(|| {
    EncodingKey::from_secret(ENV.access_token_secret.as_ref())
});

static REFRESH_HEADER: once_cell::sync::Lazy<Header> = once_cell::sync::Lazy::new(Header::default);
static REFRESH_KEY: once_cell::sync::Lazy<EncodingKey> = once_cell::sync::Lazy::new(|| {
    EncodingKey::from_secret(ENV.refresh_token_secret.as_ref())
});

/// Create JWT claims with specified expiration duration.
///
/// # Arguments
/// * `sub` - Subject identifier
/// * `user_id` - User ID
/// * `duration` - Token validity duration
///
/// # Returns
/// JWT claims structure
fn create_claims(sub: String, user_id: String, duration: TimeDelta) -> Claims {
    let now = Utc::now();
    let exp: usize = (now + duration).timestamp() as usize;
    let iat: usize = now.timestamp() as usize;
    Claims { iat, exp, sub, user_id }
}

/// Encode a JWT token with the specified header and key.
///
/// # Arguments
/// * `claims` - JWT claims to encode
/// * `header` - JWT header
/// * `key` - Encoding key
///
/// # Returns
/// Encoded JWT token or internal server error status
fn encode_token(claims: &Claims, header: &Header, key: &EncodingKey) -> Result<String, StatusCode> {
    encode(header, claims, key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

/// Decode a JWT token with the specified secret.
///
/// # Arguments
/// * `token` - JWT token string
/// * `secret` - Secret key for decoding
///
/// # Returns
/// Decoded token data or internal server error status
fn decode_token(token: &str, secret: &str) -> Result<TokenData<Claims>, StatusCode> {
    decode(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

/// Encode an access token with 15-minute expiration.
///
/// # Arguments
/// * `sub` - Subject identifier
/// * `user_id` - User ID
///
/// # Returns
/// Encoded JWT access token
pub fn encode_access_token(sub: String, user_id: String) -> Result<String, StatusCode> {
    let claims = create_claims(sub, user_id, Duration::minutes(ACCESS_TOKEN_DURATION_MINUTES));
    encode_token(&claims, &ACCESS_HEADER, &ACCESS_KEY)
}

/// Encode a refresh token with 1-day expiration.
///
/// # Arguments
/// * `sub` - Subject identifier
/// * `user_id` - User ID
///
/// # Returns
/// Encoded JWT refresh token
pub fn encode_refresh_token(sub: String, user_id: String) -> Result<String, StatusCode> {
    let claims = create_claims(sub, user_id, Duration::days(REFRESH_TOKEN_DURATION_DAYS));
    encode_token(&claims, &REFRESH_HEADER, &REFRESH_KEY)
}

/// Encode a password reset token with 5-minute expiration.
///
/// # Arguments
/// * `sub` - Subject identifier
/// * `user_id` - User ID
///
/// # Returns
/// Encoded JWT reset token
pub fn encode_reset_password_token(sub: String, user_id: String) -> Result<String, StatusCode> {
    let claims = create_claims(sub, user_id, Duration::minutes(RESET_TOKEN_DURATION_MINUTES));
    let key = EncodingKey::from_secret(ENV.access_token_secret.as_ref());
    encode_token(&claims, &Header::default(), &key)
}

/// Decode an access token.
///
/// # Arguments
/// * `jwt_token` - JWT token string
///
/// # Returns
/// Decoded token data containing claims
pub fn decode_access_token(jwt_token: &str) -> Result<TokenData<Claims>, StatusCode> {
    decode_token(jwt_token, &ENV.access_token_secret)
}

/// Decode a refresh token.
///
/// # Arguments
/// * `jwt_token` - JWT token string
///
/// # Returns
/// Decoded token data containing claims
pub fn decode_refresh_token(jwt_token: &str) -> Result<TokenData<Claims>, StatusCode> {
    decode_token(jwt_token, &ENV.refresh_token_secret)
}

/// Generate a simple JWT access token using user_id as both sub and user_id.
///
/// # Arguments
/// * `user_id` - User identifier
///
/// # Returns
/// Encoded JWT access token
pub fn generate_jwt(user_id: &str) -> Result<String, StatusCode> {
    encode_access_token(user_id.to_string(), user_id.to_string())
}
