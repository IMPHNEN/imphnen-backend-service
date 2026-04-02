use axum::{
	Extension,
	body::Body,
	http::{Request, Response},
	middleware::Next,
};
use chrono::{DateTime, FixedOffset, Utc};
use imphnen_entities::seaorm::common::audit_log::Model as AuditLogSchema;
use imphnen_libs::AppState;
use imphnen_utils::{extract_email, extract_email_async, extract_real_ip};
use sea_orm::prelude::Uuid;
use sea_orm::{ActiveModelTrait, Set};
use std::convert::Infallible;

pub async fn audit_logging_middleware(
	Extension(state): Extension<AppState>,
	req: Request<Body>,
	next: Next,
) -> Result<Response<Body>, Infallible> {
	let uri = req.uri().path().to_string();

	if is_admin_action(&uri) {
		let headers = req.headers();
		let user_email = extract_user_email(headers).await;
		let user_id = extract_user_id(&state, &user_email).await;
		let ip_address =
			extract_real_ip(headers).unwrap_or_else(|| "unknown".to_string());
		let user_id_uuid =
			Uuid::parse_str(&user_id.clone().unwrap_or_else(|| "unknown".to_string()))
				.unwrap_or(Uuid::nil());
		let user_agent = extract_user_agent(headers);

		let action = extract_action(&uri, req.method().as_str());
		let resource = extract_resource(&uri);
		let resource_id = extract_resource_id(&uri);

		let audit_log = AuditLogSchema {
			id: Uuid::new_v4(),
			user_id: user_id_uuid,
			user_email: user_email.clone().unwrap_or_else(|| "unknown".to_string()),
			action,
			resource,
			resource_id,
			old_data: None,
			new_data: None,
			ip_address,
			user_agent,
			timestamp: DateTime::<FixedOffset>::from(Utc::now()),
		};

		let action = audit_log.action.clone();
		match save_audit_log(&state.postgres_connection.conn, audit_log.clone()).await {
			Ok(_) => log::debug!("Audit log saved for action: {}", action),
			Err(e) => log::error!("Failed to save audit log: {}", e),
		}
	}

	let response = next.run(req).await;
	Ok(response)
}

fn is_admin_action(uri: &str) -> bool {
	let admin_endpoints = [
		"/v1/admin/",
		"/v1/users/admin/",
		"/v1/permissions/",
		"/v1/roles/",
		"/v1/gacha/admin/",
		"/v1/cms/admin/",
	];

	admin_endpoints
		.iter()
		.any(|endpoint| uri.starts_with(endpoint))
}

async fn extract_user_email(headers: &axum::http::HeaderMap) -> Option<String> {
	match extract_email(headers) {
		Some(email) => Some(email),
		None => extract_email_async(headers).await,
	}
}

async fn extract_user_id(
	state: &AppState,
	email: &Option<String>,
) -> Option<String> {
	if let Some(email) = email {
		match state
			.auth_repository
			.get_user_for_auth(&email.clone(), state)
			.await
		{
			Ok(user) => Some(user.id.to_string()),
			Err(_) => None,
		}
	} else {
		None
	}
}

fn extract_user_agent(headers: &axum::http::HeaderMap) -> Option<String> {
	headers
		.get("user-agent")
		.and_then(|value| value.to_str().ok())
		.map(|s| s.to_string())
}

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
		}
		_ => "UNKNOWN",
	}
	.to_string()
}

fn extract_resource(uri: &str) -> String {
	if let Some(resource_part) = uri.split("/v1/").nth(1)
		&& let Some(resource) = resource_part.split('/').next()
	{
		return resource.to_string();
	}
	"unknown".to_string()
}

fn extract_resource_id(uri: &str) -> Option<String> {
	let segments = uri.split('/').collect::<Vec<&str>>();

	for segment in segments.iter().rev() {
		if (segment.len() == 36 && segment.contains('-'))
			|| segment.chars().all(|c| c.is_ascii_digit())
		{
			return Some(segment.to_string());
		}
	}

	None
}

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

pub async fn detailed_audit_logging_middleware(
	Extension(state): Extension<AppState>,
	req: Request<Body>,
	next: Next,
) -> Result<Response<Body>, Infallible> {
	audit_logging_middleware(Extension(state), req, next).await
}
