//! Custom extractor for automatic JSON validation and sanitization
//!
//! This extractor automatically validates request payloads using the validator crate
//! and returns appropriate error responses if validation fails.

use axum::{
    extract::{rejection::JsonRejection, FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::de::DeserializeOwned;
use serde_json;
use validator::Validate;

/// Custom extractor that automatically validates JSON payloads
///
/// # Example
/// ```rust
/// use validated_json::ValidatedJson;
/// use serde::Deserialize;
/// use validator::Validate;
///
/// #[derive(Deserialize, Validate)]
/// struct CreateUserRequest {
///     #[validate(email)]
///     email: String,
///     #[validate(length(min = 8))]
///     password: String,
/// }
///
/// async fn create_user(
///     ValidatedJson(payload): ValidatedJson<CreateUserRequest>
/// ) -> Response {
///     // payload is already validated
///     // ... your logic here
/// }
/// ```
pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate + 'static,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // First, extract JSON
        let Json(value) = match Json::<T>::from_request(req, state).await {
            Ok(value) => value,
            Err(rejection) => {
                let error_message = format!("Invalid JSON payload: {}", rejection);
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({
                        "error": error_message,
                        "version": env!("CARGO_PKG_VERSION"),
                    })),
                )
                    .into_response());
            }
        };

        // Then, validate it
        if let Err(errors) = value.validate() {
            let error_messages: Vec<String> = errors
                .field_errors()
                .iter()
                .flat_map(|(field, errors)| {
                    errors.iter().map(move |error| {
                        format!(
                            "{}: {}",
                            field,
                            error.message.as_ref().map(|m| m.to_string()).unwrap_or_else(|| error.code.to_string())
                        )
                    })
                })
                .collect();

            return Err((
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "Validation failed",
                    "details": error_messages,
                    "version": env!("CARGO_PKG_VERSION"),
                })),
            )
                .into_response());
        }

        Ok(ValidatedJson(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Debug, Deserialize, Validate)]
    struct TestPayload {
        #[validate(email)]
        email: String,
        #[validate(length(min = 8))]
        password: String,
    }

    // Note: Full integration tests should be done at the application level
}
