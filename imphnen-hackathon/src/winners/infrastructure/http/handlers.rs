use super::dto::WinnerResponse;
use crate::winners::domain::service::WinnerService;
use axum::{Extension, response::IntoResponse};
use imphnen_utils::{errors::AppError, response_format::ApiSuccess};
use std::sync::Arc;

#[utoipa::path(
    get,
    path = "/v1/hackathon/winners",
    responses(
        (status = 200, description = "List hackathon winners")
    ),
    tag = "Hackathon - Winners"
)]
pub async fn list_winners_handler(
	Extension(service): Extension<Arc<dyn WinnerService>>,
) -> Result<axum::response::Response, AppError> {
	let winners = service.list_winners().await?;
	let response: Vec<WinnerResponse> =
		winners.into_iter().map(WinnerResponse::from).collect();
	Ok(ApiSuccess(response).into_response())
}
