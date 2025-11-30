#[cfg(test)]
mod tests {
	use crate::{generate_unique_email, get_role_id, UsersRepository};
	use imphnen_iam::UsersSchema;
	use imphnen_entities::users::UserProfileExtensionDto;
	use imphnen_utils::{make_thing_from_enum, ResourceEnum as UtilsResourceEnum};
	use uuid::Uuid;

	#[tokio::test]
	async fn test_query_create_user() {
	    let app_state = crate::get_app_state().await;
	    let repo = UsersRepository::new(&app_state);
		let role_id = get_role_id("user", &app_state).await;

		// Test data
		let email = generate_unique_email("test_create_user");
		let user_schema = UsersSchema {
			id: make_thing_from_enum(UtilsResourceEnum::Users, &Uuid::new_v4().to_string()),
			email: Some(email.clone()),
			fullname: Some("Test User Create".to_string()),
			password: Some("password123".to_string()),
			profile_extension: Some(UserProfileExtensionDto { phone_number: Some("+1234567890".to_string()), ..Default::default() }),
			is_active: true,
			role_id: Uuid::parse_str(&role_id).ok(),
			..Default::default()
		};

		// Create user
		let result = repo.query_create_user(user_schema.clone()).await;
		assert!(result.is_ok());

		// Verify user was created
		let created_user_result = repo.query_user_by_email(email.clone()).await;
		assert!(created_user_result.is_ok());
		let created_user = created_user_result.as_ref().unwrap();
		assert_eq!(created_user.email, email);

		// Clean up
		let user = created_user_result.unwrap();
		let _ = repo.query_delete_user(user.id.clone()).await;
	}

	#[tokio::test]
	async fn test_query_user_by_email() {
	    let app_state = crate::get_app_state().await;
	    let repo = UsersRepository::new(&app_state);
		let role_id = get_role_id("user", &app_state).await;

		// Test data
		let email = generate_unique_email("test_get_by_email");
		let user_schema = UsersSchema {
			id: make_thing_from_enum(UtilsResourceEnum::Users, &Uuid::new_v4().to_string()),
			email: Some(email.clone()),
			fullname: Some("Test User Email".to_string()),
			password: Some("password123".to_string()),
			profile_extension: Some(UserProfileExtensionDto { phone_number: Some("+1234567890".to_string()), ..Default::default() }),
			is_active: true,
			role_id: Uuid::parse_str(&role_id).ok(),
			..Default::default()
		};

		// Create user first
		let create_result = repo.query_create_user(user_schema.clone()).await;
		assert!(create_result.is_ok());

		// Get user by email
		let result = repo.query_user_by_email(email.clone()).await;
		assert!(result.is_ok());
		let user = result.as_ref().unwrap();
		assert_eq!(user.email, email);

		// Clean up
		let user = result.unwrap();
		let _ = repo.query_delete_user(user.id.clone()).await;
	}

	#[tokio::test]
	async fn test_query_user_by_email_not_found() {
	    let app_state = crate::get_app_state().await;
	    let repo = UsersRepository::new(&app_state);

		// Try to get non-existent user
		let result = repo.query_user_by_email("nonexistent@example.com".to_string()).await;
		assert!(result.is_err());
	}

