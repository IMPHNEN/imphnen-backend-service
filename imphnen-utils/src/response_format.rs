use axum::{
	http::StatusCode,
	response::{IntoResponse, Response},
	Json,
};
use serde::Serialize;
use serde_json::json;

use crate::{ResponseListSuccessDto, ResponseSuccessDto};

pub fn success_response<T: Serialize>(params: ResponseSuccessDto<T>) -> Response {
	(
		StatusCode::OK,
		Json(json!({
			"data": params.data,
			"version": "0.1.0",
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
			"version": "0.1.0",
		})),
	)
		.into_response()
}

pub fn common_response(status: StatusCode, message: &str) -> Response {
	(
		status,
		Json(json!({
			"message": message,
			"version": "0.1.0",
		})),
	)
		.into_response()
}
