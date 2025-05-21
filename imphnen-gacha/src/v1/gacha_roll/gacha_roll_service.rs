use crate::{
	AppState, GachaClaimRepository, GachaClaimSchema, GachaRollItemDto,
	GachaRollRepository, GachaRollRequestDto, GachaRollSchema, ResponseSuccessDto,
	common_response, success_response, validate_request,
};
use axum::http::{HeaderMap, StatusCode};
use axum::response::Response;
use imphnen_iam::{UsersRepository, extract_email};

pub struct GachaRollService;

impl GachaRollService {
	pub async fn get_gacha_roll_by_id(state: &AppState, id: String) -> Response {
		let repo = GachaRollRepository::new(state);
		match repo.query_gacha_roll_by_id(id).await {
			Ok(roll) => success_response(ResponseSuccessDto {
				data: GachaRollItemDto::from(&roll),
			}),
			Err(e) => common_response(StatusCode::NOT_FOUND, &e.to_string()),
		}
	}

	pub async fn create_gacha_roll(
		state: &AppState,
		payload: GachaRollRequestDto,
	) -> Response {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}
		let schema = GachaRollSchema::create(payload);
		let repo = GachaRollRepository::new(state);
		match repo.query_create_gacha_roll(schema).await {
			Ok(msg) => common_response(StatusCode::CREATED, &msg),
			Err(e) => common_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
		}
	}

	pub async fn execute_roll_once(headers: HeaderMap, state: &AppState) -> Response {
		let repo = GachaRollRepository::new(state);
		let repo_claim = GachaClaimRepository::new(state);
		let repo_user = UsersRepository::new(state);
		let Some(email) = extract_email(&headers) else {
			return common_response(StatusCode::UNAUTHORIZED, "Unauthorized");
		};
		let Ok(user) = repo_user.query_user_by_email(email.to_string()).await else {
			return common_response(StatusCode::NOT_FOUND, "User not found");
		};
		match repo.query_all_active_rolls().await {
			Ok(rolls) => match GachaRollRepository::roll_once(&rolls) {
				Some(roll) => {
					let claim = GachaClaimSchema::roll(roll.clone(), user.id);
					match repo_claim.query_create_gacha_claim(claim).await {
						Ok(_) => success_response(ResponseSuccessDto {
							data: GachaRollItemDto::from(&roll),
						}),
						Err(e) => {
							common_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string())
						}
					}
				}
				None => common_response(StatusCode::NOT_FOUND, "No rollable item available"),
			},
			Err(e) => common_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
		}
	}
}
