use std::env;

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
}

impl Env {
	pub fn new() -> Self {
		Self {
			port: env::var("PORT")
				.unwrap_or_else(|_| "3000".to_string())
				.parse()
				.unwrap_or(3000),
			access_token_secret: env::var("ACCESS_TOKEN_SECRET")
				.unwrap_or_else(|_| "default_access_secret".to_string()),
			refresh_token_secret: env::var("REFRESH_TOKEN_SECRET")
				.unwrap_or_else(|_| "default_refresh_secret".to_string()),
			surrealdb_url: env::var("SURREALDB_URL")
				.unwrap_or_else(|_| "http://localhost:8000".to_string()),
			surrealdb_username: env::var("SURREALDB_USERNAME")
				.unwrap_or_else(|_| "root".to_string()),
			surrealdb_password: env::var("SURREALDB_PASSWORD")
				.unwrap_or_else(|_| "password".to_string()),
			surrealdb_namespace: env::var("SURREALDB_NAMESPACE")
				.unwrap_or_else(|_| "namespace".to_string()),
			surrealdb_dbname: env::var("SURREALDB_DBNAME")
				.unwrap_or_else(|_| "database".to_string()),
			smtp_email: env::var("SMTP_EMAIL")
				.unwrap_or_else(|_| "no-reply@example.com".to_string()),
			smtp_password: env::var("SMTP_PASSWORD")
				.unwrap_or_else(|_| "default_smtp_password".to_string()),
			smtp_name: env::var("SMTP_NAME").unwrap_or_else(|_| "MyApp SMTP".to_string()),
			smtp_host: env::var("SMTP_HOST")
				.unwrap_or_else(|_| "smtp.gmail.com".to_string()),
			redisdb_url: env::var("REDISDB_URL")
				.unwrap_or_else(|_| "localhost".to_string()),
			fe_url: env::var("FE_URL").unwrap_or_else(|_| "http://localhost".to_string()),
			rust_env: env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string()),
			minio_endpoint: env::var("MINIO_ENDPOINT")
				.unwrap_or_else(|_| "http://localhost:9000".to_string()),
			minio_bucket_name: env::var("MINIO_BUCKET_NAME")
				.unwrap_or_else(|_| "default_bucket".to_string()),
			minio_access_key: env::var("MINIO_ACCESS_KEY")
				.unwrap_or_else(|_| "minio_access".to_string()),
			minio_secret_key: env::var("MINIO_SECRET_KEY")
				.unwrap_or_else(|_| "minio_secret".to_string()),
		}
	}
}
