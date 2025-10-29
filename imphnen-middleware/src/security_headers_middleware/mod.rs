use axum::{
    http::{HeaderValue, Request, Response},
    middleware::Next,
    Extension,
};
use imphnen_libs::{AppState, ENV};
use rand::RngCore;
use std::convert::Infallible;

/// Security headers middleware that adds various security-related HTTP headers to all responses.
///
/// This middleware implements security best practices by adding headers that help protect
/// against common web attacks like clickjacking, XSS, and information leakage.
pub async fn security_headers_middleware(
    Extension(_state): Extension<AppState>,
    req: Request<axum::body::Body>,
    next: Next,
) -> Result<Response<axum::body::Body>, Infallible> {
    // Generate nonce for CSP if in development mode
    let nonce = if ENV.rust_env != "production" {
        generate_nonce()
    } else {
        String::new()
    };
    
    let res = next.run(req).await;
    
    let res = add_security_headers(res, &nonce);
    
    Ok(res)
}

/// Adds security headers to a response based on the current environment.
///
/// # Arguments
/// * `res` - The response to add headers to
/// * `nonce` - Nonce value for CSP (empty in production)
///
/// # Returns
/// The response with security headers added
fn add_security_headers(mut res: Response<axum::body::Body>, nonce: &str) -> Response<axum::body::Body> {
    let headers = res.headers_mut();
    
    // Strict-Transport-Security (HSTS)
    // Prevents downgrade attacks and cookie hijacking
    // Only enable in production to avoid HSTS pinning issues during development
    if ENV.rust_env == "production" {
        headers.insert(
            "Strict-Transport-Security",
            HeaderValue::from_static("max-age=31536000; includeSubDomains; preload"),
        );
    } else {
        headers.insert(
            "Strict-Transport-Security",
            HeaderValue::from_static("max-age=0"),
        );
    }
    
    // Content-Security-Policy (CSP)
    // Mitigates XSS and data injection attacks
    let csp = if ENV.rust_env == "production" {
        // Production CSP - strict policy for production
        "default-src 'self'; script-src 'self' https://trusted-cdn.com; style-src 'self' 'unsafe-inline' https://fonts.googleapis.com; font-src 'self' https://fonts.gstatic.com; img-src 'self' data: https://images.example.com; connect-src 'self' https://api.example.com; frame-src 'none'; object-src 'none'; base-uri 'self'; form-action 'self'; report-uri /csp-violation-report-endpoint".to_string()
    } else {
        // Development CSP - secure nonce-based approach
        if nonce.is_empty() {
            // Fallback if nonce generation fails
            "default-src 'self' http://localhost:3000; script-src 'self' http://localhost:3000; style-src 'self' http://localhost:3000; img-src 'self' data: http://localhost:3000; connect-src 'self' http://localhost:3000 ws://localhost:3000; frame-src 'none'; object-src 'none'; base-uri 'self'; form-action 'self'".to_string()
        } else {
            // Nonce-based CSP for development
            format!("default-src 'self' http://localhost:3000; script-src 'self' http://localhost:3000 'nonce-{}'; style-src 'self' http://localhost:3000 'nonce-{}'; img-src 'self' data: http://localhost:3000; connect-src 'self' http://localhost:3000 ws://localhost:3000; frame-src 'none'; object-src 'none'; base-uri 'self'; form-action 'self'", nonce, nonce)
        }
    };
    
    headers.insert("Content-Security-Policy", HeaderValue::from_str(&csp).unwrap());
    
    // Add nonce to response headers for frontend use (development only)
    if ENV.rust_env != "production" && !nonce.is_empty() {
        headers.insert("X-CSP-Nonce", HeaderValue::from_str(nonce).unwrap());
    }
    
    // X-Frame-Options
    // Prevents clickjacking attacks
    headers.insert(
        "X-Frame-Options",
        HeaderValue::from_static("DENY"),
    );
    
    // X-Content-Type-Options
    // Prevents MIME sniffing attacks
    headers.insert(
        "X-Content-Type-Options",
        HeaderValue::from_static("nosniff"),
    );
    
    // Referrer-Policy
    // Controls how much referrer information should be included with requests
    headers.insert(
        "Referrer-Policy",
        HeaderValue::from_static("strict-origin-when-cross-origin"),
    );
    
    // Permissions-Policy (Feature Policy)
    // Controls which features and APIs can be used
    headers.insert(
        "Permissions-Policy",
        HeaderValue::from_static("camera=(), microphone=(), geolocation=()"),
    );
    
    // X-XSS-Protection
    // Provides basic XSS protection (note: this is a legacy header and CSP is preferred)
    headers.insert(
        "X-XSS-Protection",
        HeaderValue::from_static("1; mode=block"),
    );
    
    res
}

/// Generate a random nonce for CSP
fn generate_nonce() -> String {
    use base64::{Engine as _, engine::general_purpose::STANDARD};
    let mut rng = rand::rng();
    let mut random_bytes = [0u8; 16];
    rng.fill_bytes(&mut random_bytes);
    STANDARD.encode(random_bytes)
}