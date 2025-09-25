#[cfg(test)]
mod tests {
	use crate::{generate_unique_email, get_role_id, UsersRepository, setup_all_test_environment};
	use axum::{http::StatusCode, response::Response};
	use imphnen_entities::{AppState, MetaRequestDto};
	use imphnen_iam::{
		users_dto::{UserCreateRequestDto, UserUpdateRequestDto},
		users_service::UsersService,
		UsersRepository
	};
	use imphnen_utils::{hash_password, make_thing_from_enum};
	use surrealdb::Uuid;

	#[tokio::test]
	async fn test_create_user_service() {
		let app_state = setup_all_test_environment().await;
		let user_repo = UsersRepository::new(&app_state);
		let role_repo = imphnen_iam::RolesRepository::new(&app_state);

		// Test data
		let email = generate_unique_email("test_create_user");
		let password = "Password123!".to_string();
		let hashed_password = hash_password(&password).await.unwrap();
		
		let user_dto = UserCreateRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "Test User Service".to_string(),
			phone_number: Some("1234567890".to_string()),
			role_id: get_role_id(&app_state, "user").await.unwrap(),
		};

		// Create user
		let response = UsersService::create_user(&app_state, user_dto.clone()).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);

		// Verify user was created in database
		let created_user = user_repo.query_user_by_email(email.clone()).await;
		assert!(created_user.is_ok());
		assert_eq!(created_user.unwrap().email, email);
		assert_eq!(created_user.unwrap().fullname, "Test User Service".to_string());

		// Clean up
		let user = created_user.unwrap();
		let _ = user_repo.query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_get_user_list_service() {
		let app_state = setup_all_test_environment().await;
		let user_repo = UsersRepository::new(&app_state);

		// Create test user first
		let email = generate_unique_email("test_user_list");
		let password = "Password123!".to_string();
		
		let user_dto = UserCreateRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "Test User List".to_string(),
			phone_number: Some("1234567890".to_string()),
			role_id: get_role_id(&app_state, "user").await.unwrap(),
		};

		let _ = UsersService::create_user(&app_state, user_dto).await;

		// Get user list
		let meta = MetaRequestDto {
			limit: 10,
			page: 1,
			search: None,
			sort: None,
			filter: None,
		};

		let response = UsersService::get_user_list(&app_state, meta).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);

		// Clean up
		let user = user_repo.query_user_by_email(email.clone()).await.unwrap();
		let _ = user_repo.query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_get_user_by_id_service() {
		let app_state = setup_all_test_environment().await;
		let user_repo = UsersRepository::new(&app_state);

		// Create test user first
		let email = generate_unique_email("test_user_by_id");
		let password = "Password123!".to_string();
		
		let user_dto = UserCreateRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "Test User By ID".to_string(),
			phone_number: Some("1234567890".to_string()),
			role_id: get_role_id(&app_state, "user").await.unwrap(),
		};

		let _ = UsersService::create_user(&app_state, user_dto).await;

		let user = user_repo.query_user_by_email(email.clone()).await.unwrap();
		let user_id = user.id.id.to_raw();

		// Get user by ID
		let response = UsersService::get_user_by_id(&app_state, user_id).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);

		// Clean up
		let _ = user_repo.query_delete_user(user_id).await;
	}

	#[tokio::test]
	async fn test_update_user_service() {
		let app_state = setup_all_test_environment().await;
		let user_repo = UsersRepository::new(&app_state);

		// Create test user first
		let email = generate_unique_email("test_update_user");
		let password = "Password123!".to_string();
		
		let user_dto = UserCreateRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "Test User Update".to_string(),
			phone_number: Some("1234567890".to_string()),
			role_id: get_role_id(&app_state, "user").await.unwrap(),
		};

		let _ = UsersService::create_user(&app_state, user_dto).await;

		let user = user_repo.query_user_by_email(email.clone()).await.unwrap();
		let user_id = user.id.id.to_raw();

		// Prepare update request
		let update_dto = UserUpdateRequestDto {
			fullname: Some("Updated Test User".to_string()),
			phone_number: Some("0987654321".to_string()),
			is_active: Some(true),
		};

		// Update user
		let response = UsersService::update_user(&app_state, user_id, update_dto).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);

		// Verify user was updated
		let updated_user = user_repo.query_user_by_id(&user.id, false).await.unwrap();
		assert_eq!(updated_user.fullname, "Updated Test User".to_string());
		assert_eq!(updated_user.phone_number, Some("0987654321".to_string()));
		assert_eq!(updated_user.is_active, true);

		// Clean up
		let _ = user_repo.query_delete_user(user_id).await;
	}

	#[tokio::test]
	async fn test_delete_user_service() {
		let app_state = setup_all_test_environment().await;
		let user_repo = UsersRepository::new(&app_state);

		// Create test user first
		let email = generate_unique_email("test_delete_user");
		let password = "Password123!".to_string();
		
		let user_dto = UserCreateRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "Test User Delete".to_string(),
			phone_number: Some("1234567890".to_string()),
			role_id: get_role_id(&app_state, "user").await.unwrap(),
		};

		let _ = UsersService::create_user(&app_state, user_dto).await;

		let user = user_repo.query_user_by_email(email.clone()).await.unwrap();
		let user_id = user.id.id.to_raw();

		// Delete user
		let response = UsersService::delete_user(&app_state, user_id).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);

		// Verify user was deleted
		let deleted_user = user_repo.query_user_by_email(email.clone()).await;
		assert!(deleted_user.is_err());
	}

	#[tokio::test]
	async fn test_get_user_by_email_service() {
		let app_state = setup_all_test_environment().await;
		let user_repo = UsersRepository::new(&app_state);

		// Create test user first
		let email = generate_unique_email("test_user_by_email");
		let password = "Password123!".to_string();
		
		let user_dto = UserCreateRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "Test User By Email".to_string(),
			phone_number: Some("1234567890".to_string()),
			role_id: get_role_id(&app_state, "user").await.unwrap(),
		};

		let _ = UsersService::create_user(&app_state, user_dto).await;

		// Get user by email
		let response = UsersService::get_user_by_email(&app_state, email.clone()).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);

		// Clean up
		let user = user_repo.query_user_by_email(email.clone()).await.unwrap();
		let _ = user_repo.query_delete_user(user.id.id.to_raw()).await;
	}
}