use crate::{AppState, decode_access_token};
use axum::{Extension, http::HeaderMap};
use axum_extra::headers::{Authorization, HeaderMapExt, authorization::Bearer};
use imphnen_entities::PermissionsEnum;
use imphnen_utils::AppError;
use uuid::Uuid;

pub async fn permissions_guard(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	required_permissions: Vec<PermissionsEnum>,
) -> Result<(imphnen_libs::jsonwebtoken::Claims, AppState), AppError> {
	let auth_header =
		headers
			.typed_get::<Authorization<Bearer>>()
			.ok_or_else(|| {
				AppError::AuthenticationError(
					"Invalid or missing authorization token".to_string(),
				)
			})?;

	let token = auth_header.token();

	let claims = decode_access_token(token)
		.map_err(|_| {
			AppError::AuthenticationError("Invalid or expired token".to_string())
		})?
		.claims;

	let user_info = {
		let by_email = state
			.user_lookup_service
			.get_user_by_email(&claims.sub, &state)
			.await;
		match by_email {
			Ok(info) => info,
			Err(_) => {
				let user_id = Uuid::parse_str(&claims.sub).map_err(|_| {
					AppError::AuthenticationError("Invalid user ID format".to_string())
				})?;
				state
					.user_lookup_service
					.get_user_by_id(user_id, &state)
					.await
					.map_err(|_| AppError::AuthenticationError("User not found".to_string()))?
			}
		}
	};

	let user_permissions: Vec<String> = user_info
		.basic_info
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

	let admin_name = PermissionsEnum::Administrator.to_string();
	let admin_id = PermissionsEnum::Administrator.id();

	if user_permissions.contains(&admin_name) || user_permissions.contains(&admin_id) {
		return Ok((claims, state));
	}

	for required in &required_permissions {
		let required_str = required.to_string();
		let required_id = required.id();
		if !user_permissions.contains(&required_str)
			&& !user_permissions.contains(&required_id)
		{
			return Err(AppError::ForbiddenError(
				"You don't have the required permissions".to_string(),
			));
		}
	}

	Ok((claims, state))
}
