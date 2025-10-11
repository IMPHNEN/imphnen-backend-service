use axum::{
    body::Body,
    http::{Request, Response},
    middleware::Next,
    Extension,
};
use chrono::Utc;
use imphnen_entities::AuditLogSchema;
use imphnen_libs::{AppState, ResourceEnum};
use imphnen_utils::{extract_email, extract_email_async, extract_real_ip};
use std::convert::Infallible;

/// Middleware untuk mencatat semua aksi admin ke dalam audit log
pub async fn audit_logging_middleware(
    Extension(state): Extension<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response<Body>, Infallible> {
    let uri = req.uri().path().to_string();
    
    // Hanya catat aksi admin (endpoint yang memerlukan permissions)
    if is_admin_action(&uri) {
        // Extract informasi pengguna dari headers
        let headers = req.headers();
        let user_email = extract_user_email(headers).await;
        let user_id = extract_user_id(&state, &user_email).await;
        let ip_address = extract_real_ip(headers).unwrap_or_else(|| "unknown".to_string());
        let user_agent = extract_user_agent(headers);
        
        // Ekstrak informasi aksi dari request
        let action = extract_action(&uri, req.method().as_str());
        let resource = extract_resource(&uri);
        let resource_id = extract_resource_id(&uri);
        
        // Simpan audit log sebelum memproses request
        let audit_log = AuditLogSchema {
            id: None,
            user_id: user_id.clone().unwrap_or_else(|| "unknown".to_string()),
            user_email: user_email.clone().unwrap_or_else(|| "unknown".to_string()),
            action,
            resource,
            resource_id,
            old_data: None, // Untuk UPDATE/DELETE, perlu diisi setelah request
            new_data: None, // Untuk CREATE/UPDATE, perlu diisi setelah request
            ip_address,
            user_agent,
            timestamp: Utc::now(),
        };
        
        // Simpan audit log ke database
        if let Err(e) = save_audit_log(&state.surrealdb_mem, audit_log).await {
            log::error!("Failed to save audit log: {}", e);
        }
    }
    
    // Lanjutkan dengan request
    let response = next.run(req).await;
    Ok(response)
}

/// Periksa apakah endpoint termasuk aksi admin
fn is_admin_action(uri: &str) -> bool {
    // Daftar endpoint admin yang perlu diaudit
    let admin_endpoints = [
        "/v1/admin/",
        "/v1/teams/admin/",
        "/v1/users/admin/",
        "/v1/permissions/",
        "/v1/roles/",
        "/v1/gacha/admin/",
        "/v1/hackathon/admin/",
        "/v1/cms/admin/",
    ];
    
    admin_endpoints.iter().any(|endpoint| uri.starts_with(endpoint))
}

/// Extract email pengguna dari headers
async fn extract_user_email(headers: &axum::http::HeaderMap) -> Option<String> {
    // Coba extract email secara synchronous terlebih dahulu
    match extract_email(headers) {
        Some(email) => Some(email),
        None => {
            // Jika tidak ada, coba secara asynchronous
            extract_email_async(headers).await
        }
    }
}

/// Extract user ID dari email menggunakan auth repository
async fn extract_user_id(state: &AppState, email: &Option<String>) -> Option<String> {
    if let Some(email) = email {
        match state.auth_repository.query_get_stored_user(email.clone()).await {
            Ok(user) => Some(user.id.id.to_string()),
            Err(_) => None,
        }
    } else {
        None
    }
}

/// Extract user agent dari headers
fn extract_user_agent(headers: &axum::http::HeaderMap) -> Option<String> {
    headers.get("user-agent")
        .and_then(|value| value.to_str().ok())
        .map(|s| s.to_string())
}

/// Extract tipe aksi dari URI dan method
fn extract_action(uri: &str, method: &str) -> String {
    match method {
        "POST" => "CREATE",
        "PUT" | "PATCH" => "UPDATE", 
        "DELETE" => "DELETE",
        "GET" => {
            if uri.contains("/admin/") {
                "VIEW"
            } else {
                "ACCESS"
            }
        },
        _ => "UNKNOWN",
    }.to_string()
}

/// Extract resource dari URI
fn extract_resource(uri: &str) -> String {
    // Ambil bagian setelah /v1/ sebagai resource
    if let Some(resource_part) = uri.split("/v1/").nth(1) {
        if let Some(resource) = resource_part.split('/').next() {
            return resource.to_string();
        }
    }
    "unknown".to_string()
}

/// Extract resource ID dari URI
fn extract_resource_id(uri: &str) -> Option<String> {
    // Cari bagian yang seperti UUID atau ID numerik
    let segments = uri.split('/').collect::<Vec<&str>>();
    
    for segment in segments.iter().rev() {
        if segment.len() == 36 && segment.contains('-') {
            // Kemungkinan UUID
            return Some(segment.to_string());
        } else if segment.chars().all(|c| c.is_ascii_digit()) {
            // Kemungkinan ID numerik
            return Some(segment.to_string());
        }
    }
    
    None
}

/// Simpan audit log ke database
async fn save_audit_log(
    db: &imphnen_libs::SurrealMemClient,
    audit_log: AuditLogSchema,
) -> Result<(), Box<dyn std::error::Error>> {
    let table = ResourceEnum::AuditLog.to_string();
    let key = (table.as_str(), surrealdb::sql::Id::rand().to_string());
    
    db.create(key)
        .content(audit_log)
        .await?;
    
    log::debug!("Audit log saved for action: {}", audit_log.action);
    Ok(())
}

/// Middleware khusus untuk aksi UPDATE/DELETE yang menangkap data sebelum dan sesudah
pub async fn detailed_audit_logging_middleware(
    Extension(state): Extension<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response<Body>, Infallible> {
    // Implementasi ini akan lebih kompleks dan membutuhkan intercept response
    // Untuk sekarang, gunakan basic audit logging
    audit_logging_middleware(Extension(state), req, next).await
}