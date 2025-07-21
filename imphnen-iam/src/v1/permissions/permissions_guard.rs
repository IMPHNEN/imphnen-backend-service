use super::PermissionsEnum;
use crate::{AppState, AuthRepository, common_response, extract_email};
use axum::{
	http::{HeaderMap, StatusCode},
	response::Response,
};

pub async fn permissions_guard(
	headers: &HeaderMap,
	state: AppState,
	required_permissions: Vec<PermissionsEnum>,
) -> Result<(), Response> {
	let auth_repo = AuthRepository::new(&state);
	let email = extract_email(headers).ok_or_else(|| {
		common_response(
			StatusCode::UNAUTHORIZED,
			"Invalid or missing authorization token",
		)
	})?;
	let raw_user = auth_repo
		.query_get_stored_user(email.clone())
		.await
		.map_err(|_| {
			common_response(
				StatusCode::UNAUTHORIZED,
				"User session expired or not found",
			)
		})?;
	let role = raw_user.role;
	let role_permissions: Vec<String> =
		role.permissions.into_iter().map(|perm| perm.name).collect();

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
	Ok(())
}
