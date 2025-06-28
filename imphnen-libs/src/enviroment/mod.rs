use std::env;

pub fn load_env() {
	dotenvy::dotenv().ok();
}

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
	#[must_use]
	pub fn new() -> Self {
		Self {
			port: env::var("PORT")
				.unwrap_or_else(|_| {
					println!("INFO: PORT is not set, using default '3000'.");
					"3000".to_string()
				})
				.parse()
				.unwrap_or(3000),

			rust_env: env::var("RUST_ENV").unwrap_or_else(|_| {
				println!("INFO: RUST_ENV is not set, using default 'development'.");
				"development".to_string()
			}),

			access_token_secret: env::var("ACCESS_TOKEN_SECRET").unwrap_or_else(|_| {
				println!("WARNING: ACCESS_TOKEN_SECRET is not set, using fallback!");
				"default_access_secret".to_string()
			}),

			refresh_token_secret: env::var("REFRESH_TOKEN_SECRET").unwrap_or_else(|_| {
				println!("WARNING: REFRESH_TOKEN_SECRET is not set, using fallback!");
				"default_refresh_secret".to_string()
			}),

			surrealdb_url: env::var("SURREALDB_URL").unwrap_or_else(|_| {
				println!(
					"WARNING: SURREALDB_URL is not set, using fallback 'http://localhost:8000'!"
				);
				"http://localhost:8000".to_string()
			}),

			surrealdb_username: env::var("SURREALDB_USERNAME").unwrap_or_else(|_| {
				println!("WARNING: SURREALDB_USERNAME is not set, using fallback 'root'!");
				"root".to_string()
			}),

			surrealdb_password: env::var("SURREALDB_PASSWORD").unwrap_or_else(|_| {
				println!("WARNING: SURREALDB_PASSWORD is not set, using fallback!");
				"password".to_string()
			}),

			surrealdb_namespace: env::var("SURREALDB_NAMESPACE").unwrap_or_else(|_| {
				println!(
					"WARNING: SURREALDB_NAMESPACE is not set, using fallback 'namespace'!"
				);
				"namespace".to_string()
			}),

			surrealdb_dbname: env::var("SURREALDB_DBNAME").unwrap_or_else(|_| {
				println!("WARNING: SURREALDB_DBNAME is not set, using fallback 'database'!");
				"database".to_string()
			}),

			smtp_email: env::var("SMTP_EMAIL").unwrap_or_else(|_| {
				println!(
					"WARNING: SMTP_EMAIL is not set, using fallback 'no-reply@example.com'!"
				);
				"no-reply@example.com".to_string()
			}),

			smtp_password: env::var("SMTP_PASSWORD").unwrap_or_else(|_| {
				println!("WARNING: SMTP_PASSWORD is not set, using fallback!");
				"default_smtp_password".to_string()
			}),

			smtp_name: env::var("SMTP_NAME").unwrap_or_else(|_| {
				println!("WARNING: SMTP_NAME is not set, using fallback 'MyApp SMTP'!");
				"MyApp SMTP".to_string()
			}),

			smtp_host: env::var("SMTP_HOST").unwrap_or_else(|_| {
				println!("WARNING: SMTP_HOST is not set, using fallback 'smtp.gmail.com'!");
				"smtp.gmail.com".to_string()
			}),

			redisdb_url: env::var("REDISDB_URL").unwrap_or_else(|_| {
				println!("WARNING: REDISDB_URL is not set, using fallback 'localhost'!");
				"localhost".to_string()
			}),

			fe_url: env::var("FE_URL").unwrap_or_else(|_| {
				println!("WARNING: FE_URL is not set, using fallback 'http://localhost'!");
				"http://localhost".to_string()
			}),

			minio_endpoint: env::var("MINIO_ENDPOINT").unwrap_or_else(|_| {
				println!(
					"WARNING: MINIO_ENDPOINT is not set, using fallback 'http://localhost:9000'!"
				);
				"http://localhost:9000".to_string()
			}),

			minio_bucket_name: env::var("MINIO_BUCKET_NAME").unwrap_or_else(|_| {
				println!(
					"WARNING: MINIO_BUCKET_NAME is not set, using fallback 'default_bucket'!"
				);
				"default_bucket".to_string()
			}),

			minio_access_key: env::var("MINIO_ACCESS_KEY").unwrap_or_else(|_| {
				println!("WARNING: MINIO_ACCESS_KEY is not set, using fallback!");
				"minio_access".to_string()
			}),

			minio_secret_key: env::var("MINIO_SECRET_KEY").unwrap_or_else(|_| {
				println!("WARNING: MINIO_SECRET_KEY is not set, using fallback!");
				"minio_secret".to_string()
			}),
		}
	}
}

impl Default for Env {
	fn default() -> Self {
		Self::new()
	}
}
