use crate::AppState;
use imphnen_entities::ResponseSuccessDto;
use imphnen_utils::{errors::AppError, error_response};
use imphnen_utils::{common_response, success_response, validate_request};
use uuid::Uuid;
use crate::v1::gacha_credits::gacha_credits_dto::{GachaCreditRequestDto, GachaCreditResponseDto};
use crate::v1::gacha_credits::gacha_credits_repository::GachaCreditRepository;
use axum::http::StatusCode;
use axum::response::Response;
use imphnen_iam::UsersRepository;
use imphnen_utils::extract_email;

pub struct GachaCreditService;

impl GachaCreditService {
    pub async fn get_user_credits(headers: &axum::http::HeaderMap, state: &AppState) -> Response {
        let repo = GachaCreditRepository::new(state);
        let repo_user = UsersRepository::new(state);
        let Some(email) = extract_email(headers) else {
                    return error_response(AppError::AuthenticationError("Unauthorized".into()));
        };
        
        let Ok(user) = repo_user.query_user_by_email(email.to_string()).await else {
                    return error_response(AppError::NotFoundError("User not found".into()));
        };
        
        let parsed_user_id = match Uuid::parse_str(&user.id) {
            Ok(uuid) => uuid,
            Err(e) => return error_response(AppError::BadRequestError(format!("Invalid User ID format: {}", e))),
        };

        match repo.query_by_user_id(parsed_user_id).await {
            Ok(Some(credit)) => {
                let response_dto = GachaCreditResponseDto {
                    id: credit.id.to_string(),
                    user_id: credit.user_id.to_string(),
                    available_rolls: credit.available_rolls,
                    is_deleted: credit.is_deleted,
                    created_at: credit.created_at.map(|d| d.to_string()),
                    updated_at: credit.updated_at.map(|d| d.to_string()),
                };
                success_response(ResponseSuccessDto { data: response_dto })
            }
            Ok(None) => {
                // Return empty credits if no record exists
                let response_dto = GachaCreditResponseDto {
                    id: "".to_string(),
                    user_id: user.id,
                    available_rolls: 0,
                    is_deleted: false,
                    created_at: None,
                    updated_at: None,
                };
                success_response(ResponseSuccessDto { data: response_dto })
            }
            Err(e) => error_response(AppError::InternalServerError(e.to_string())),
        }
    }

    pub async fn add_user_credits(
        headers: &axum::http::HeaderMap,
        state: &AppState,
        payload: GachaCreditRequestDto,
    ) -> Response {
        if let Err((status, message)) = validate_request(&payload) {
            return common_response(status, &message);
        }

        let repo = GachaCreditRepository::new(state);
        let repo_user = UsersRepository::new(state);
        let Some(email) = extract_email(headers) else {
                    return error_response(AppError::AuthenticationError("Unauthorized".into()));
        };
        
        let Ok(user) = repo_user.query_user_by_email(email.to_string()).await else {
                    return error_response(AppError::NotFoundError("User not found".into()));
        };

        // Ensure the user can only modify their own credits
        if payload.user_id != user.id {
            return error_response(AppError::AuthorizationError("You can only modify your own credits".into()));
        }

        let amount = payload.amount; // Extract amount before moving payload
        match repo.query_add_credit(payload).await {
            Ok(_) => common_response(
                StatusCode::OK,
                &format!("Added {} credits successfully", amount)
            ),
            Err(e) => error_response(AppError::InternalServerError(e.to_string())),
        }
    }

    pub async fn consume_user_credit(headers: &axum::http::HeaderMap, state: &AppState) -> Response {
        let repo = GachaCreditRepository::new(state);
        let repo_user = UsersRepository::new(state);
        let Some(email) = extract_email(headers) else {
                    return error_response(AppError::AuthenticationError("Unauthorized".into()));
        };
        
        let Ok(user) = repo_user.query_user_by_email(email.to_string()).await else {
                    return error_response(AppError::NotFoundError("User not found".into()));
        };

        let parsed_user_id = match Uuid::parse_str(&user.id) {
            Ok(uuid) => uuid,
            Err(e) => return error_response(AppError::BadRequestError(format!("Invalid User ID format: {}", e))),
        };

        match repo.query_consume_credit(parsed_user_id).await {
            Ok(_) => common_response(StatusCode::OK, "Consumed 1 credit successfully"),
            Err(e) => error_response(AppError::BadRequestError(e.to_string())),
        }
    }
}