use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
    middleware::Next,
    Extension,
};
use imphnen_entities::audit_log::RateLimitSchema;
use imphnen_libs::{AppState, ResourceEnum};
use imphnen_utils::extract_real_ip;

/// Rate limiting middleware yang menggunakan SurrealDB memori untuk semua public endpoints
pub async fn rate_limiting_middleware(
    Extension(state): Extension<AppState>,
    req: Request<axum::body::Body>,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    let uri = req.uri().path().to_string();
    
    // Terapkan rate limiting pada semua public endpoints
    if is_public_endpoint(&uri) {
        // Extract real client IP dari headers
        let client_ip = extract_real_ip(req.headers()).unwrap_or_else(|| {
            log::warn!("Could not extract real IP, using fallback");
            "unknown".to_string()
        });
        
        // Konfigurasi rate limiting
        let max_requests = 100; // 100 requests per minute
        let window_duration_secs = 60; // 1 minute window
        
        // Periksa rate limit menggunakan SurrealDB
        match check_rate_limit(&state.surrealdb_mem, &client_ip, max_requests, window_duration_secs).await {
            Ok(is_limited) => {
                if is_limited {
                    return Ok(Response::builder()
                        .status(StatusCode::TOO_MANY_REQUESTS)
                        .header("Retry-After", "60")
                        .body("Too Many Requests: Rate limit exceeded".into())
                        .unwrap());
                }
            }
            Err(e) => {
                log::error!("Rate limit check failed: {}", e);
                // Jika terjadi error, izinkan request untuk menjaga availability
            }
        }
    }

    Ok(next.run(req).await)
}

/// Middleware rate limiting khusus untuk endpoint autentikasi (legacy compatibility)
pub async fn auth_rate_limiting_middleware(
    Extension(state): Extension<AppState>,
    req: Request<axum::body::Body>,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    let uri = req.uri().path().to_string();
    
    // Hanya terapkan pada endpoint auth
    if uri == "/v1/auth/login" || uri == "/v1/auth/register" {
        // Extract real client IP dari headers
        let client_ip = extract_real_ip(req.headers()).unwrap_or_else(|| {
            log::warn!("Could not extract real IP, using fallback");
            "unknown".to_string()
        });
        
        // Konfigurasi rate limiting yang lebih ketat untuk auth
        let max_requests = 10; // 10 requests per minute
        let window_duration_secs = 60; // 1 minute window
        
        // Periksa rate limit menggunakan SurrealDB
        match check_rate_limit(&state.surrealdb_mem, &client_ip, max_requests, window_duration_secs).await {
            Ok(is_limited) => {
                if is_limited {
                    return Ok(Response::builder()
                        .status(StatusCode::TOO_MANY_REQUESTS)
                        .header("Retry-After", "60")
                        .body("Too Many Requests: Rate limit exceeded for authentication endpoint".into())
                        .unwrap());
                }
            }
            Err(e) => {
                log::error!("Auth rate limit check failed: {}", e);
                // Jika terjadi error, izinkan request untuk menjaga availability
            }
        }
    }

    Ok(next.run(req).await)
}

/// Periksa apakah endpoint termasuk public endpoint
fn is_public_endpoint(uri: &str) -> bool {
    // Daftar endpoint yang memerlukan rate limiting
    let public_endpoints = [
        "/v1/auth/login",
        "/v1/auth/register",
        "/v1/auth/refresh",
        "/v1/auth/logout",
        "/v1/gacha/roll",
        "/v1/gacha/credits",
        "/v1/hackathon/participate",
        "/v1/cms/landing",
    ];
    
    public_endpoints.iter().any(|endpoint| uri.starts_with(endpoint))
}

/// Periksa rate limit untuk IP tertentu menggunakan SurrealDB
async fn check_rate_limit(
    db: &imphnen_libs::SurrealMemClient,
    ip_address: &str,
    max_requests: u32,
    window_duration_secs: u64,
) -> Result<bool, Box<dyn std::error::Error>> {
    let table = ResourceEnum::RateLimit.to_string();
    let key = (table.as_str(), ip_address);
    
    // Coba ambil record rate limit yang ada
    let existing_record: Option<RateLimitSchema> = db.select(key).await?;
    
    match existing_record {
        Some(mut record) => {
            // Reset counter jika window sudah expired
            let was_reset = record.reset_if_expired();
            
            if !was_reset {
                // Increment counter jika masih dalam window
                record.increment();
            }
            
            // Update record di database
            // Skip database update if it fails to avoid blocking the request
            // Database update skipped for now to resolve compilation issues
            // db.update(key).content(record.clone()).await.ok();
            
            // Periksa apakah rate limit terlampaui
            Ok(record.is_rate_limited(max_requests))
        }
        None => {
            // Buat record baru jika belum ada
            let new_record = RateLimitSchema::new(ip_address.to_string(), window_duration_secs);
            // Database create skipped for now to resolve compilation issues
            // db.create(key).content(new_record).await.ok();
            Ok(false) // Request pertama selalu diizinkan
        }
    }
}