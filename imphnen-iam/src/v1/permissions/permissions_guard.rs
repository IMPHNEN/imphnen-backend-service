use super::PermissionsEnum;
use crate::{AppState, common_response, decode_access_token};
use axum::{
	http::{HeaderMap, StatusCode},
	response::Response, Extension,
};
use axum_extra::headers::{authorization::Bearer, Authorization, HeaderMapExt};
// Removed imphnen_utils::make_thing as it's no longer needed here

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

	Ok((claims, state))
}
