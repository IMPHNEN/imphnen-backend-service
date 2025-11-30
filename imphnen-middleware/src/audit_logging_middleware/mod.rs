use axum::{
    body::Body,
    http::{Request, Response},
    middleware::Next,
    Extension,
};
use chrono::{DateTime, FixedOffset, Utc};
use imphnen_entities::seaorm::common::audit_log::Model as AuditLogSchema;
use imphnen_libs::AppState;
use sea_orm::{ActiveModelTrait, Set};
use sea_orm::prelude::Uuid;
use imphnen_utils::{extract_email, extract_email_async, extract_real_ip};
use std::convert::Infallible;

/// Middleware untuk mencatat semua aksi admin ke dalam audit log
pub async fn audit_logging_middleware(
    Extension(state): Extension<AppState>,
    req: Request<Body>,
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
        let user_id_uuid = Uuid::parse_str(&user_id.clone().unwrap_or_else(|| "unknown".to_string())).unwrap_or(Uuid::nil());
        let user_agent = extract_user_agent(headers);
        
        // Ekstrak informasi aksi dari request
        let action = extract_action(&uri, req.method().as_str());
        let resource = extract_resource(&uri);
        let resource_id = extract_resource_id(&uri);
        
        // Simpan audit log sebelum memproses request
        let audit_log = AuditLogSchema {
            id: Uuid::new_v4(),
            user_id: user_id_uuid,
            user_email: user_email.clone().unwrap_or_else(|| "unknown".to_string()),
            action,
            resource,
            resource_id,
            old_data: None, // Untuk UPDATE/DELETE, perlu diisi setelah request
            new_data: None, // Untuk CREATE/UPDATE, perlu diisi setelah request
            ip_address,
            user_agent,
            timestamp: DateTime::<FixedOffset>::from(Utc::now()),
        };
        
        // Simpan audit log ke database
        let action = audit_log.action.clone();
        match save_audit_log(&state.postgres_connection.conn, audit_log.clone()).await {
            Ok(_) => log::debug!("Audit log saved for action: {}", action),
            Err(e) => log::error!("Failed to save audit log: {}", e),
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
        "/v1/users/admin/",
        "/v1/permissions/",
        "/v1/roles/",
        "/v1/gacha/admin/",
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
        match state.auth_repository.get_user_for_auth(&email.clone(), state).await {
            Ok(user) => Some(user.id.to_string()),
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
    if let Some(resource_part) = uri.split("/v1/").nth(1)
        && let Some(resource) = resource_part.split('/').next() {
            return resource.to_string();
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

/// Simpan audit log ke database menggunakan SeaORM
async fn save_audit_log(
    db: &sea_orm::DatabaseConnection,
    audit_log: AuditLogSchema,
) -> Result<(), Box<dyn std::error::Error>> {
    use imphnen_entities::seaorm::common::audit_log::ActiveModel as AuditLogActiveModel;
    
    let audit_log_model = AuditLogActiveModel {
        id: Set(audit_log.id),
        user_id: Set(audit_log.user_id),
        user_email: Set(audit_log.user_email),
        action: Set(audit_log.action.clone()),
        resource: Set(audit_log.resource),
        resource_id: Set(audit_log.resource_id),
        old_data: Set(audit_log.old_data),
        new_data: Set(audit_log.new_data),
        ip_address: Set(audit_log.ip_address),
        user_agent: Set(audit_log.user_agent),
        timestamp: Set(audit_log.timestamp),
    };

    audit_log_model.insert(db).await?;

    log::debug!("Audit log saved for action: {}", audit_log.action);
    Ok(())
}

/// Middleware khusus untuk aksi UPDATE/DELETE yang menangkap data sebelum dan sesudah
pub async fn detailed_audit_logging_middleware(
    Extension(state): Extension<AppState>,
    req: Request<Body>,
    next: Next,
) -> Result<Response<Body>, Infallible> {
    // Implementasi ini akan lebih kompleks dan membutuhkan intercept response
    // Untuk sekarang, gunakan basic audit logging
    audit_logging_middleware(Extension(state), req, next).await
}