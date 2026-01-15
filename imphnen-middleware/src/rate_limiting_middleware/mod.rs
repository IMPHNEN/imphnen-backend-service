use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
    middleware::Next,
    Extension,
};
use chrono::{DateTime, FixedOffset, Utc, Duration};
use imphnen_libs::{AppState};
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, Set, ActiveModelTrait};
use uuid::Uuid;
use imphnen_utils::extract_real_ip;
use imphnen_entities::seaorm::common::rate_limit::Entity as RateLimitEntity;
use imphnen_entities::seaorm::common::rate_limit::ActiveModel as RateLimitActiveModel;
use imphnen_entities::seaorm::common::rate_limit::Column as RateLimitColumn;

/// Rate limiting middleware yang menggunakan PostgreSQL (SeaORM) untuk semua public endpoints
///
/// Migration dari SurrealDB ke PostgreSQL selesai - kini menggunakan sistem rate limiting
/// yang lebih scalable dan terintegrasi dengan backend utama
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
        
        // Periksa rate limit menggunakan PostgreSQL (SeaORM)
                match check_rate_limit(&state.postgres_connection.conn, &client_ip, max_requests, window_duration_secs).await {
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

/// Middleware rate limiting khusus untuk endpoint autentikasi
///
/// Menggunakan PostgreSQL (SeaORM) sebagai backend - kompatibilitas legacy dengan SurrealDB
/// telah dihapus selain fungsionalitas yang sama
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
        
        // Periksa rate limit menggunakan PostgreSQL (SeaORM)
                match check_rate_limit(&state.postgres_connection.conn, &client_ip, max_requests, window_duration_secs).await {
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
        "/v1/cms/landing",
    ];
    
    public_endpoints.iter().any(|endpoint| uri.starts_with(endpoint))
}

/// Periksa rate limit untuk IP tertentu menggunakan PostgreSQL (SeaORM)
///
/// Implementasi rate limiting yang didesain untuk skala besar dengan PostgreSQL,
/// menggantikan implementasi SurrealDB yang sebelumnya
async fn check_rate_limit(
    db: &sea_orm::DatabaseConnection,
    ip_address: &str,
    max_requests: u32,
    window_duration_secs: u64,
) -> Result<bool, Box<dyn std::error::Error>> {
    let now = Utc::now();
    let window_start = now - Duration::seconds(window_duration_secs as i64);

    // Cari record rate limit untuk IP ini
    let existing_record = RateLimitEntity::find()
        .filter(RateLimitColumn::IpAddress.eq(ip_address))
        .one(db)
        .await?;

    match existing_record {
        Some(record) => {
            // Konversi ke ActiveModel untuk modifikasi
            let mut active_model: RateLimitActiveModel = record.into();
            
            // Reset counter jika window sudah expired
            let was_reset = if active_model.last_request_time.clone().unwrap() <= window_start {
                active_model.request_count = Set(0);
                active_model.last_request_time = Set(DateTime::<FixedOffset>::from(now));
                true
            } else {
                false
            };

            if !was_reset {
                // Increment counter jika masih dalam window
                let current_count = active_model.request_count.clone().unwrap();
                active_model.request_count = Set(current_count + 1);
            }

            // Simpan perubahan ke database
            let updated_model = active_model.update(db).await?;

            // Periksa apakah rate limit terlampaui
            Ok(updated_model.request_count > max_requests)
        }
        None => {
            // Buat record baru dengan nilai awal
            let new_record = RateLimitActiveModel {
                id: Set(Uuid::new_v4().to_string()),
                ip_address: Set(ip_address.to_string()),
                request_count: Set(1),
                first_request_time: Set(DateTime::<FixedOffset>::from(now)),
                last_request_time: Set(DateTime::<FixedOffset>::from(now)),
                window_duration_secs: Set(window_duration_secs as i64),
            };

            // Simpan record baru ke database
            new_record.insert(db).await?;

            Ok(false) // Request pertama selalu diizinkan
        }
    }
}