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

use crate::{ResponseListSuccessDto, ResponseSuccessDto};

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
