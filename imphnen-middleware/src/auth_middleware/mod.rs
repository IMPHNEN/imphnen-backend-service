use axum::{
	Extension, extract::Request, http::StatusCode, middleware::Next,
	response::Response,
};
use imphnen_libs::{AppState, jsonwebtoken::decode_access_token};
use imphnen_utils::common_response;
use axum_extra::headers::{authorization::Bearer, Authorization, HeaderMapExt};
use std::convert::Infallible;

pub async fn auth_middleware(
	Extension(_state): Extension<AppState>, // state is currently unused in this middleware
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

	req.extensions_mut().insert(claims);
	Ok(next.run(req).await)
}
