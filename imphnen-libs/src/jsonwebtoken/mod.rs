use crate::environment::ENV;
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

const ACCESS_TOKEN_DURATION_MINUTES: i64 = 15;
const REFRESH_TOKEN_DURATION_DAYS: i64 = 1;
const RESET_TOKEN_DURATION_MINUTES: i64 = 5;

static ACCESS_HEADER: once_cell::sync::Lazy<Header> =
	once_cell::sync::Lazy::new(Header::default);
static ACCESS_KEY: once_cell::sync::Lazy<EncodingKey> =
	once_cell::sync::Lazy::new(|| {
		EncodingKey::from_secret(ENV.access_token_secret.as_ref())
	});

static REFRESH_HEADER: once_cell::sync::Lazy<Header> =
	once_cell::sync::Lazy::new(Header::default);
static REFRESH_KEY: once_cell::sync::Lazy<EncodingKey> =
	once_cell::sync::Lazy::new(|| {
		EncodingKey::from_secret(ENV.refresh_token_secret.as_ref())
	});

fn create_claims(sub: String, user_id: String, duration: TimeDelta) -> Claims {
	let now = Utc::now();
	let exp: usize = (now + duration).timestamp() as usize;
	let iat: usize = now.timestamp() as usize;
	Claims {
		iat,
		exp,
		sub,
		user_id,
	}
}

fn encode_token(
	claims: &Claims,
	header: &Header,
	key: &EncodingKey,
) -> Result<String, StatusCode> {
	encode(header, claims, key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

fn decode_token(token: &str, secret: &str) -> Result<TokenData<Claims>, StatusCode> {
	decode(
		token,
		&DecodingKey::from_secret(secret.as_ref()),
		&Validation::default(),
	)
	.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn encode_access_token(
	sub: String,
	user_id: String,
) -> Result<String, StatusCode> {
	let claims = create_claims(
		sub,
		user_id,
		Duration::minutes(ACCESS_TOKEN_DURATION_MINUTES),
	);
	encode_token(&claims, &ACCESS_HEADER, &ACCESS_KEY)
}

pub fn encode_refresh_token(
	sub: String,
	user_id: String,
) -> Result<String, StatusCode> {
	let claims =
		create_claims(sub, user_id, Duration::days(REFRESH_TOKEN_DURATION_DAYS));
	encode_token(&claims, &REFRESH_HEADER, &REFRESH_KEY)
}

pub fn encode_reset_password_token(
	sub: String,
	user_id: String,
) -> Result<String, StatusCode> {
	let claims = create_claims(
		sub,
		user_id,
		Duration::minutes(RESET_TOKEN_DURATION_MINUTES),
	);
	let key = EncodingKey::from_secret(ENV.access_token_secret.as_ref());
	encode_token(&claims, &Header::default(), &key)
}

pub fn decode_access_token(
	jwt_token: &str,
) -> Result<TokenData<Claims>, StatusCode> {
	decode_token(jwt_token, &ENV.access_token_secret)
}

pub fn decode_refresh_token(
	jwt_token: &str,
) -> Result<TokenData<Claims>, StatusCode> {
	decode_token(jwt_token, &ENV.refresh_token_secret)
}

pub fn generate_jwt(user_id: &str) -> Result<String, StatusCode> {
	encode_access_token(user_id.to_string(), user_id.to_string())
}
