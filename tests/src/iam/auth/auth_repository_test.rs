#[cfg(test)]
mod auth_repository_test {
	use crate::{
		generate_unique_email,
		get_iso_date,
		get_role_id,
		make_thing,
		setup_all_test_environment, // Import the new setup function
		AuthOtpSchema,
		AuthRepository,
		ResourceEnum,
		UsersRepository,
		UsersSchema,
	};
	use chrono::{Duration, Utc};
	use imphnen_iam::{AppState, RolesDetailQueryDto, UsersDetailQueryDto};
	use surrealdb::Uuid;

	async fn create_mock_user(state: &AppState, email: &str) -> UsersSchema {
		UsersSchema {
			id: make_thing("app_users", &Uuid::new_v4().to_string()),
			email: email.to_string(),
			fullname: "Test User".to_string(),
			legal_name: None,
			password: "password".to_string(),
			is_deleted: false,
			avatar: None,
			phone_number: "081234567890".to_string(),
			phone_for_verification: None,
			is_active: true,
			gender: None,
			birthdate: None,
			domicile: None,
			identity_document_url: None,
			bio: None,
			last_education: None,
			linkedin_url: None,
			github_url: None,
			cv_url: None,
			portfolio_url: None,
			role: make_thing("app_roles", &get_role_id(state).await),
			mentor_id: None,
			created_at: get_iso_date(),
			updated_at: get_iso_date(),
		}
	}

	#[tokio::test]
	async fn test_store_and_get_user() {
		let app_state = setup_all_test_environment().await; // Use the new setup function
		let repo = AuthRepository::new(&app_state);
		let email = generate_unique_email("forgot");
		let mut user = create_mock_user(&app_state, &email).await;
		user.role = make_thing("app_roles", &get_role_id(&app_state).await);
		let user_repo = UsersRepository::new(&app_state);
		let create_user = user_repo.query_create_user(user.clone()).await;
		assert!(create_user.is_ok());
		let user_data = user_repo.query_user_by_email(email).await;
		assert!(
			user_data.is_ok(),
			"Failed to get user by email: {:?}",
			user_data.err()
		);
		let user_data = user_data.unwrap();
		let store = repo.query_store_user(user_data.clone()).await;
		assert!(store.is_ok(), "Failed to store user: {:?}", store.err());
		let fetched = repo.query_get_stored_user(user.email.clone()).await;
		assert!(
			fetched.is_ok(),
			"Failed to fetch stored user: {:?}",
			fetched.err()
		);
		assert_eq!(fetched.unwrap().email, user.email);
	}

	#[tokio::test]
	async fn test_delete_stored_user() {
		let state = setup_all_test_environment().await; // Use the new setup function
		let auth_repo = AuthRepository::new(&state);
		let email = "delete_me@example.com".to_string();
		let mock_user = UsersDetailQueryDto {
			id: make_thing(&ResourceEnum::UsersCache.to_string(), &email),
			fullname: "Test User".into(),
			legal_name: None,
			email: email.clone(),
			avatar: None,
			phone_number: "08123456789".into(),
			phone_for_verification: None,
			is_active: true,
			gender: None,
			birthdate: None,
			domicile: None,
			identity_document_url: None,
			bio: None,
			last_education: None,
			linkedin_url: None,
			github_url: None,
			cv_url: None,
			portfolio_url: None,
			role: RolesDetailQueryDto {
				id: make_thing("app_roles", &Uuid::new_v4().to_string()),
				name: "Dummy Role".into(),
				permissions: vec![],
				is_deleted: false,
				created_at: Some(get_iso_date()),
				updated_at: Some(get_iso_date()),
			},
			is_deleted: false,
			password: "".into(),
			mentor_id: None,
			created_at: get_iso_date(),
			updated_at: get_iso_date(),
		};
		let created: Result<Option<UsersDetailQueryDto>, surrealdb::Error> = state
			.surrealdb_mem
			.create((ResourceEnum::UsersCache.to_string(), email.clone()))
			.content(mock_user)
			.await;
		assert!(
			created.is_ok(),
			"Failed to create mock user: {:?}",
			created.err()
		);
		let result = auth_repo.query_delete_stored_user(email.clone()).await;
		assert!(
			result.is_ok(),
			"Delete operation failed: {:?}",
			result.err()
		);
		assert_eq!(result.unwrap(), "Success delete stored user");
	}

