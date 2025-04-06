use crate::decode_access_token;
use axum::http::{header::AUTHORIZATION, HeaderMap};

pub fn extract_email(headers: &HeaderMap) -> Option<String> {
	println!("📥 Received headers: {:?}", headers);

	let auth_header = headers.get(AUTHORIZATION)?.to_str().ok()?;
	println!("🔍 Authorization Header: {}", auth_header);

	let token = auth_header.strip_prefix("Bearer ")?;
	println!("🧪 Token: {}", token);

	match decode_access_token(token) {
		Ok(data) => {
			println!("✅ Token claims: {:?}", data.claims);
			Some(data.claims.sub)
		}
		Err(e) => {
			eprintln!("❌ Failed to decode token: {}", e);
			None
		}
	}
}

pub fn extract_email_token(token: String) -> Option<String> {
	let token_data = decode_access_token(&token).ok()?;
	Some(token_data.claims.sub)
}
