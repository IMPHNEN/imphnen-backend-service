use axum::{
	Extension, extract::Request, http::StatusCode, middleware::Next,
	response::Response,
};
use imphnen_libs::{AppState, jsonwebtoken::decode_access_token};
use imphnen_iam::v1::users::users_dto::UsersDetailQueryDto;
use imphnen_utils::common_response;
use axum_extra::headers::{authorization::Bearer, Authorization, HeaderMapExt};
use std::convert::Infallible;
use imphnen_iam::v1::users::{users_service::{UsersService, UsersServiceTrait}};
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
	let mut user_data: Option<UsersDetailQueryDto> = None;
	if let Ok(opt_user) = mem_db.select(("users", &user_id)).await {
		if let Some(user) = opt_user {
			let user: imphnen_iam::v1::users::users_dto::UsersDetailQueryDto = user;
			if !user.is_deleted && !user.role.is_deleted {
				user_data = Some(user);
			}
		}
	}

	// Fallback to main DB if not found in mem
	let user_data = match user_data {
		Some(user) => user,
		None => {
			let repo = UsersService {};
			match repo.get_user_by_id_internal(&thing_id, &state).await {
				Ok(user) => {
					// Optionally: insert into mem for future requests
					let _: Result<Option<UsersDetailQueryDto>, _> = mem_db.update(("users", &user_id)).content(user.clone()).await;
					user
				},
				Err(_) => return Ok(common_response(StatusCode::UNAUTHORIZED, "User not found")),
			}
		}
	};

	req.extensions_mut().insert(user_data);
	Ok(next.run(req).await)
}
