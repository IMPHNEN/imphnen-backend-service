use crate::AppState;
use imphnen_entities::ResponseSuccessDto;
use imphnen_utils::{common_response, success_response, validate_request};
use crate::v1::gacha_claims::gacha_claims_repository::GachaClaimRepository;
use crate::v1::gacha_claims::gacha_claims_schema::GachaClaimSchema;
use crate::v1::gacha_rolls::gacha_rolls_dto::{GachaRollItemDto, GachaRollRequestDto};
use crate::v1::gacha_rolls::gacha_rolls_repository::GachaRollRepository;
use crate::v1::gacha_rolls::gacha_rolls_schema::GachaRollSchema;
use crate::v1::gacha_credits::gacha_credits_repository::GachaCreditRepository;
use axum::http::{HeaderMap, StatusCode};
use axum::response::Response;
use imphnen_iam::UsersRepository;
use imphnen_utils::extract_email;
use uuid::Uuid;

pub struct GachaRollService;

impl GachaRollService {
	pub async fn get_gacha_roll_by_id(state: &AppState, id: Uuid) -> Response {
		let repo = GachaRollRepository::new(state);
		match repo.query_gacha_roll_by_id(id).await {
			Ok(roll) => success_response(ResponseSuccessDto {
				data: GachaRollItemDto::from(&roll),
			}),
			Err(e) => common_response(StatusCode::NOT_FOUND, &e.to_string()),
		}
	}

	pub async fn create_gacha_roll(
		headers: HeaderMap, // Add headers
		state: &AppState,
		payload: GachaRollRequestDto,
		gacha_id: String, // Add gacha_id
	) -> Response {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}
		let repo_user = UsersRepository::new(state); // Need UsersRepository here
		let Some(email) = extract_email(&headers) else {
			return common_response(StatusCode::UNAUTHORIZED, "Unauthorized");
		};
		let Ok(user) = repo_user.query_user_by_email(email.to_string()).await else {
			return common_response(StatusCode::NOT_FOUND, "User not found");
		};
		
		let schema = GachaRollSchema::create(payload, user.id.clone(), gacha_id); // Pass user.id and gacha_id
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
			let repo_credits = GachaCreditRepository::new(state);
			let Some(email) = extract_email(&headers) else {
				return common_response(StatusCode::UNAUTHORIZED, "Unauthorized");
			};
			let Ok(user) = repo_user.query_user_by_email(email.to_string()).await else {
				return common_response(StatusCode::NOT_FOUND, "User not found");
			};
			
			let parsed_user_id = match Uuid::parse_str(&user.id) {
                Ok(uuid) => uuid,
                Err(e) => return common_response(StatusCode::BAD_REQUEST, &format!("Invalid User ID format: {}", e)),
            };

			// Check if user has enough credits
			let credit_opt = repo_credits.query_by_user_id(parsed_user_id).await;
			let has_enough_credits = match credit_opt {
				Ok(Some(credit)) => credit.available_rolls > 0,
				Ok(None) => false, // No credit record means no credits
				Err(e) => {
					return common_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string())
				}
			};
			
			if !has_enough_credits {
				return common_response(StatusCode::PAYMENT_REQUIRED, "Not enough credits to perform this action");
			}
			
			// Consume one credit
			if let Err(e) = repo_credits.query_consume_credit(parsed_user_id).await { return common_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()) }
			
			// Proceed with the roll
			match repo.query_all_active_rolls().await {
				Ok(rolls) => match GachaRollRepository::roll_once(&rolls) {
					Some(roll) => {
						let user_id_clone = user.id.clone();
						let claim = GachaClaimSchema::roll(roll.clone(), user_id_clone);
						match repo_claim.query_create_gacha_claim(claim).await {
							Ok(_) => success_response(ResponseSuccessDto {
								data: GachaRollItemDto::from(&roll),
							}),
							Err(e) => {
								// Refund the credit if claim creation fails
								let user_id = user.id.clone(); // Extract value before potential move
								let _ = repo_credits.query_add_credit(crate::v1::gacha_credits::gacha_credits_dto::GachaCreditRequestDto {
									user_id,
									amount: 1,
								}).await;
								common_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string())
							}
						}
					}
					None => common_response(StatusCode::NOT_FOUND, "No rollable item available"),
				},
				Err(e) => common_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
			}
		}

	pub async fn soft_delete_gacha_roll(state: &AppState, id: Uuid) -> Response {
		let repo = GachaRollRepository::new(state);
		match repo.query_soft_delete_gacha_roll(id).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(e) => common_response(StatusCode::NOT_FOUND, &e.to_string()),
		}
	}
}
