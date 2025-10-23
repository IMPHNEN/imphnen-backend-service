use axum::{
	Extension, extract::Request, http::StatusCode, middleware::Next,
	response::Response,
};
use imphnen_libs::{AppState, jsonwebtoken::decode_access_token};
use imphnen_entities::UsersDetailQueryDto;
use imphnen_utils::common_response;
use axum_extra::headers::{authorization::Bearer, Authorization, HeaderMapExt};
use std::convert::Infallible;
use imphnen_libs::ResourceEnum;
use imphnen_utils::make_thing;

pub async fn auth_middleware(
	Extension(state): Extension<AppState>,
	mut req: Request,
	next: Next,
) -> Result<Response, Infallible> {
	let auth_header = match req
		.headers()
		.typed_get::<Authorization<Bearer>>() {
            Some(header) => header,
            None => return Ok(common_response(
                StatusCode::UNAUTHORIZED,
                "Invalid or missing authorization token",
            )),
        };

	let token = auth_header.token();

	let claims = match decode_access_token(token) {
        Ok(token_data) => token_data.claims,
        Err(_) => return Ok(common_response(
            StatusCode::UNAUTHORIZED,
            "Invalid or expired token",
        )),
    };

	let user_id = claims.user_id.clone();

	let thing_id = make_thing(&ResourceEnum::Users.to_string(), &user_id);

	// Try SurrealDB mem first
	let mem_db = &state.surrealdb_mem;
	let user_data = if let Ok(Some(user)) = mem_db.select::<Option<UsersDetailQueryDto>>(("users", &user_id)).await {
		if !user.is_deleted && !user.role.is_deleted {
			Some(user)
		} else {
			None
		}
	} else {
		None
	};

	// Fallback to main DB if not found in mem
	let user_data = if let Some(user) = user_data {
		user
	} else {
		match state.user_lookup_service.get_user_by_id_internal(&thing_id, &state).await {
			Ok(user) => {
				// Cache in mem for future requests with retry logic
				let mut retry_count = 0;
				const MAX_RETRIES: u8 = 3;
				
				while retry_count < MAX_RETRIES {
					match mem_db.update::<Option<UsersDetailQueryDto>>(("users", &user_id)).content(user.clone()).await {
						Ok(_) => {
							log::debug!("User {} cached successfully", user_id);
							break;
						}
						Err(e) => {
							retry_count += 1;
							log::warn!(
								"Failed to cache user {} (attempt {}/{}): {}",
								user_id, retry_count, MAX_RETRIES, e
							);
							if retry_count < MAX_RETRIES {
								tokio::time::sleep(tokio::time::Duration::from_millis(50 * retry_count as u64)).await;
							} else {
								log::error!("Failed to cache user {} after {} retries", user_id, MAX_RETRIES);
							}
						}
					}
				}
				
				user
			},
			Err(_) => return Ok(common_response(StatusCode::UNAUTHORIZED, "User not found")),
		}
	};

	req.extensions_mut().insert(user_data);
	Ok(next.run(req).await)
}
