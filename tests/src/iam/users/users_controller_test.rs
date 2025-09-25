#[cfg(test)]
mod tests {
	use crate::{generate_unique_email, get_role_id, UsersRepository};
	use axum::{http::StatusCode, response::Response};
	use imphnen_iam::{
		UsersCreateRequestDto, UsersUpdateRequestDto, UsersSchema, ResourceEnum,
		UsersActiveInactiveRequestDto, UsersSetNewPasswordRequestDto
	};
	use imphnen_utils::{make_thing_from_enum, ResourceEnum as UtilsResourceEnum};
	use uuid::Uuid;

	#[tokio::test]
	async fn test_create_user_controller() {
		let app_state = crate::get_app_state().await;
		let repo = UsersRepository::new(&app_state);
		let role_id = get_role_id("mentee", &app_state).await;

		// Test data
		let email = generate_unique_email("test_user_controller");
		let user_request = UsersCreateRequestDto {
			email: email.clone(),
			password: "password123".to_string(),
			fullname: "Test User Controller".to_string(),
			phone_number: Some("+1234567890".to_string()),
			is_active: true,
			avatar: None,
			role_id: role_id,
		};

		// Create user through controller
		let response = imphnen_iam::UsersController::create_user(
			&app_state,
			user_request.clone(),
		)
		.await;

		// Verify response
		assert_eq!(response.status(), StatusCode::CREATED);

		// Verify user was created in database
		let created_user = repo
			.query_user_by_email(email.clone())
			.await
			.unwrap();
		assert_eq!(created_user.email, email);
		assert_eq!(created_user.fullname, "Test User Controller");
		assert_eq!(created_user.is_active, true);

		// Clean up
		let _ = repo.query_delete_user(created_user.id.id.to_raw()).await;
	}
}