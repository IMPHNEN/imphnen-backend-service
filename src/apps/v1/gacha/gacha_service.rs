use super::{
	GachaCreateClaimRequestDto, GachaCreateItemRequestDto, GachaCreateRollRequestDto,
	GachaRepository,
};
use crate::{common_response, extract_email, AppState};
use axum::{
	http::{HeaderMap, StatusCode},
	response::Response,
};

pub struct GachaService;

impl GachaService {
	pub async fn mutation_create_gacha_claim(
		payload: GachaCreateClaimRequestDto,
		state: &AppState,
		header: HeaderMap,
	) -> Response {
		let repository = GachaRepository::new(state);

		let email = match extract_email(&header) {
			Some(email) => email,
			None => {
				return common_response(
					StatusCode::UNAUTHORIZED,
					"Invalid or expired token",
				);
			}
		};

		match repository.query_create_gacha_claim(payload, email).await {
			Ok(msg) => common_response(StatusCode::CREATED, &msg),
			Err(err) => common_response(StatusCode::BAD_REQUEST, &err.to_string()),
		}
	}

	pub async fn mutation_create_gacha_item(
		payload: GachaCreateItemRequestDto,
		state: &AppState,
	) -> Response {
		let repository = GachaRepository::new(state);

		match repository.query_create_gacha_item(payload).await {
			Ok(msg) => common_response(StatusCode::CREATED, &msg),
			Err(err) => common_response(StatusCode::BAD_REQUEST, &err.to_string()),
		}
	}

	pub async fn mutation_create_gacha_roll(
		payload: GachaCreateRollRequestDto,
		state: &AppState,
	) -> Response {
		let repository = GachaRepository::new(state);

		match repository.query_create_gacha_roll(payload).await {
			Ok(msg) => common_response(StatusCode::CREATED, &msg),
			Err(err) => common_response(StatusCode::BAD_REQUEST, &err.to_string()),
		}
	}
}
