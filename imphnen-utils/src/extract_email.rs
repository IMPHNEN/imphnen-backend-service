use tracing::{info, error};
use crate::decode_access_token;
use axum::http::{HeaderMap, header::AUTHORIZATION};

/// Extracts the email from the Authorization header, if present and valid.
pub fn extract_email(headers: &HeaderMap) -> Option<String> {
    info!(?headers, "extract_email called with headers");
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
    info!(token, "Extracted bearer token in extract_email");
    match decode_access_token(token) {
        Ok(data) => {
            info!(email = %data.claims.sub, "Successfully decoded access token in extract_email");
            Some(data.claims.sub)
        }
        Err(e) => {
            error!(error = ?e, "Failed to decode access token in extract_email");
            None
        }
    }
}

/// Extracts the email from a JWT token string.
pub fn extract_email_token(token: String) -> Option<String> {
    info!(token = %token, "extract_email_token called with token");
    match decode_access_token(&token) {
        Ok(data) => {
            info!(email = %data.claims.sub, "Successfully decoded token in extract_email_token");
            Some(data.claims.sub)
        }
        Err(e) => {
            error!(error = ?e, "Failed to decode token in extract_email_token");
            None
        }
    }
}
