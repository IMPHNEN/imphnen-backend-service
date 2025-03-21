use crate::decode_access_token;
use axum::http::{header::AUTHORIZATION, HeaderMap};

pub fn extract_email(headers: &HeaderMap) -> Option<String> {
	let auth_header = headers.get(AUTHORIZATION)?.to_str().ok()?;
	let token = auth_header.strip_prefix("Bearer ")?;
	let token_data = decode_access_token(token).ok()?;
	Some(token_data.claims.sub)
}
