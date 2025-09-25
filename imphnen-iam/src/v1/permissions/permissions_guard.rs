use super::PermissionsEnum;
use crate::{AppState, common_response, decode_access_token, UsersRepository};
use axum::{
	http::{HeaderMap, StatusCode},
	response::Response, Extension,
};
use axum_extra::headers::{authorization::Bearer, Authorization, HeaderMapExt};

pub async fn permissions_guard(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	required_permissions: Vec<PermissionsEnum>,
) -> Result<(imphnen_libs::jsonwebtoken::Claims, AppState), Response> {
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

	// Fetch user from database to get permissions
	let user_repo = UsersRepository::new(&state);
	let user = match user_repo.query_user_by_email(claims.sub.clone()).await {
		Ok(user) => user,
		Err(_) => {
			return Err(common_response(
				StatusCode::UNAUTHORIZED,
				"User not found",
			));
		}
	};

	// Check permissions from database
	let user_permissions: Vec<String> = user.role.permissions.as_ref().unwrap_or(&vec![]).iter().filter_map(|p| p.as_ref().and_then(|pp| pp.name.clone())).collect();
	for required in &required_permissions {
		let required_str = required.to_string();
		if !user_permissions.contains(&required_str) {
			eprintln!("  MISSING REQUIRED PERMISSION: {required_str}");
			return Err(common_response(
				StatusCode::FORBIDDEN,
				"You don't have the required permissions",
			));
		}
	}

	Ok((claims, state))
}
