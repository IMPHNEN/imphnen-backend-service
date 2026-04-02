use axum::{
	Json,
	body::Bytes,
	extract::{FromRequest, Request},
	http::StatusCode,
	response::{IntoResponse, Response},
};
use serde_json::json;

use super::zod_validate::ZodValidate;

pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
	T: ZodValidate + 'static,
	S: Send + Sync,
{
	type Rejection = Response;

	async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
		let bytes = Bytes::from_request(req, state).await.map_err(|e| {
			(
				StatusCode::BAD_REQUEST,
				Json(json!({
						"message": format!("Failed to read body: {e}"),
						"version": env!("CARGO_PKG_VERSION"),
				})),
			)
				.into_response()
		})?;

		let json_value: serde_json::Value =
			serde_json::from_slice(&bytes).map_err(|e| {
				(
					StatusCode::BAD_REQUEST,
					Json(json!({
							"message": format!("Invalid JSON: {e}"),
							"version": env!("CARGO_PKG_VERSION"),
					})),
				)
					.into_response()
			})?;

		let value = T::zod_validate(&json_value).map_err(|e| {
			(
				StatusCode::BAD_REQUEST,
				Json(json!({
						"message": format!("Validation error: {e}"),
						"version": env!("CARGO_PKG_VERSION"),
				})),
			)
				.into_response()
		})?;

		Ok(ValidatedJson(value))
	}
}
