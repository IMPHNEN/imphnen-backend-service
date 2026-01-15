//! Email extraction utilities from authentication tokens.
//!
//! This module provides functions to extract email addresses from JWT tokens
//! and Google OAuth access tokens, supporting both synchronous and asynchronous
//! validation methods.

use tracing::{error, info};
use imphnen_libs::jsonwebtoken::decode_access_token;
use axum::http::{HeaderMap, header::AUTHORIZATION};

/// Extracts the email from the Authorization header, if present and valid.
/// Supports both our internal JWT tokens and Google access tokens.
pub fn extract_email(headers: &HeaderMap) -> Option<String> {
    let auth_header = match headers.get(AUTHORIZATION) {
        Some(h) => h,
        None => {
            error!("Authorization header missing in extract_email");
            return None;
        }
    };
    let auth_str = match auth_header.to_str() {
        Ok(s) => s,
        Err(e) => {
            error!(error = ?e, "Failed to convert Authorization header to str in extract_email");
            return None;
        }
    };
    let token = match auth_str.strip_prefix("Bearer ") {
        Some(t) => t,
        None => {
            error!(auth_str, "Authorization header does not start with 'Bearer ' in extract_email");
            return None;
        }
    };

    // First try to decode as our internal JWT token
    match decode_access_token(token) {
        Ok(data) => {
            Some(data.claims.sub)
        }
        Err(_) => {
            // If it fails, it might be a Google access token
            // For Google tokens, we need async validation, so we'll return None here
            // and handle Google tokens separately in the calling code
            error!("Token is not a valid internal JWT. If this is a Google token, please use extract_email_async or handle Google OAuth flow properly.");
            None
        }
    }
}

/// Async version that can handle Google access tokens
pub async fn extract_email_async(headers: &HeaderMap) -> Option<String> {
    let auth_header = match headers.get(AUTHORIZATION) {
        Some(h) => h,
        None => {
            error!("Authorization header missing in extract_email_async");
            return None;
        }
    };
    let auth_str = match auth_header.to_str() {
        Ok(s) => s,
        Err(e) => {
            error!(error = ?e, "Failed to convert Authorization header to str in extract_email_async");
            return None;
        }
    };
    let token = match auth_str.strip_prefix("Bearer ") {
        Some(t) => t,
        None => {
            error!(auth_str, "Authorization header does not start with 'Bearer ' in extract_email_async");
            return None;
        }
    };

    // First try to decode as our internal JWT token
    match decode_access_token(token) {
        Ok(data) => {
            Some(data.claims.sub)
        }
        Err(_) => {
            // If it fails, try to validate as Google access token
            extract_email_from_google_token(token).await
        }
    }
}

/// Extracts email from Google access token by calling Google's tokeninfo endpoint
async fn extract_email_from_google_token(token: &str) -> Option<String> {
    use serde_json::Value;
    
    let client = reqwest::Client::new();
    let tokeninfo_url = format!("https://oauth2.googleapis.com/tokeninfo?access_token={token}");
    
    match client.get(&tokeninfo_url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<Value>().await {
                    Ok(token_info) => {
                        if let Some(email) = token_info.get("email").and_then(|e| e.as_str()) {
                            info!(email = %email, "Successfully extracted email from Google token");
                            Some(email.to_string())
                        } else {
                            error!("Email not found in Google token info response");
                            None
                        }
                    }
                    Err(e) => {
                        error!(error = ?e, "Failed to parse Google token info response");
                        None
                    }
                }
            } else {
                error!(status = %response.status(), "Google token validation failed");
                None
            }
        }
        Err(e) => {
            error!(error = ?e, "Failed to validate Google token");
            None
        }
    }
}

/// Extracts the email from a JWT token string.
/// Supports both our internal JWT tokens and Google access tokens.
pub fn extract_email_token(token: String) -> Option<String> {
    match decode_access_token(&token) {
        Ok(data) => {
            Some(data.claims.sub)
        }
        Err(_) => {
            // If it fails, it might be a Google access token
            // For Google tokens, we need async validation, so we'll return None here
            // and handle Google tokens separately in the calling code
            error!("Token is not a valid internal JWT. If this is a Google token, please use extract_email_token_async or handle Google OAuth flow properly.");
            None
        }
    }
}

/// A simple helper to check if a token string looks like a JWT.
fn is_jwt(token: &str) -> bool {
    let parts: Vec<_> = token.split('.').collect();
    parts.len() == 3
}

/// Async version of extract_email_token that can handle Google access tokens
pub async fn extract_email_token_async(token: String) -> Option<String> {
    if is_jwt(&token) && let Ok(data) = decode_access_token(&token) {
        return Some(data.claims.sub);
    }

    // If it's not a valid internal JWT, try to validate as Google access token
    extract_email_from_google_token(&token).await
}