use axum::{
	Extension,
	body::Body,
	http::{Request, Response, StatusCode},
	middleware::Next,
};
use chrono::{DateTime, Duration, FixedOffset, Utc};
use imphnen_entities::seaorm::common::rate_limit::ActiveModel as RateLimitActiveModel;
use imphnen_entities::seaorm::common::rate_limit::Column as RateLimitColumn;
use imphnen_entities::seaorm::common::rate_limit::Entity as RateLimitEntity;
use imphnen_libs::AppState;
use imphnen_utils::extract_real_ip;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

pub async fn rate_limiting_middleware(
	Extension(state): Extension<AppState>,
	req: Request<axum::body::Body>,
	next: Next,
) -> Result<Response<Body>, StatusCode> {
	let uri = req.uri().path().to_string();

	if is_public_endpoint(&uri) {
		let client_ip = extract_real_ip(req.headers()).unwrap_or_else(|| {
			log::warn!("Could not extract real IP, using fallback");
			"unknown".to_string()
		});

		let max_requests = 100;
		let window_duration_secs = 60;

		match check_rate_limit(
			&state.postgres_connection.conn,
			&client_ip,
			max_requests,
			window_duration_secs,
		)
		.await
		{
			Ok(is_limited) => {
				if is_limited {
					return Ok(
						Response::builder()
							.status(StatusCode::TOO_MANY_REQUESTS)
							.header("Retry-After", "60")
							.body("Too Many Requests: Rate limit exceeded".into())
							.expect("valid rate limit response"),
					);
				}
			}
			Err(e) => {
				log::error!("Rate limit check failed: {}", e);
			}
		}
	}

	Ok(next.run(req).await)
}

pub async fn auth_rate_limiting_middleware(
	Extension(state): Extension<AppState>,
	req: Request<axum::body::Body>,
	next: Next,
) -> Result<Response<Body>, StatusCode> {
	let uri = req.uri().path().to_string();

	if uri == "/v1/auth/login" || uri == "/v1/auth/register" {
		let client_ip = extract_real_ip(req.headers()).unwrap_or_else(|| {
			log::warn!("Could not extract real IP, using fallback");
			"unknown".to_string()
		});

		let max_requests = 10;
		let window_duration_secs = 60;

		match check_rate_limit(
			&state.postgres_connection.conn,
			&client_ip,
			max_requests,
			window_duration_secs,
		)
		.await
		{
			Ok(is_limited) => {
				if is_limited {
					return Ok(
						Response::builder()
							.status(StatusCode::TOO_MANY_REQUESTS)
							.header("Retry-After", "60")
							.body(
								"Too Many Requests: Rate limit exceeded for authentication endpoint"
									.into(),
							)
							.expect("valid auth rate limit response"),
					);
				}
			}
			Err(e) => {
				log::error!("Auth rate limit check failed: {}", e);
			}
		}
	}

	Ok(next.run(req).await)
}

fn is_public_endpoint(uri: &str) -> bool {
	let public_endpoints = [
		"/v1/auth/login",
		"/v1/auth/register",
		"/v1/auth/refresh",
		"/v1/auth/logout",
		"/v1/gacha/roll",
		"/v1/gacha/credits",
		"/v1/cms/landing",
	];

	public_endpoints
		.iter()
		.any(|endpoint| uri.starts_with(endpoint))
}

async fn check_rate_limit(
	db: &sea_orm::DatabaseConnection,
	ip_address: &str,
	max_requests: u32,
	window_duration_secs: u64,
) -> Result<bool, Box<dyn std::error::Error>> {
	let now = Utc::now();
	let window_start = now - Duration::seconds(window_duration_secs as i64);

	let existing_record = RateLimitEntity::find()
		.filter(RateLimitColumn::IpAddress.eq(ip_address))
		.one(db)
		.await?;

	match existing_record {
		Some(record) => {
			let mut active_model: RateLimitActiveModel = record.into();

			let last_request = active_model
				.last_request_time
				.clone()
				.take()
				.unwrap_or(DateTime::<FixedOffset>::from(window_start));
			let was_reset = if last_request <= window_start {
				active_model.request_count = Set(0);
				active_model.last_request_time = Set(DateTime::<FixedOffset>::from(now));
				true
			} else {
				false
			};

			if !was_reset {
				let current_count = active_model.request_count.clone().take().unwrap_or(0);
				active_model.request_count = Set(current_count + 1);
			}

			let updated_model = active_model.update(db).await?;

			Ok(updated_model.request_count > max_requests)
		}
		None => {
			let new_record = RateLimitActiveModel {
				id: Set(Uuid::new_v4().to_string()),
				ip_address: Set(ip_address.to_string()),
				request_count: Set(1),
				first_request_time: Set(DateTime::<FixedOffset>::from(now)),
				last_request_time: Set(DateTime::<FixedOffset>::from(now)),
				window_duration_secs: Set(window_duration_secs as i64),
			};

			new_record.insert(db).await?;

			Ok(false)
		}
	}
}
