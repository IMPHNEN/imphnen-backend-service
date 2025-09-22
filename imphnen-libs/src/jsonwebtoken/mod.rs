use crate::enviroment::ENV;
use axum::http::StatusCode;
use chrono::{Duration, TimeDelta, Utc};
use jsonwebtoken::{
	DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
	pub exp: usize,
	pub iat: usize,
	pub sub: String,
    pub user_id: String,
}

static ACCESS_HEADER: once_cell::sync::Lazy<Header> = once_cell::sync::Lazy::new(Header::default);
static ACCESS_KEY: once_cell::sync::Lazy<EncodingKey> = once_cell::sync::Lazy::new(|| {
	EncodingKey::from_secret(ENV.access_token_secret.as_ref())
});
pub fn encode_access_token(sub: String, user_id: String) -> Result<String, StatusCode> {
	let now = Utc::now();
	let expire: TimeDelta = Duration::minutes(15);
	let exp: usize = (now + expire).timestamp() as usize;
	let iat: usize = now.timestamp() as usize;
	let claim = Claims { iat, exp, sub, user_id };
	encode(
		&ACCESS_HEADER,
		&claim,
		&ACCESS_KEY,
	)
	.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn encode_reset_password_token(sub: String, user_id: String) -> Result<String, StatusCode> {
	let env = &ENV;
	let secret: String = env.access_token_secret.clone();
	let now = Utc::now();
	let expire: TimeDelta = Duration::minutes(5);
	let exp: usize = (now + expire).timestamp() as usize;
	let iat: usize = now.timestamp() as usize;
	let claim = Claims { iat, exp, sub, user_id };
	encode(
		&Header::default(),
		&claim,
		&EncodingKey::from_secret(secret.as_ref()),
	)
	.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn decode_access_token(
	jwt_token: &str,
) -> Result<TokenData<Claims>, StatusCode> {
	let env = &ENV;
	let secret: String = env.access_token_secret.clone();
	let result: Result<TokenData<Claims>, StatusCode> = decode(
		jwt_token,
		&DecodingKey::from_secret(secret.as_ref()),
		&Validation::default(),
	)
	.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
	result
}

static REFRESH_HEADER: once_cell::sync::Lazy<Header> = once_cell::sync::Lazy::new(Header::default);
static REFRESH_KEY: once_cell::sync::Lazy<EncodingKey> = once_cell::sync::Lazy::new(|| {
	EncodingKey::from_secret(ENV.refresh_token_secret.as_ref())
});
pub fn encode_refresh_token(sub: String, user_id: String) -> Result<String, StatusCode> {
	let now = Utc::now();
	let expire: TimeDelta = Duration::days(1);
	let exp: usize = (now + expire).timestamp() as usize;
	let iat: usize = now.timestamp() as usize;
	let claim = Claims { iat, exp, sub, user_id };
	encode(
		&REFRESH_HEADER,
		&claim,
		&REFRESH_KEY,
	)
	.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn decode_refresh_token(
	jwt_token: &str,
) -> Result<TokenData<Claims>, StatusCode> {
	let env = &ENV;
	let secret: String = env.refresh_token_secret.clone();
	let result: Result<TokenData<Claims>, StatusCode> = decode(
		jwt_token,
		&DecodingKey::from_secret(secret.as_ref()),
		&Validation::default(),
	)
	.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
	result // Explicitly return result
}

pub fn generate_jwt(user_id: &str) -> Result<String, StatusCode> {
    encode_access_token(user_id.to_string(), user_id.to_string())
}
