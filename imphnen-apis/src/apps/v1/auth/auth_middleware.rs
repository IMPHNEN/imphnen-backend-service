use crate::{
	common_response, extract_email, AppState, UsersItemDtoRaw, UsersRepository,
};
use axum::{
	extract::Request, http::StatusCode, middleware::Next, response::Response,
	Extension,
};
use std::convert::Infallible;

pub async fn auth_middleware(
	Extension(state): Extension<AppState>,
	mut req: Request,
	next: Next,
) -> Result<Response, Infallible> {
	let headers = req.headers();

	let email = match extract_email(headers) {
		Some(email) => email,
		None => {
			return Ok(common_response(
				StatusCode::UNAUTHORIZED,
				"Invalid or expired token",
			));
		}
	};

	let repository = UsersRepository::new(&state);

	let user: Option<UsersItemDtoRaw> =
		match repository.query_user_by_email(email).await {
			Ok(user) => Some(user),
			Err(err) => {
				return Ok(common_response(
					StatusCode::INTERNAL_SERVER_ERROR,
					&err.to_string(),
				))
			}
		};

	if user.is_none() {
		return Ok(common_response(
			StatusCode::UNAUTHORIZED,
			"Unauthorized user",
		));
	}

	req.extensions_mut().insert(user.unwrap());

	Ok(next.run(req).await)
}
