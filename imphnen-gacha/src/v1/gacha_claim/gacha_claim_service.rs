use crate::{
	AppState, GachaClaimItemDto, GachaClaimRepository, GachaClaimRequestDto,
	GachaClaimSchema, ResponseSuccessDto, common_response, success_response,
	validate_request,
};
use axum::http::StatusCode;
use axum::response::Response;

pub struct GachaClaimService;

impl GachaClaimService {
	pub async fn get_gacha_claim_by_id(state: &AppState, id: String) -> Response {
		let repo = GachaClaimRepository::new(state);
		match repo.query_gacha_claim_by_id(id).await {
			Ok(claim) => success_response(ResponseSuccessDto {
				data: GachaClaimItemDto::from(&claim),
			}),
			Err(e) => common_response(StatusCode::NOT_FOUND, &e.to_string()),
		}
	}

	pub async fn create_gacha_claim(
		state: &AppState,
		payload: GachaClaimRequestDto,
	) -> Response {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}
		let repo = GachaClaimRepository::new(state);
		let schema = GachaClaimSchema::from(payload);
		match repo.query_create_gacha_claim(schema).await {
			Ok(msg) => common_response(StatusCode::CREATED, &msg),
			Err(e) => common_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
		}
	}
}
