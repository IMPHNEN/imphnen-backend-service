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
        port: match env::var("PORT") {
            Ok(val) => val.parse().unwrap_or(3000),
            Err(_) => {
                warn!("Environment variable PORT not set, using default 3000.");
                3000
            }
        },

        rust_env: match env::var("RUST_ENV") {
            Ok(val) => val,
            Err(_) => {
                warn!("Environment variable RUST_ENV not set, using default 'development'.");
                "development".to_string()
            }
        },

        access_token_secret: match env::var("ACCESS_TOKEN_SECRET") {
            Ok(val) => val,
            Err(_) => {
                warn!("Environment variable ACCESS_TOKEN_SECRET not set, using default.");
                "default_access_secret".to_string()
            }
        },

        refresh_token_secret: match env::var("REFRESH_TOKEN_SECRET") {
            Ok(val) => val,
            Err(_) => {
                warn!("Environment variable REFRESH_TOKEN_SECRET not set, using default.");
                "default_refresh_secret".to_string()
            }
        },

        surrealdb_url: match env::var("SURREALDB_URL") {
            Ok(val) => val,
            Err(_) => {
                warn!("Environment variable SURREALDB_URL not set, using default 'http://localhost:8000'.");
                "http://localhost:8000".to_string()
            }
        },

        surrealdb_username: match env::var("SURREALDB_USERNAME") {
            Ok(val) => val,
            Err(_) => {
                warn!("Environment variable SURREALDB_USERNAME not set, using default 'root'.");
                "root".to_string()
            }
        },

        surrealdb_password: match env::var("SURREALDB_PASSWORD") {
            Ok(val) => val,
            Err(_) => {
                warn!("Environment variable SURREALDB_PASSWORD not set, using default 'password'.");
                "password".to_string()
            }
        },

        surrealdb_namespace: match env::var("SURREALDB_NAMESPACE") {
            Ok(val) => val,
            Err(_) => {
                warn!("Environment variable SURREALDB_NAMESPACE not set, using default 'namespace'.");
                "namespace".to_string()
            }
        },

        surrealdb_dbname: match env::var("SURREALDB_DBNAME") {
            Ok(val) => val,
            Err(_) => {
                warn!("Environment variable SURREALDB_DBNAME not set, using default 'database'.");
                "database".to_string()
            }
        },

        surrealdb_url_ws: match env::var("SURREALDB_URL_WS") {
            Ok(val) => val,
            Err(_) => {
                warn!("Environment variable SURREALDB_URL_WS not set, using default 'ws://localhost:8000/rpc'.");
                "ws://localhost:8000/rpc".to_string()
            }
        },

        smtp_email: match env::var("SMTP_EMAIL") {
            Ok(val) => val,
            Err(_) => {
                warn!("Environment variable SMTP_EMAIL not set, using default 'no-reply@example.com'.");
                "no-reply@example.com".to_string()
            }
        },

        smtp_password: match env::var("SMTP_PASSWORD") {
            Ok(val) => val,
            Err(_) => {
                warn!("Environment variable SMTP_PASSWORD not set, using default.");
                "default_smtp_password".to_string()
            }
        },

        smtp_name: match env::var("SMTP_NAME") {
            Ok(val) => val,
            Err(_) => {
                warn!("Environment variable SMTP_NAME not set, using default 'MyApp SMTP'.");
                "MyApp SMTP".to_string()
            }
        },

        smtp_host: match env::var("SMTP_HOST") {
            Ok(val) => val,
            Err(_) => {
                warn!("Environment variable SMTP_HOST not set, using default 'smtp.gmail.com'.");
                "smtp.gmail.com".to_string()
            }
        },

        redisdb_url: match env::var("REDISDB_URL") {
            Ok(val) => val,
            Err(_) => {
                warn!("Environment variable REDISDB_URL not set, using default 'localhost'.");
                "localhost".to_string()
            }
        },

        fe_url: match env::var("FE_URL") {
            Ok(val) => val,
            Err(_) => {
                warn!("Environment variable FE_URL not set, using default 'http://localhost'.");
                "http://localhost".to_string()
            }
        },

        minio_endpoint: match env::var("MINIO_ENDPOINT") {
            Ok(val) => val,
            Err(_) => {
                warn!("Environment variable MINIO_ENDPOINT not set, using default 'http://localhost:9000'.");
                "http://localhost:9000".to_string()
            }
        },

        minio_bucket_name: match env::var("MINIO_BUCKET_NAME") {
            Ok(val) => val,
            Err(_) => {
                warn!("Environment variable MINIO_BUCKET_NAME not set, using default 'default_bucket'.");
                "default_bucket".to_string()
            }
        },

        minio_access_key: match env::var("MINIO_ACCESS_KEY") {
            Ok(val) => val,
            Err(_) => {
                warn!("Environment variable MINIO_ACCESS_KEY not set, using default 'minio_access'.");
                "minio_access".to_string()
            }
        },

        minio_secret_key: match env::var("MINIO_SECRET_KEY") {
            Ok(val) => val,
            Err(_) => {
                warn!("Environment variable MINIO_SECRET_KEY not set, using default 'minio_secret'.");
                "minio_secret".to_string()
            }
        },
    }
});
