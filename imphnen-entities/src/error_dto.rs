pub mod error {
	use axum::Json;
	use axum::http::StatusCode;
	use axum::response::IntoResponse;
	use axum::response::Response;
	use thiserror::Error;

	#[derive(Error, Debug)]
	pub enum Error {
		#[error("database error: {0}")]
		Db(String),
		#[error("anyhow error: {0}")]
		Anyhow(#[from] anyhow::Error),
		#[error("HTTP status code error: {0}")]
		StatusCode(StatusCode),
		#[error("authentication error: {0}")]
		Auth(String),
		#[error("validation error: {0}")]
		Validation(String),
	}

	impl IntoResponse for Error {
		fn into_response(self) -> Response {
			let (status, error_message) = match self {
				Error::Db(detail) => (
					StatusCode::INTERNAL_SERVER_ERROR,
					format!("Database error: {detail}"),
				),
				Error::Anyhow(detail) => (
					StatusCode::INTERNAL_SERVER_ERROR,
					format!("Internal server error: {detail}"),
				),
				Error::StatusCode(s) => (s, format!("HTTP error: {s}")),
				Error::Auth(detail) => (
					StatusCode::UNAUTHORIZED,
					format!("Authentication error: {detail}"),
				),
				Error::Validation(detail) => (
					StatusCode::BAD_REQUEST,
					format!("Validation error: {detail}"),
				),
			};
			(status, Json(error_message)).into_response()
		}
	}

	impl From<StatusCode> for Error {
		fn from(status: StatusCode) -> Self {
			Self::StatusCode(status)
		}
	}
}
