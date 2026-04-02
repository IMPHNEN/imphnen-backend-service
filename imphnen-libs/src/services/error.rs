use crate::postgres::PostgresError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
	#[error("User not found: {0}")]
	UserNotFound(String),

	#[error("Database error: {0}")]
	DatabaseError(#[from] sea_orm::DbErr),

	#[error("Connection error: {0}")]
	ConnectionError(#[from] PostgresError),

	#[error("Authentication failed: {0}")]
	AuthenticationFailed(String),

	#[error("Authorization failed: {0}")]
	AuthorizationFailed(String),

	#[error("Validation error: {0}")]
	ValidationError(String),

	#[error("Internal service error: {0}")]
	InternalError(String),
}
