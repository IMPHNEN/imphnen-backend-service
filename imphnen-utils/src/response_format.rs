//! Standardized response formatting utilities.
//!
//! This module provides consistent response formatting for API endpoints,
//! including success responses, error responses, and list responses with
//! configurable versioning from Cargo.toml.

use axum::{
  Json,
  http::StatusCode,
  response::{IntoResponse, Response},
};
use serde::Serialize;
use serde_json::json;

use imphnen_entities::{ResponseListSuccessDto, ResponseSuccessDto};
use imphnen_entities::error_dto::error::Error;
use crate::errors::AppError;

// Convert from imphnen_entities::Error to AppError
impl From<Error> for AppError {
    fn from(error: Error) -> Self {
        match error {
            Error::Db(detail) => AppError::InternalServerError(format!("Database error: {detail}")),
            Error::Anyhow(detail) => AppError::InternalServerError(format!("Internal server error: {detail}")),
            Error::StatusCode(status) => AppError::InternalServerError(format!("HTTP error: {status}")),
            Error::Auth(detail) => AppError::AuthenticationError(format!("Authentication error: {detail}")),
            Error::Validation(detail) => AppError::ValidationError(format!("Validation error: {detail}")),
        }
    }
}

pub fn success_response<T: Serialize>(params: ResponseSuccessDto<T>) -> Response {
	(
		StatusCode::OK,
		Json(json!({
			"data": params.data,
			"version": env!("CARGO_PKG_VERSION"),
		})),
	)
		.into_response()
}

pub fn success_list_response<T: Serialize>(
	params: ResponseListSuccessDto<T>,
) -> Response {
	(
		StatusCode::OK,
		Json(json!({
			"data": params.data,
			"meta": params.meta,
			"version": env!("CARGO_PKG_VERSION"),
		})),
	)
		.into_response()
}

pub fn common_response(status: StatusCode, message: &str) -> Response {
  (
    status,
    Json(json!({
      "message": message,
      "version": env!("CARGO_PKG_VERSION"),
    })),
  )
    .into_response()
}

pub fn error_response(error: AppError) -> Response {
  (
    error.status_code(),
    Json(json!({
      "error": error.message(),
      "version": env!("CARGO_PKG_VERSION"),
    })),
  )
    .into_response()
}

pub fn success_created_response<T: Serialize>(params: ResponseSuccessDto<T>) -> Response {
    (
        StatusCode::CREATED,
        Json(json!({
            "data": params.data,
            "version": env!("CARGO_PKG_VERSION"),
        })),
    )
        .into_response()
}
