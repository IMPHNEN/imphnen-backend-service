#[cfg(test)]
mod auth_repository_test {
	use crate::{
		generate_unique_email,
		get_iso_date,
		get_role_id,
		setup_postgres_test_environment, // Updated setup for PostgreSQL
		AuthOtpSchema,
		AuthRepository,
		UsersRepository,
		UsersSchema,
	};
	use chrono::{Duration, Utc};
	use imphnen_iam::{AppState, UsersDetailQueryDto};
	use imphnen_entities::RolesDetailQueryDto;
	use imphnen_utils::generate_otp::OtpManager;
	use sea_orm::{DatabaseConnection, EntityTrait};
	use uuid::Uuid;

	async fn create_mock_user(state: &AppState, email: &str) -> UsersSchema {
		UsersSchema {
			id: Uuid::new_v4().to_string(), // String id instead of SurrealDB Thing
			email: Some(email.to_string()),
			fullname: Some("Test User".to_string()),
			legal_name: None,
			password: Some("password".to_string()),
			is_deleted: false,
			avatar: None,
			phone_number: Some("081234567890".to_string()),
			phone_for_verification: None,
			is_active: true,
			gender: None,
			birthdate: None,
			domicile: None,
			bio: None,
			last_education: None,
			linkedin_url: None,
			github_url: None,
			cv_url: None,
			portfolio_url: None,
			website_url: None,
			twitter_url: None,
			location: None,
			skills: None,
			experience: None,
			education: None,
			career_status: None,
			role_id: Uuid::parse_str(&get_role_id("user", state).await).ok(),
			mentor_id: None,
			created_at: get_iso_date(),
			updated_at: get_iso_date(),
		}
	}

	#[tokio::test]
	async fn test_store_and_get_user() {
	    let app_state = setup_postgres_test_environment().await; // Updated setup for PostgreSQL
	    let repo = AuthRepository::new(app_state.postgres_pool.clone());
			let email = generate_unique_email("forgot");
			let mut user = create_mock_user(&app_state, &email).await;
			user.role = get_role_id("user", &app_state).await;
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
	    let state = setup_postgres_test_environment().await; // Updated setup for PostgreSQL
	    let auth_repo = AuthRepository::new(state.postgres_pool.clone());
			let email = "delete_me@example.com".to_string();
			let mock_user = UsersDetailQueryDto {
				id: Uuid::new_v4(), // Direct UUID instead of SurrealDB Thing
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
				bio: None,
				last_education: None,
				linkedin_url: None,
				github_url: None,
				cv_url: None,
				portfolio_url: None,
				website_url: None,
				twitter_url: None,
				location: None,
				skills: None,
				experience: None,
				education: None,
				career_status: None,
				role: RolesDetailQueryDto {
					id: Uuid::new_v4(), // Direct UUID instead of SurrealDB Thing
					name: "Dummy Role".into(),
					permissions: Some(vec![]),
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
			// Use SeaORM to create test user instead of SurrealDB
			let user_repo = UsersRepository::new(&state);
			let create_result = user_repo.query_create_user(mock_user.clone().into()).await;
			assert!(
				create_result.is_ok(),
				"Failed to create mock user: {:?}",
				create_result.err()
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
	    let app_state = setup_postgres_test_environment().await; // Updated setup for PostgreSQL
	    let repo = AuthRepository::new(app_state.postgres_pool.clone());
			let email = "otp_user@example.com".to_string();
			let otp_data = OtpManager::generate_otp();
			let stored = repo.query_store_otp(email.clone(), otp_data.clone()).await;
			assert!(stored.is_ok(), "Failed to store OTP: {:?}", stored.err());
			let fetched = repo.query_get_stored_otp(email.clone()).await;
			assert!(fetched.is_ok(), "Failed to fetch OTP: {:?}", fetched.err());
			assert_eq!(fetched.unwrap(), otp_data.code);
	}

	#[tokio::test]
	async fn test_delete_stored_otp() {
	    let app_state = setup_postgres_test_environment().await; // Updated setup for PostgreSQL
	    let repo = AuthRepository::new(app_state.postgres_pool.clone());
			let email = "otp_del@example.com".to_string();
			let otp_data = OtpManager::generate_otp();
			let store_res = repo.query_store_otp(email.clone(), otp_data.clone()).await;
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
	    let app_state = setup_postgres_test_environment().await; // Updated setup for PostgreSQL
	    let repo = AuthRepository::new(app_state.postgres_pool.clone());
			let email = "expired_otp@example.com".to_string();
			let otp_data = OtpManager::generate_otp();
			let expires_at = Utc::now() - Duration::seconds(1);
			
			// Use SeaORM to create expired OTP instead of SurrealDB
			let otp_schema = AuthOtpSchema {
				id: Uuid::new_v4(),
				email: email.clone(),
				otp: otp_data.code,
				hash: otp_data.hash,
				expires_at: Some(expires_at),
				created_at: get_iso_date(),
				updated_at: get_iso_date()
			};
			let create_result = repo.create_otp(otp_schema).await;
			assert!(
				create_result.is_ok(),
				"Failed to create expired OTP: {:?}",
				create_result.err()
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
	    let app_state = setup_postgres_test_environment().await; // Updated setup for PostgreSQL
	    let repo = AuthRepository::new(app_state.postgres_pool.clone());
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
	    let app_state = setup_postgres_test_environment().await; // Updated setup for PostgreSQL
	    let repo = AuthRepository::new(app_state.postgres_pool.clone());
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
	    let app_state = setup_postgres_test_environment().await; // Updated setup for PostgreSQL
	    let repo = AuthRepository::new(app_state.postgres_pool.clone());
			let email = "valid_otp@example.com";
			let otp_data = OtpManager::generate_otp();
			let store_result = repo.query_store_otp(email.into(), otp_data.clone()).await;
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
			assert_eq!(get_result.unwrap(), otp_data.code);
	}
}
