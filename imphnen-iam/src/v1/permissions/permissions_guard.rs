use super::PermissionsEnum;
use crate::{AppState, AuthRepository, common_response, extract_email, extract_email_async, UsersDetailQueryDto};
use axum::{
	http::{HeaderMap, StatusCode},
	response::Response,
};

pub async fn permissions_guard(
	headers: &HeaderMap,
	state: AppState,
	required_permissions: Vec<PermissionsEnum>,
) -> Result<UsersDetailQueryDto, Response> {
	let auth_repo = AuthRepository::new(&state);
	
	// Try synchronous email extraction first (for internal JWT tokens)
	let email = match extract_email(headers) {
		Some(email) => email,
		None => {
			// If sync extraction fails, try async (for Google tokens)
			match extract_email_async(headers).await {
				Some(email) => email,
				None => {
					return Err(common_response(
						StatusCode::UNAUTHORIZED,
						"Invalid or missing authorization token",
					));
				}
			}
		}
	};
	
	let raw_user = auth_repo
		.query_get_stored_user(email.clone())
		.await
		.map_err(|_| {
			common_response(
				StatusCode::UNAUTHORIZED,
				"User session expired or not found",
			)
		})?;
	let role_permissions: Vec<String> =
		raw_user.role.permissions.iter().map(|perm| perm.name.clone()).collect();

	for required in &required_permissions {
		let required_str = required.to_string();
		if !role_permissions.contains(&required_str) {
			eprintln!("  MISSING REQUIRED PERMISSION: {required_str}");
			return Err(common_response(
				StatusCode::FORBIDDEN,
				"You don't have the required permissions",
			));
		}
	}
	Ok(raw_user)
}
