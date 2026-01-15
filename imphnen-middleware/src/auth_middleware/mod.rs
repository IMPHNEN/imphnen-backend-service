use axum::{
	Extension, extract::Request, http::StatusCode, middleware::Next,
	response::Response,
};
use imphnen_libs::{AppState, jsonwebtoken::decode_access_token};
use axum_extra::headers::{authorization::Bearer, Authorization, HeaderMapExt};
use std::convert::Infallible;
use uuid::Uuid;
use imphnen_utils::response_format::common_response;

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

	// Validate UUID format
	let user_uuid = match Uuid::parse_str(&user_id) {
		Ok(uuid) => uuid,
		Err(_) => return Ok(common_response(
			StatusCode::UNAUTHORIZED,
			"Invalid user identifier format",
		)),
	};

	// Use UserLookupService to fetch full user details including roles/permissions
    // This ensures consistency and populates the DTO expected by controllers
    let user_info = match state.user_lookup_service.get_user_by_id(user_uuid, &state).await {
        Ok(info) => info,
        Err(_) => return Ok(common_response(StatusCode::UNAUTHORIZED, "User not found or inactive")),
    };

    // Insert the Model (reconstructed or fetched? Wait, UserLookupService returns ExtendedUserInfo)
    // We need to insert what the controllers expect.
    // Some controllers might expect Model, others DTO.
    // Let's fetch Model separately if needed, or better, insert DTO.
    // The error said "Extension of type `imphnen_entities::users::UsersDetailQueryDto` was not found".
    
    req.extensions_mut().insert(user_info.basic_info);
    // If controllers also need Model, we might need to insert it too. 
    // But usually they switch to DTO. 
    // Let's try inserting DTO first.
    
	Ok(next.run(req).await)
}
