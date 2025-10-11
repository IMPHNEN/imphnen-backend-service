use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    Extension,
};
use imphnen_libs::AppState;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};

// Simple rate limiting middleware for auth endpoints
pub async fn auth_rate_limiting_middleware(
    Extension(_state): Extension<AppState>,
    mut req: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let uri = req.uri().path().to_string();
    
    // Only apply rate limiting to auth endpoints
    if uri == "/v1/auth/login" || uri == "/v1/auth/register" {
        // Get client IP (simplified for this example)
        let client_ip = "127.0.0.1"; // In production, use proper IP extraction
        
        // Create a simple in-memory rate limiter
        let limiter = Arc::new(RwLock::new(HashMap::new()));
        
        let now = Instant::now();
        let window = Duration::from_secs(60); // 1 minute window
        let max_requests = 10; // 10 requests per minute
        
        {
            let mut limiter = limiter.write().unwrap();
            
            // Clean up old entries
            limiter.retain(|_, (timestamp, _)| {
                now.duration_since(*timestamp) < window
            });
            
            // Check rate limit
            let entry = limiter.entry(client_ip.to_string()).or_insert((now, 0));
            let (timestamp, count) = entry;
            
            if now.duration_since(*timestamp) > window {
                *count = 1;
            } else if *count >= max_requests {
                return Ok(Response::builder()
                    .status(StatusCode::TOO_MANY_REQUESTS)
                    .header("Retry-After", "60")
                    .body("Too Many Requests: Rate limit exceeded for authentication endpoint".into())
                    .unwrap());
            } else {
                *count += 1;
            }
        }
    }

    Ok(next.run(req).await)
}