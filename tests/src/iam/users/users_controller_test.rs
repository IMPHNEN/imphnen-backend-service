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

		// Verify response body contains user data
		let created_user: imphnen_iam::v1::users::users_dto::UsersDetailItemDto =
			crate::common::response_helpers::parse_response(response, 4096).await;
		
		// Validate all required fields in UsersDetailItemDto
		assert!(!created_user.id.is_empty(), "Created user must have non-empty id");
		assert!(!created_user.role.id.is_empty(), "Created user must have non-empty role id");
		assert!(!created_user.role.name.is_empty(), "Created user must have non-empty role name");
		assert!(!created_user.fullname.is_empty(), "Created user must have non-empty fullname");
		assert_eq!(created_user.email, email, "Created user email must match request");
		assert!(!created_user.phone_number.is_empty(), "Created user must have non-empty phone_number");
		assert_eq!(created_user.is_active, true, "Created user must be active");
		assert!(!created_user.created_at.is_empty(), "Created user must have non-empty created_at");
		assert!(!created_user.updated_at.is_empty(), "Created user must have non-empty updated_at");
		
		// Validate optional fields that should exist
		assert!(created_user.avatar.is_some(), "Created user should have avatar field");
		assert!(created_user.phone_for_verification.is_some(), "Created user should have phone_for_verification field");
		assert!(created_user.gender.is_some(), "Created user should have gender field");
		assert!(created_user.birthdate.is_some(), "Created user should have birthdate field");
		assert!(created_user.domicile.is_some(), "Created user should have domicile field");
		assert!(created_user.bio.is_some(), "Created user should have bio field");
		assert!(created_user.last_education.is_some(), "Created user should have last_education field");
		assert!(created_user.linkedin_url.is_some(), "Created user should have linkedin_url field");
		assert!(created_user.github_url.is_some(), "Created user should have github_url field");
		assert!(created_user.cv_url.is_some(), "Created user should have cv_url field");
		assert!(created_user.portfolio_url.is_some(), "Created user should have portfolio_url field");
		assert!(created_user.website_url.is_some(), "Created user should have website_url field");
		assert!(created_user.twitter_url.is_some(), "Created user should have twitter_url field");
		assert!(created_user.location.is_some(), "Created user should have location field");
		assert!(created_user.skills.is_some(), "Created user should have skills field");
		assert!(created_user.experience.is_some(), "Created user should have experience field");
		assert!(created_user.education.is_some(), "Created user should have education field");
		assert!(created_user.career_status.is_some(), "Created user should have career_status field");

		// Verify user was created in database
		let db_user = repo
			.query_user_by_email(email.clone())
			.await
			.unwrap();
		assert_eq!(db_user.email, email);
		assert_eq!(db_user.fullname, "Test User Controller");
		assert_eq!(db_user.is_active, true);

		// Clean up
		let _ = repo.query_delete_user(db_user.id.id.to_raw()).await;
	}
}