use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use serde_json::json;
use paginator_utils::PaginatorResponse;
use imphnen_entities::error_dto::error::Error;
use crate::errors::AppError;

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

pub struct ApiSuccess<T: Serialize>(pub T);

impl<T: Serialize> IntoResponse for ApiSuccess<T> {
    fn into_response(self) -> Response {
        (
            StatusCode::OK,
            Json(json!({ "data": self.0, "version": env!("CARGO_PKG_VERSION") })),
        )
            .into_response()
    }
}

pub struct ApiCreated<T: Serialize>(pub T);

impl<T: Serialize> IntoResponse for ApiCreated<T> {
    fn into_response(self) -> Response {
        (
            StatusCode::CREATED,
            Json(json!({ "data": self.0, "version": env!("CARGO_PKG_VERSION") })),
        )
            .into_response()
    }
}

pub struct ApiPaginated<T: Serialize>(pub PaginatorResponse<T>);

impl<T: Serialize> IntoResponse for ApiPaginated<T> {
    fn into_response(self) -> Response {
        (
            StatusCode::OK,
            Json(json!({
                "data": self.0.data,
                "meta": self.0.meta,
                "version": env!("CARGO_PKG_VERSION"),
            })),
        )
            .into_response()
    }
}

pub struct ApiMessage {
    pub status: StatusCode,
    pub message: String,
}

impl ApiMessage {
    pub fn ok(message: impl Into<String>) -> Self {
        Self { status: StatusCode::OK, message: message.into() }
    }

    pub fn created(message: impl Into<String>) -> Self {
        Self { status: StatusCode::CREATED, message: message.into() }
    }

    pub fn new(status: StatusCode, message: impl Into<String>) -> Self {
        Self { status, message: message.into() }
    }
}

impl IntoResponse for ApiMessage {
    fn into_response(self) -> Response {
        (
            self.status,
            Json(json!({ "message": self.message, "version": env!("CARGO_PKG_VERSION") })),
        )
            .into_response()
    }
}
