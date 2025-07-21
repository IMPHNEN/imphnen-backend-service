use crate::decode_access_token;
use axum::http::{HeaderMap, header::AUTHORIZATION};

pub fn extract_email(headers: &HeaderMap) -> Option<String> {
	let auth_header = headers.get(AUTHORIZATION)?.to_str().ok()?;
	let token = auth_header.strip_prefix("Bearer ")?;

	match decode_access_token(token) {
		Ok(data) => Some(data.claims.sub),
		Err(_e) => None,
	}
}

pub fn extract_email_token(token: String) -> Option<String> {
	let token_data = decode_access_token(&token).ok()?;
	Some(token_data.claims.sub)
}
