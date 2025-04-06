use super::Env;
use axum::http::StatusCode;
use chrono::{Duration, TimeDelta, Utc};
use jsonwebtoken::{
	decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
	pub exp: usize,
	pub iat: usize,
	pub sub: String,
}

pub fn encode_access_token(sub: String) -> Result<String, StatusCode> {
	let env = Env::new();
	let secret: String = env.access_token_secret;
	let now = Utc::now();
	let expire: TimeDelta = Duration::minutes(15);
	let exp: usize = (now + expire).timestamp() as usize;
	let iat: usize = now.timestamp() as usize;
	let claim = Claims { iat, exp, sub };
	encode(
		&Header::default(),
		&claim,
		&EncodingKey::from_secret(secret.as_ref()),
	)
	.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn encode_reset_password_token(sub: String) -> Result<String, StatusCode> {
	let env = Env::new();
	let secret: String = env.access_token_secret;
	let now = Utc::now();
	let expire: TimeDelta = Duration::minutes(5);
	let exp: usize = (now + expire).timestamp() as usize;
	let iat: usize = now.timestamp() as usize;
	let claim = Claims { iat, exp, sub };
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
	let env = Env::new();
	let secret: String = env.access_token_secret;
	let result: Result<TokenData<Claims>, StatusCode> = decode(
		&jwt_token,
		&DecodingKey::from_secret(secret.as_ref()),
		&Validation::default(),
	)
	.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
	result
}

pub fn encode_refresh_token(sub: String) -> Result<String, StatusCode> {
	let env = Env::new();
	let secret: String = env.refresh_token_secret;
	let now = Utc::now();
	let expire: TimeDelta = Duration::days(1);
	let exp: usize = (now + expire).timestamp() as usize;
	let iat: usize = now.timestamp() as usize;
	let claim = Claims { iat, exp, sub };
	encode(
		&Header::default(),
		&claim,
		&EncodingKey::from_secret(secret.as_ref()),
	)
	.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn decode_refresh_token(
	jwt_token: &str,
) -> Result<TokenData<Claims>, StatusCode> {
	let env = Env::new();
	let secret: String = env.refresh_token_secret;
	let result: Result<TokenData<Claims>, StatusCode> = decode(
		&jwt_token,
		&DecodingKey::from_secret(secret.as_ref()),
		&Validation::default(),
	)
	.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
	result
}
