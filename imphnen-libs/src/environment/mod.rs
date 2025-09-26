//! Environment configuration module using once_cell::sync::Lazy for one-time loading.
//!
//! This module provides centralized configuration management for the application.
//! All environment variables are loaded once at startup and cached for performance.

use std::env;
use once_cell::sync::Lazy;
use log::{warn, info};

/// Struct holding all environment configuration.
///
/// This struct contains all configuration values loaded from environment variables.
/// Sensitive values are masked in debug output for security.
#[derive(Clone)]
pub struct Env {
    pub port: u16,
    pub access_token_secret: String,
    pub refresh_token_secret: String,
    pub surrealdb_url: String,
    pub surrealdb_username: String,
    pub surrealdb_password: String,
    pub surrealdb_namespace: String,
    pub surrealdb_dbname: String,
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

// Custom Debug implementation to mask secrets in logs
impl std::fmt::Debug for Env {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Env")
            .field("port", &self.port)
            .field("access_token_secret", &"***")
            .field("refresh_token_secret", &"***")
            .field("surrealdb_url", &self.surrealdb_url)
            .field("surrealdb_username", &self.surrealdb_username)
            .field("surrealdb_password", &"***")
            .field("surrealdb_namespace", &self.surrealdb_namespace)
            .field("surrealdb_dbname", &self.surrealdb_dbname)
            .field("smtp_email", &self.smtp_email)
            .field("smtp_password", &"***")
            .field("smtp_name", &self.smtp_name)
            .field("smtp_host", &self.smtp_host)
            .field("redisdb_url", &self.redisdb_url)
            .field("fe_url", &self.fe_url)
            .field("rust_env", &self.rust_env)
            .field("minio_endpoint", &self.minio_endpoint)
            .field("minio_bucket_name", &self.minio_bucket_name)
            .field("minio_access_key", &"***")
            .field("minio_secret_key", &"***")
            .field("minio_region", &self.minio_region)
            .field("minio_secure", &self.minio_secure)
            .field("google_client_id", &self.google_client_id)
            .field("google_client_secret", &"***")
            .field("google_redirect_url", &self.google_redirect_url)
            .finish()
    }
}

/// Get environment variable with warning if not set.
///
/// This helper function attempts to read an environment variable and logs a warning
/// if it's not set, falling back to the provided default value.
///
/// # Arguments
/// * `key` - The environment variable name
/// * `default` - The default value to use if the variable is not set
///
/// # Returns
/// The environment variable value or the default
fn get_env_with_warning(key: &str, default: &str) -> String {
    match env::var(key) {
        Ok(val) => val,
        Err(_) => {
            warn!("Environment variable '{}' is not set. Using default: '{}'", key, default);
            default.to_string()
        }
    }
}

/// Parse environment variable as u16 with fallback.
///
/// # Arguments
/// * `key` - The environment variable name
/// * `default` - The default numeric value
///
/// # Returns
/// The parsed u16 value or the default if parsing fails
fn get_env_u16_with_warning(key: &str, default: u16) -> u16 {
    match env::var(key) {
        Ok(val) => val.parse().unwrap_or_else(|_| {
            warn!("Environment variable '{}' has invalid value '{}'. Using default: {}", key, val, default);
            default
        }),
        Err(_) => {
            warn!("Environment variable '{}' is not set. Using default: {}", key, default);
            default
        }
    }
}

/// Parse environment variable as bool with fallback.
///
/// # Arguments
/// * `key` - The environment variable name
/// * `default` - The default boolean value
///
/// # Returns
/// The parsed boolean value or the default if parsing fails
fn get_env_bool_with_warning(key: &str, default: bool) -> bool {
    match env::var(key) {
        Ok(val) => val.parse().unwrap_or_else(|_| {
            warn!("Environment variable '{}' has invalid value '{}'. Using default: {}", key, val, default);
            default
        }),
        Err(_) => {
            warn!("Environment variable '{}' is not set. Using default: {}", key, default);
            default
        }
    }
}

/// Global environment configuration loaded once at startup.
///
/// This static variable loads all environment configuration exactly once
/// and caches it for the lifetime of the application.
pub static ENV: Lazy<Env> = Lazy::new(|| {
    // Load .env file if present
    load_dotenv_file();

    let env = Env {
        // Server configuration
        port: get_env_u16_with_warning("PORT", 3000),

        // JWT secrets
        access_token_secret: get_env_with_warning("ACCESS_TOKEN_SECRET", "default_access_secret"),
        refresh_token_secret: get_env_with_warning("REFRESH_TOKEN_SECRET", "default_refresh_secret"),

        // SurrealDB configuration
        surrealdb_url: get_env_with_warning("SURREALDB_URL", "http://localhost:8000"),
        surrealdb_username: get_env_with_warning("SURREALDB_USERNAME", "root"),
        surrealdb_password: get_env_with_warning("SURREALDB_PASSWORD", "root"),
        surrealdb_namespace: get_env_with_warning("SURREALDB_NAMESPACE", "namespace"),
        surrealdb_dbname: get_env_with_warning("SURREALDB_DBNAME", "database"),

        // SMTP configuration
        smtp_email: get_env_with_warning("SMTP_EMAIL", "no-reply@example.com"),
        smtp_password: get_env_with_warning("SMTP_PASSWORD", "default_smtp_password"),
        smtp_name: get_env_with_warning("SMTP_NAME", "MyApp SMTP"),
        smtp_host: get_env_with_warning("SMTP_HOST", "smtp.gmail.com"),

        // Redis configuration
        redisdb_url: get_env_with_warning("REDISDB_URL", "localhost"),

        // Frontend URL
        fe_url: get_env_with_warning("FE_URL", "http://localhost"),

        // Environment
        rust_env: get_env_with_warning("RUST_ENV", "development"),

        // MinIO configuration
        minio_endpoint: get_env_with_warning("MINIO_ENDPOINT", "http://localhost:9000"),
        minio_bucket_name: get_env_with_warning("MINIO_BUCKET_NAME", "imphnen-uploads"),
        minio_access_key: get_env_with_warning("MINIO_ACCESS_KEY", "minio_access"),
        minio_secret_key: get_env_with_warning("MINIO_SECRET_KEY", "minio_secret"),
        minio_region: get_env_with_warning("MINIO_REGION", "us-east-1"),
        minio_secure: get_env_bool_with_warning("MINIO_SECURE", false),

        // Google OAuth 2.1
        google_client_id: get_env_with_warning("GOOGLE_CLIENT_ID", "default_google_client_id"),
        google_client_secret: get_env_with_warning("GOOGLE_CLIENT_SECRET", "default_google_client_secret"),
        google_redirect_url: get_env_with_warning("GOOGLE_REDIRECT_URL", "http://localhost:8000/api/v1/auth/google/callback"),
    };

    info!("Environment configuration loaded successfully");
    env
});

/// Load .env file if present, with appropriate logging.
fn load_dotenv_file() {
    match dotenvy::dotenv() {
        Ok(path) => info!("Loaded environment file: {:?}", path),
        Err(dotenvy::Error::Io(ref e)) if e.kind() == std::io::ErrorKind::NotFound => {
            warn!(".env file not found, falling back to system environment variables");
        }
        Err(e) => {
            warn!("Failed to load .env file: {}. Falling back to system environment variables", e);
        }
    }
}
