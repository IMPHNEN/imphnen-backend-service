//! Environment configuration module using once_cell::sync::Lazy for one-time loading.

use std::env;
use once_cell::sync::Lazy;
// Logging for warnings if .env is missing
use log::warn;

/// Struct holding all environment configuration.
pub struct Env {
    pub port: u16,
    pub access_token_secret: String,
    pub refresh_token_secret: String,
    pub surrealdb_url: String,
    pub surrealdb_username: String,
    pub surrealdb_password: String,
    pub surrealdb_namespace: String,
    pub surrealdb_dbname: String,
    pub surrealdb_url_ws: String,
    pub smtp_email: String,
    pub smtp_password: String,
    pub smtp_name: String,
    pub smtp_host: String,
    pub redisdb_url: String,
    pub fe_url: String,
    pub rust_env: String,
    pub minio_endpoint: String,
    pub minio_bucket_name: String,
    pub minio_access_key: String,
    pub minio_secret_key: String,
    pub minio_region: String,
    pub minio_secure: bool,
    // Google OAuth 2.1
    pub google_client_id: String,
    pub google_client_secret: String,
    pub google_redirect_url: String,
}

/// Helper to get env var with warning if not set.
fn get_env_with_warning(key: &str, default: &str) -> String {
    match env::var(key) {
        Ok(val) => val,
        Err(_) => {
            warn!("Environment variable '{}' is not set. Using default: '{}'", key, default);
            default.to_string()
        }
    }
}

/// Loads environment variables from .env and system, only once.
pub static ENV: Lazy<Env> = Lazy::new(|| {
    // Try to load .env file, log a warning if not found, proceed regardless.
    match dotenvy::dotenv() {
        Ok(_) => {}
        Err(dotenvy::Error::Io(ref e)) if e.kind() == std::io::ErrorKind::NotFound => {
            warn!(".env file not found, falling back to system environment variables");
        }
        Err(_) => {}
    }

    Env {
        port: get_env_with_warning("PORT", "3000")
            .parse()
            .unwrap_or(3000),
        access_token_secret: get_env_with_warning("ACCESS_TOKEN_SECRET", "default_access_secret"),
        refresh_token_secret: get_env_with_warning("REFRESH_TOKEN_SECRET", "default_refresh_secret"),
        surrealdb_url: get_env_with_warning("SURREALDB_URL", "http://localhost:8000"),
        surrealdb_username: get_env_with_warning("SURREALDB_USERNAME", "root"),
        surrealdb_password: get_env_with_warning("SURREALDB_PASSWORD", "root"),
        surrealdb_namespace: get_env_with_warning("SURREALDB_NAMESPACE", "namespace"),
        surrealdb_dbname: get_env_with_warning("SURREALDB_DBNAME", "database"),
        smtp_email: get_env_with_warning("SMTP_EMAIL", "no-reply@example.com"),
        smtp_password: get_env_with_warning("SMTP_PASSWORD", "default_smtp_password"),
        smtp_name: get_env_with_warning("SMTP_NAME", "MyApp SMTP"),
        smtp_host: get_env_with_warning("SMTP_HOST", "smtp.gmail.com"),
        redisdb_url: get_env_with_warning("REDISDB_URL", "localhost"),
        fe_url: get_env_with_warning("FE_URL", "http://localhost"),
        rust_env: get_env_with_warning("RUST_ENV", "development"),
        minio_endpoint: get_env_with_warning("MINIO_ENDPOINT", "http://localhost:9000"),
        minio_bucket_name: get_env_with_warning("MINIO_BUCKET_NAME", "imphnen-uploads"),
        minio_access_key: get_env_with_warning("MINIO_ACCESS_KEY", "minio_access"),
        minio_secret_key: get_env_with_warning("MINIO_SECRET_KEY", "minio_secret"),
        minio_region: get_env_with_warning("MINIO_REGION", "us-east-1"),
        minio_secure: get_env_with_warning("MINIO_SECURE", "false")
            .parse()
            .unwrap_or(false),
        surrealdb_url_ws: String::new(),
        // Google OAuth 2.1
        google_client_id: get_env_with_warning("GOOGLE_CLIENT_ID", "default_google_client_id"),
        google_client_secret: get_env_with_warning("GOOGLE_CLIENT_SECRET", "default_google_client_secret"),
        google_redirect_url: get_env_with_warning("GOOGLE_REDIRECT_URL", "http://localhost:8000/api/v1/auth/google/callback"),
    }
});