	#[tokio::test]
	async fn test_query_update_user() {
	    let app_state = crate::get_app_state().await;
	    let repo = UsersRepository::new(&app_state);
		let role_id = get_role_id("user", &app_state).await;

		// Test data
		let email = generate_unique_email("test_update_user");
		let user_schema = UsersSchema {
			id: make_thing_from_enum(UtilsResourceEnum::Users, &Uuid::new_v4().to_string()),
			email: Some(email.clone()),
			fullname: Some("Original Name".to_string()),
			password: Some("password123".to_string()),
			profile_extension: Some(UserProfileExtensionDto { phone_number: Some("+1234567890".to_string()), ..Default::default() }),
			is_active: true,
			role_id: Uuid::parse_str(&role_id).ok(),
			..Default::default()
		};

		// Create user first
		let create_result = repo.query_create_user(user_schema.clone()).await;
		assert!(create_result.is_ok());

		// Get user to update
		let user = repo.query_user_by_email(email.clone()).await.unwrap();
		
		// Update user
		let updated_schema = UsersSchema {
			id: user.id.clone(),
			email: Some(user.email.clone()),
			fullname: Some("Updated Name".to_string()),
			profile_extension: Some(UserProfileExtensionDto { phone_number: Some("+9876543210".to_string()), ..Default::default() }),
			role_id: Uuid::parse_str(&user.role.id.to_raw()).ok(),
			..Default::default()
		};

		let result = repo.query_update_user(updated_schema).await;
		assert!(result.is_ok());

		// Verify user was updated
		let retrieved_user = repo.query_user_by_email(email.clone()).await.unwrap();
		assert_eq!(retrieved_user.fullname, "Updated Name");
		assert_eq!(retrieved_user.profile_extension.as_ref().unwrap().phone_number.as_ref().unwrap(), "+9876543210");

		// Clean up
		let _ = repo.query_delete_user(retrieved_user.id.clone()).await;
	}

	#[tokio::test]
	async fn test_query_delete_user() {
	    let app_state = crate::get_app_state().await;
	    let repo = UsersRepository::new(&app_state);
		let role_id = get_role_id("user", &app_state).await;

		// Test data
		let email = generate_unique_email("test_delete_user");
		let user_schema = UsersSchema {
			id: make_thing_from_enum(UtilsResourceEnum::Users, &Uuid::new_v4().to_string()),
			email: Some(email.clone()),
			fullname: Some("Test User Delete".to_string()),
			password: Some("password123".to_string()),
			profile_extension: Some(UserProfileExtensionDto { phone_number: Some("+1234567890".to_string()), ..Default::default() }),
			is_active: true,
			role_id: Uuid::parse_str(&role_id).ok(),
			..Default::default()
		};

		// Create user first
		let create_result = repo.query_create_user(user_schema.clone()).await;
		assert!(create_result.is_ok());

		// Get user to delete
		let user = repo.query_user_by_email(email.clone()).await.unwrap();

		// Delete user
		let result = repo.query_delete_user(user.id.clone()).await;
		assert!(result.is_ok());

		// Verify user was deleted
		let deleted_user = repo.query_user_by_email(email.clone()).await;
		assert!(deleted_user.is_err());
	}

	#[tokio::test]
	async fn test_query_user_list() {
	    let app_state = crate::get_app_state().await;
	    let repo = UsersRepository::new(&app_state);
		let role_id = get_role_id("user", &app_state).await;

		// Create test users
		let user_emails = vec![
			generate_unique_email("user_list_1"),
			generate_unique_email("user_list_2"),
			generate_unique_email("user_list_3"),
		];

		for email in &user_emails {
			let user_schema = UsersSchema {
				id: make_thing_from_enum(UtilsResourceEnum::Users, &Uuid::new_v4().to_string()),
				email: Some(email.clone()),
				fullname: Some(format!("Test User {email}")),
				password: Some("password123".to_string()),
				profile_extension: Some(UserProfileExtensionDto { phone_number: Some("+1234567890".to_string()), ..Default::default() }),
				is_active: true,
				role_id: Uuid::parse_str(&role_id).ok(),
				..Default::default()
			};
			let _ = repo.query_create_user(user_schema).await;
		}

		// Get user list
		let meta = crate::get_meta_request_dto(1, 10);
		let result = repo.query_user_list(meta).await;
		assert!(result.is_ok());
		assert!(result.unwrap().data.len() >= 3);

		// Clean up
		for email in user_emails {
			let user = repo.query_user_by_email(email).await.unwrap();
			let _ = repo.query_delete_user(user.id.clone()).await;
		}
	}
}
