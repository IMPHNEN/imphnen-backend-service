use super::PermissionsEnum;
use crate::{AppState, common_response, decode_access_token, UsersRepository};
use axum::{
	http::{HeaderMap, StatusCode},
	response::Response, Extension,
};
use axum_extra::headers::{authorization::Bearer, Authorization, HeaderMapExt};
use uuid::Uuid;

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

	// Fetch user from database to get permissions. Try email first, then try using the sub as a user id.
	let user_repo = UsersRepository::new(&state);
	let user = match user_repo.query_user_by_email(claims.sub.clone()).await {
		Ok(u) => u,
		Err(_) => {
			// Try treat claims.sub as a UUID (user id)
			let user_id = Uuid::parse_str(&claims.sub).map_err(|_| {
				common_response(StatusCode::UNAUTHORIZED, "Invalid user ID format")
			})?;
			match user_repo.query_user_by_id(&user_id.to_string()).await {
				Ok(u2) => u2,
				Err(_) => {
					return Err(common_response(
						StatusCode::UNAUTHORIZED,
						"User not found",
					));
				}
			}
		}
	};

	// Check permissions from database: collect both names and raw ids so checks
	// succeed whether permissions are stored by name or by UUID.
	let user_permissions: Vec<String> = user
		.role
		.permissions
		.as_ref()
		.unwrap_or(&vec![])
		.iter()
		.filter_map(|p| p.as_ref())
		.flat_map(|pp| {
			let mut res: Vec<String> = Vec::new();
			if let Some(name) = pp.name.clone() {
				res.push(name);
			}
			if let Some(id) = pp.id.as_ref().map(|id| id.to_string()) {
				res.push(id);
			}
			res
		})
		.collect();


	// If user has Administrator permission, allow all.
	// Accept either the permission name or the canonical permission id.
	let admin_name = PermissionsEnum::Administrator.to_string();
    let admin_id = PermissionsEnum::Administrator.id();
    
	if user_permissions.contains(&admin_name) || user_permissions.contains(&admin_id) {
		return Ok((claims, state));
	}

	for required in &required_permissions {
		let required_str = required.to_string();
        let required_id = required.id();
        
		if !user_permissions.contains(&required_str) && !user_permissions.contains(&required_id) {
			eprintln!("  MISSING REQUIRED PERMISSION: {required_str} (ID: {required_id})");
            eprintln!("  USER PERMISSIONS: {:?}", user_permissions);
			return Err(common_response(
				StatusCode::FORBIDDEN,
				"You don't have the required permissions",
			));
		}
	}

	Ok((claims, state))
}
