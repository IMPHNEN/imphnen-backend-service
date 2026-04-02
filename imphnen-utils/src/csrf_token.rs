use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use imphnen_entities::error_dto::error::Error;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::error;

#[derive(Debug, Serialize, Deserialize)]
struct CsrfPayload {
	pub timestamp: u64,
	pub random: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OAuthCsrfPayload {
	pub timestamp: u64,
	pub random: String,
	pub pkce_verifier: String,
}

pub fn generate_csrf_token(secret: &str) -> Result<String, Error> {
	let timestamp = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.map_err(|_| Error::Auth("Failed to get timestamp".to_string()))?
		.as_secs();

	let random = uuid::Uuid::new_v4().to_string();

	let payload = CsrfPayload { timestamp, random };

	let payload_json = serde_json::to_string(&payload).map_err(|e| {
		error!(
			"CSRF Token Generation: Failed to serialize CSRF payload: {:?}",
			e
		);
		Error::Auth("Failed to serialize CSRF payload".to_string())
	})?;

	let payload_b64 = URL_SAFE_NO_PAD.encode(payload_json.as_bytes());

	let mut hasher = Sha256::new();
	hasher.update(payload_b64.as_bytes());
	hasher.update(secret.as_bytes());
	let signature = URL_SAFE_NO_PAD.encode(hasher.finalize());

	Ok(format!("{payload_b64}.{signature}"))
}

pub fn generate_oauth_csrf_token(
	secret: &str,
	pkce_verifier: &str,
) -> Result<String, Error> {
	let timestamp = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.map_err(|_| Error::Auth("Failed to get timestamp".to_string()))?
		.as_secs();

	let random = uuid::Uuid::new_v4().to_string();

	let payload = OAuthCsrfPayload {
		timestamp,
		random,
		pkce_verifier: pkce_verifier.to_string(),
	};

	let payload_json = serde_json::to_string(&payload).map_err(|e| {
		error!(
			"OAuth CSRF Token Generation: Failed to serialize payload: {:?}",
			e
		);
		Error::Auth("Failed to serialize OAuth CSRF payload".to_string())
	})?;

	let payload_b64 = URL_SAFE_NO_PAD.encode(payload_json.as_bytes());

	let mut hasher = Sha256::new();
	hasher.update(payload_b64.as_bytes());
	hasher.update(secret.as_bytes());
	let signature = URL_SAFE_NO_PAD.encode(hasher.finalize());

	Ok(format!("{payload_b64}.{signature}"))
}

pub fn validate_csrf_token(
	token: &str,
	secret: &str,
	max_age_seconds: u64,
) -> Result<(), Error> {
	let parts: Vec<&str> = token.split('.').collect();
	if parts.len() != 2 {
		return Err(Error::Auth("Invalid CSRF token format".to_string()));
	}

	let payload_b64 = parts[0];
	let provided_signature = parts[1];

	let mut hasher = Sha256::new();
	hasher.update(payload_b64.as_bytes());
	hasher.update(secret.as_bytes());
	let expected_signature = URL_SAFE_NO_PAD.encode(hasher.finalize());

	if provided_signature != expected_signature {
		return Err(Error::Auth("Invalid CSRF token signature".to_string()));
	}

	let payload_json = URL_SAFE_NO_PAD
		.decode(payload_b64)
		.map_err(|_| Error::Auth("Failed to decode CSRF token".to_string()))?;

	let payload_str = String::from_utf8(payload_json)
		.map_err(|_| Error::Auth("Invalid CSRF token encoding".to_string()))?;

	let payload: CsrfPayload = serde_json::from_str(&payload_str)
		.map_err(|_| Error::Auth("Failed to parse CSRF token".to_string()))?;

	let now = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.map_err(|_| Error::Auth("Failed to get current timestamp".to_string()))?
		.as_secs();

	if now > payload.timestamp + max_age_seconds {
		return Err(Error::Auth("CSRF token has expired".to_string()));
	}

	if payload.timestamp > now + 60 {
		return Err(Error::Auth(
			"CSRF token timestamp is in the future".to_string(),
		));
	}

	Ok(())
}

pub fn validate_oauth_csrf_token(
	token: &str,
	secret: &str,
	max_age_seconds: u64,
) -> Result<String, Error> {
	let parts: Vec<&str> = token.split('.').collect();
	if parts.len() != 2 {
		return Err(Error::Auth("Invalid OAuth CSRF token format".to_string()));
	}

	let payload_b64 = parts[0];
	let provided_signature = parts[1];

	let mut hasher = Sha256::new();
	hasher.update(payload_b64.as_bytes());
	hasher.update(secret.as_bytes());
	let expected_signature = URL_SAFE_NO_PAD.encode(hasher.finalize());

	if provided_signature != expected_signature {
		return Err(Error::Auth(
			"Invalid OAuth CSRF token signature".to_string(),
		));
	}

	let payload_json = URL_SAFE_NO_PAD
		.decode(payload_b64)
		.map_err(|_| Error::Auth("Failed to decode OAuth CSRF token".to_string()))?;

	let payload_str = String::from_utf8(payload_json)
		.map_err(|_| Error::Auth("Invalid OAuth CSRF token encoding".to_string()))?;

	let payload: OAuthCsrfPayload = serde_json::from_str(&payload_str)
		.map_err(|_| Error::Auth("Failed to parse OAuth CSRF token".to_string()))?;

	let now = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.map_err(|_| Error::Auth("Failed to get current timestamp".to_string()))?
		.as_secs();

	if now > payload.timestamp + max_age_seconds {
		return Err(Error::Auth("OAuth CSRF token has expired".to_string()));
	}

	if payload.timestamp > now + 60 {
		return Err(Error::Auth(
			"OAuth CSRF token timestamp is in the future".to_string(),
		));
	}

	Ok(payload.pkce_verifier)
}
