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
	}

	impl IntoResponse for Error {
		fn into_response(self) -> Response {
			let (status, error_message) = match self {
				Error::Db(detail) => (
					StatusCode::INTERNAL_SERVER_ERROR,
					format!("Database error: {detail}"),
				),
			};
			(status, Json(error_message)).into_response()
		}
	}

	impl From<surrealdb::Error> for Error {
		fn from(error: surrealdb::Error) -> Self {
			Self::Db(error.to_string())
		}
	}
}
