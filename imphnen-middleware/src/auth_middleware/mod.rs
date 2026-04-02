use axum::{
	Extension,
	extract::Request,
	http::StatusCode,
	middleware::Next,
	response::{IntoResponse, Response},
};
use axum_extra::headers::{Authorization, HeaderMapExt, authorization::Bearer};
use imphnen_libs::{AppState, jsonwebtoken::decode_access_token};
use imphnen_utils::response_format::ApiMessage;
use std::convert::Infallible;
use uuid::Uuid;

pub async fn auth_middleware(
	Extension(state): Extension<AppState>,
	mut req: Request,
	next: Next,
) -> Result<Response, Infallible> {
	let auth_header = match req.headers().typed_get::<Authorization<Bearer>>() {
		Some(header) => header,
		None => {
			return Ok(
				ApiMessage::new(
					StatusCode::UNAUTHORIZED,
					"Invalid or missing authorization token",
				)
				.into_response(),
			);
		}
	};

	let token = auth_header.token();

	let claims = match decode_access_token(token) {
		Ok(token_data) => token_data.claims,
		Err(_) => {
			return Ok(
				ApiMessage::new(StatusCode::UNAUTHORIZED, "Invalid or expired token")
					.into_response(),
			);
		}
	};

	let user_id = claims.user_id.clone();

	let user_uuid = match Uuid::parse_str(&user_id) {
		Ok(uuid) => uuid,
		Err(_) => {
			return Ok(
				ApiMessage::new(StatusCode::UNAUTHORIZED, "Invalid user identifier format")
					.into_response(),
			);
		}
	};

	let user_info = match state
		.user_lookup_service
		.get_user_by_id(user_uuid, &state)
		.await
	{
		Ok(info) => info,
		Err(_) => {
			return Ok(
				ApiMessage::new(StatusCode::UNAUTHORIZED, "User not found or inactive")
					.into_response(),
			);
		}
	};

	req.extensions_mut().insert(user_info.basic_info);

	Ok(next.run(req).await)
}
