use super::PermissionsEnum;
use crate::{AppState, common_response, decode_access_token, UsersDetailQueryDto, UsersRepository};
use axum::{
	http::{HeaderMap, StatusCode},
	response::Response, Extension,
};
use axum_extra::headers::{authorization::Bearer, Authorization, HeaderMapExt};
use imphnen_utils::make_thing;

pub async fn permissions_guard(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	required_permissions: Vec<PermissionsEnum>,
) -> Result<(UsersDetailQueryDto, AppState), Response> {
	let auth_header = headers
		.typed_get::<Authorization<Bearer>>()
		.ok_or_else(|| {
			common_response(
				StatusCode::UNAUTHORIZED,
				"Invalid or missing authorization token",
			)
		})?;

	let token = auth_header.token();

	let claims = decode_access_token(token)
		.map_err(|_| {
			common_response(
				StatusCode::UNAUTHORIZED,
				"Invalid or expired token",
			)
		})?
		.claims;

	// Use permissions from JWT for the check
	for required in &required_permissions {
		let required_str = required.to_string();
		if !claims.permissions.contains(&required_str) {
			eprintln!("  MISSING REQUIRED PERMISSION: {required_str}");
			return Err(common_response(
				StatusCode::FORBIDDEN,
				"You don't have the required permissions",
			));
		}
	}

	// Fetch full user details from the database using user_id from JWT
	let user_repo = UsersRepository::new(&state);
	let user_id_thing = make_thing("app_users", &claims.user_id);
	let raw_user = user_repo.query_user_by_id(&user_id_thing)
		.await
		.map_err(|_| {
			common_response(
				StatusCode::INTERNAL_SERVER_ERROR, // Changed to internal server error as user ID should be valid from JWT
				"Failed to retrieve user details",
			)
		})?;
	
	Ok((raw_user, state))
}
