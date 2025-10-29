use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

/// Schema untuk audit log yang mencatat semua aksi admin
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuditLogSchema {
    /// ID unik dari log
    pub id: Option<Thing>,
    /// ID pengguna yang melakukan aksi
    pub user_id: String,
    /// Email pengguna
    pub user_email: String,
    /// Tipe aksi yang dilakukan (CREATE, UPDATE, DELETE, etc.)
    pub action: String,
    /// Resource yang terkena aksi
    pub resource: String,
    /// ID resource yang terkena aksi
    pub resource_id: Option<String>,
    /// Data sebelum perubahan (untuk UPDATE/DELETE)
    pub old_data: Option<serde_json::Value>,
    /// Data setelah perubahan (untuk CREATE/UPDATE)
    pub new_data: Option<serde_json::Value>,
    /// IP address pengguna
    pub ip_address: String,
    /// User agent pengguna
    pub user_agent: Option<String>,
    /// Timestamp ketika aksi dilakukan
    pub timestamp: DateTime<Utc>,
}

/// Schema untuk rate limiting menggunakan SurrealDB memori
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RateLimitSchema {
    /// ID unik (IP address)
    pub id: Option<Thing>,
    /// IP address klien
    pub ip_address: String,
    /// Jumlah request dalam window saat ini
    pub request_count: u32,
    /// Timestamp pertama request dalam window
    pub first_request_time: DateTime<Utc>,
    /// Timestamp terakhir request
    pub last_request_time: DateTime<Utc>,
    /// Window duration dalam detik
    pub window_duration_secs: u64,
}

impl RateLimitSchema {
    /// Buat instance baru RateLimitSchema
    pub fn new(ip_address: String, window_duration_secs: u64) -> Self {
        let now = Utc::now();
        Self {
            id: None,
            ip_address,
            request_count: 1,
            first_request_time: now,
            last_request_time: now,
            window_duration_secs,
        }
    }

    /// Periksa apakah rate limit sudah terlampaui
    pub fn is_rate_limited(&self, max_requests: u32) -> bool {
        self.request_count > max_requests
    }

    /// Perbarui counter dan timestamp
    pub fn increment(&mut self) {
        self.request_count += 1;
        self.last_request_time = Utc::now();
    }

    /// Reset counter jika window sudah expired
    pub fn reset_if_expired(&mut self) -> bool {
        let now = Utc::now();
        let duration = now - self.first_request_time;
        
        if duration.num_seconds() >= self.window_duration_secs as i64 {
            self.request_count = 1;
            self.first_request_time = now;
            self.last_request_time = now;
            true
        } else {
            false
        }
    }
}