	#[tokio::test]
	async fn test_store_and_get_otp() {
		let app_state = setup_all_test_environment().await; // Use the new setup function
		let repo = AuthRepository::new(&app_state);
		let email = "otp_user@example.com".to_string();
		let otp = 123456;
		let stored = repo.query_store_otp(email.clone(), otp).await;
		assert!(stored.is_ok(), "Failed to store OTP: {:?}", stored.err());
		let fetched = repo.query_get_stored_otp(email.clone()).await;
		assert!(fetched.is_ok(), "Failed to fetch OTP: {:?}", fetched.err());
		assert_eq!(fetched.unwrap(), otp);
	}

	#[tokio::test]
	async fn test_delete_stored_otp() {
		let app_state = setup_all_test_environment().await; // Use the new setup function
		let repo = AuthRepository::new(&app_state);
		let email = "otp_del@example.com".to_string();
		let otp = 654321;
		let store_res = repo.query_store_otp(email.clone(), otp).await;
		assert!(
			store_res.is_ok(),
			"Failed to store OTP: {:?}",
			store_res.err()
		);
		let deleted = repo.query_delete_stored_otp(email.clone()).await;
		assert!(deleted.is_ok(), "Failed to delete OTP: {:?}", deleted.err());
		let fetched = repo.query_get_stored_otp(email.clone()).await;
		assert!(
			fetched.is_err(),
			"OTP should be deleted, but got: {fetched:?}"
		);
	}

	#[tokio::test]
	async fn test_expired_otp() {
		let app_state = setup_all_test_environment().await; // Use the new setup function
		let repo = AuthRepository::new(&app_state);
		let email = "expired_otp@example.com".to_string();
		let otp = 789012;
		let table = ResourceEnum::OtpCache.to_string();
		let expires_at = Utc::now() - Duration::seconds(1);
		let created: Result<Option<AuthOtpSchema>, surrealdb::Error> = repo
			.state
			.surrealdb_mem
			.create((table.clone(), email.as_str()))
			.content(AuthOtpSchema { otp, expires_at })
			.await;
		assert!(
			created.is_ok(),
			"Failed to create expired OTP: {:?}",
			created.err()
		);
		let result = repo.query_get_stored_otp(email.clone()).await;
		assert!(
			result.is_err(),
			"Expired OTP should not be retrievable, got: {result:?}"
		);
		if let Some(err) = result.err() {
			assert!(
				err.to_string().contains("OTP expired"),
				"Expected 'OTP expired' error, got: {err}"
			);
		}
	}

	#[tokio::test]
	async fn test_get_non_existent_stored_user_should_fail() {
		let app_state = setup_all_test_environment().await; // Use the new setup function
		let repo = AuthRepository::new(&app_state);
		let result = repo
			.query_get_stored_user("not_found@example.com".into())
			.await;
		assert!(result.is_err());
		if let Some(err) = result.err() {
			assert!(
				err.to_string().contains("No stored user data found"),
				"Expected 'No stored user data found' error, got: {err}"
			);
		}
	}

	#[tokio::test]
	async fn test_delete_non_existent_user_should_fail() {
		let app_state = setup_all_test_environment().await; // Use the new setup function
		let repo = AuthRepository::new(&app_state);
		let result = repo
			.query_delete_stored_user("ghost@example.com".into())
			.await;
		assert!(result.is_err());
		if let Some(err) = result.err() {
			assert!(
				err.to_string().contains("Failed delete stored user"),
				"Expected 'Failed delete stored user' error, got: {err}"
			);
		}
	}

	#[tokio::test]
	async fn test_store_and_get_valid_otp() {
		let app_state = setup_all_test_environment().await; // Use the new setup function
		let repo = AuthRepository::new(&app_state);
		let email = "valid_otp@example.com";
		let otp = 654321;
		let store_result = repo.query_store_otp(email.into(), otp).await;
		assert!(
			store_result.is_ok(),
			"Failed to store valid OTP: {:?}",
			store_result.err()
		);
		let get_result = repo.query_get_stored_otp(email.into()).await;
		assert!(
			get_result.is_ok(),
			"Failed to get valid OTP: {:?}",
			get_result.err()
		);
		assert_eq!(get_result.unwrap(), otp);
	}
}
