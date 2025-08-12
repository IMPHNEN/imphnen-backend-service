use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use sha2::{Sha256, Digest};
use imphnen_entities::error_dto::error::Error;

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

/// Generate a signed CSRF token that can be validated without server-side storage
pub fn generate_csrf_token(secret: &str) -> Result<String, Error> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| Error::Auth("Failed to get timestamp".to_string()))?
        .as_secs();
    
    let random = uuid::Uuid::new_v4().to_string();
    
    let payload = CsrfPayload {
        timestamp,
        random,
    };
    
    let payload_json = serde_json::to_string(&payload)
        .map_err(|_| Error::Auth("Failed to serialize CSRF payload".to_string()))?;
    
    let payload_b64 = URL_SAFE_NO_PAD.encode(payload_json.as_bytes());
    
    // Create signature
    let mut hasher = Sha256::new();
    hasher.update(payload_b64.as_bytes());
    hasher.update(secret.as_bytes());
    let signature = URL_SAFE_NO_PAD.encode(hasher.finalize());
    
    Ok(format!("{}.{}", payload_b64, signature))
}

/// Generate a signed OAuth CSRF token with PKCE verifier
pub fn generate_oauth_csrf_token(secret: &str, pkce_verifier: &str) -> Result<String, Error> {
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
    
    let payload_json = serde_json::to_string(&payload)
        .map_err(|_| Error::Auth("Failed to serialize OAuth CSRF payload".to_string()))?;
    
    let payload_b64 = URL_SAFE_NO_PAD.encode(payload_json.as_bytes());
    
    // Create signature
    let mut hasher = Sha256::new();
    hasher.update(payload_b64.as_bytes());
    hasher.update(secret.as_bytes());
    let signature = URL_SAFE_NO_PAD.encode(hasher.finalize());
    
    Ok(format!("{}.{}", payload_b64, signature))
}

/// Validate a CSRF token
pub fn validate_csrf_token(token: &str, secret: &str, max_age_seconds: u64) -> Result<(), Error> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 2 {
        return Err(Error::Auth("Invalid CSRF token format".to_string()));
    }
    
    let payload_b64 = parts[0];
    let provided_signature = parts[1];
    
    // Verify signature
    let mut hasher = Sha256::new();
    hasher.update(payload_b64.as_bytes());
    hasher.update(secret.as_bytes());
    let expected_signature = URL_SAFE_NO_PAD.encode(hasher.finalize());
    
    if provided_signature != expected_signature {
        return Err(Error::Auth("Invalid CSRF token signature".to_string()));
    }
    
    // Decode and validate payload
    let payload_json = URL_SAFE_NO_PAD.decode(payload_b64)
        .map_err(|_| Error::Auth("Failed to decode CSRF token".to_string()))?;
    
    let payload_str = String::from_utf8(payload_json)
        .map_err(|_| Error::Auth("Invalid CSRF token encoding".to_string()))?;
    
    let payload: CsrfPayload = serde_json::from_str(&payload_str)
        .map_err(|_| Error::Auth("Failed to parse CSRF token".to_string()))?;
    
    // Check timestamp
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| Error::Auth("Failed to get current timestamp".to_string()))?
        .as_secs();
    
    if now > payload.timestamp + max_age_seconds {
        return Err(Error::Auth("CSRF token has expired".to_string()));
    }
    
    if payload.timestamp > now + 60 {  // Allow 1 minute clock skew
        return Err(Error::Auth("CSRF token timestamp is in the future".to_string()));
    }
    
    Ok(())
}

/// Validate OAuth CSRF token and extract PKCE verifier
pub fn validate_oauth_csrf_token(token: &str, secret: &str, max_age_seconds: u64) -> Result<String, Error> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 2 {
        return Err(Error::Auth("Invalid OAuth CSRF token format".to_string()));
    }
    
    let payload_b64 = parts[0];
    let provided_signature = parts[1];
    
    // Verify signature
    let mut hasher = Sha256::new();
    hasher.update(payload_b64.as_bytes());
    hasher.update(secret.as_bytes());
    let expected_signature = URL_SAFE_NO_PAD.encode(hasher.finalize());
    
    if provided_signature != expected_signature {
        return Err(Error::Auth("Invalid OAuth CSRF token signature".to_string()));
    }
    
    // Decode and validate payload
    let payload_json = URL_SAFE_NO_PAD.decode(payload_b64)
        .map_err(|_| Error::Auth("Failed to decode OAuth CSRF token".to_string()))?;
    
    let payload_str = String::from_utf8(payload_json)
        .map_err(|_| Error::Auth("Invalid OAuth CSRF token encoding".to_string()))?;
    
    let payload: OAuthCsrfPayload = serde_json::from_str(&payload_str)
        .map_err(|_| Error::Auth("Failed to parse OAuth CSRF token".to_string()))?;
    
    // Check timestamp
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| Error::Auth("Failed to get current timestamp".to_string()))?
        .as_secs();
    
    if now > payload.timestamp + max_age_seconds {
        return Err(Error::Auth("OAuth CSRF token has expired".to_string()));
    }
    
    if payload.timestamp > now + 60 {  // Allow 1 minute clock skew
        return Err(Error::Auth("OAuth CSRF token timestamp is in the future".to_string()));
    }
    
    Ok(payload.pkce_verifier)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csrf_token_generation_and_validation() {
        let secret = "test_secret";
        
        // Generate token
        let token = generate_csrf_token(secret).unwrap();
        
        // Validate token (should pass)
        assert!(validate_csrf_token(&token, secret, 300).is_ok());
        
        // Validate with wrong secret (should fail)
        assert!(validate_csrf_token(&token, "wrong_secret", 300).is_err());
    }
    
    #[test]
    fn test_csrf_token_expiration() {
        let secret = "test_secret";
        let token = generate_csrf_token(secret).unwrap();
        
        // Add a 2 second delay to ensure the token expires when max_age is 1 second
        std::thread::sleep(std::time::Duration::from_secs(2));
        
        // Should fail with 1 second max age (token is now 2 seconds old)
        assert!(validate_csrf_token(&token, secret, 1).is_err());
        
        // Should still work with a large max age
        assert!(validate_csrf_token(&token, secret, 300).is_ok());
    }
    
    #[test]
    fn test_invalid_csrf_token_format() {
        let secret = "test_secret";
        
        // Invalid format (no dot)
        assert!(validate_csrf_token("invalid_token", secret, 300).is_err());
        
        // Invalid format (too many dots)
        assert!(validate_csrf_token("a.b.c", secret, 300).is_err());
    }
}
