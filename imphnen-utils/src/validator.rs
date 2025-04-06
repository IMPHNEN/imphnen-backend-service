use axum::http::StatusCode;
use validator::Validate;

pub fn validate_request<T: Validate>(
	payload: &T,
) -> Result<(), (StatusCode, String)> {
	if let Err(validation_errors) = payload.validate() {
		let error_messages: Vec<String> = validation_errors
			.field_errors()
			.iter()
			.flat_map(|(_, errors)| {
				errors.iter().map(move |error| {
					format!(
						"{}",
						error
							.message
							.clone()
							.unwrap_or_else(|| "Invalid value".into())
					)
				})
			})
			.collect();

		return Err((StatusCode::BAD_REQUEST, error_messages.join(", ")));
	}

	Ok(())
}
