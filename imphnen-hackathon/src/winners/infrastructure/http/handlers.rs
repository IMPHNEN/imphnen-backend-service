use super::dto::WinnerResponse;
use crate::winners::domain::service::WinnerService;
use axum::{Extension, response::IntoResponse};
use imphnen_utils::{errors::AppError, response_format::ApiSuccess};
use std::sync::Arc;

#[utoipa::path(
    get,
    path = "/v1/hackathon/winners",
    responses(
        (status = 200, description = "List hackathon winners",
         example = json!({
             "data": [
                 {
                     "id": "d4e5f6a7-b8c9-0123-defa-234567890123",
                     "team_id": "7c3a1d2e-8f4b-4c5a-9d6e-1f2a3b4c5d6e",
                     "team_name": "Rust Enjoyers",
                     "rank": 1,
                     "prize": "Rp 10.000.000",
                     "announced_at": "2025-02-01T10:00:00Z",
                     "created_at": "2025-02-01T10:00:00Z"
                 },
                 {
                     "id": "e5f6a7b8-c9d0-1234-efab-345678901234",
                     "team_id": "8d4b2e3f-9a5c-4d6b-0e7f-2a3b4c5d6e7f",
                     "team_name": "Go Builders",
                     "rank": 2,
                     "prize": "Rp 5.000.000",
                     "announced_at": "2025-02-01T10:00:00Z",
                     "created_at": "2025-02-01T10:00:00Z"
                 }
             ],
             "version": "0.3.0"
         }))
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
