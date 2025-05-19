#[cfg(test)]
mod auth_repository_test {
	use crate::{
		AuthOtpSchema, AuthRepository, ResourceEnum, UsersRepository, UsersSchema,
		create_mock_app_state, get_iso_date, make_thing,
	};
	use chrono::{Duration, Utc};
	use imphnen_iam::{RolesDetailQueryDto, UsersDetailQueryDto};
	use surrealdb::Uuid;

	fn create_mock_user(email: &str) -> UsersSchema {
		UsersSchema {
			id: make_thing("app_users", &Uuid::new_v4().to_string()),
			email: email.to_string(),
			fullname: "Test User".to_string(),
			password: "password".to_string(),
			is_deleted: false,
			avatar: None,
			phone_number: "081234567890".to_string(),
			is_active: true,
			gender: None,
			birthdate: None,
			role: make_thing("roles", "user"),
			created_at: get_iso_date(),
			updated_at: get_iso_date(),
		}
	}

	#[tokio::test]
	async fn test_store_and_get_user() {
		let app_state = create_mock_app_state().await;
		let repo = AuthRepository::new(&app_state);
		let user = create_mock_user("forgot@example.com");
		let user_repo = UsersRepository::new(&app_state);
		let user_data = user_repo
			.query_user_by_email("forgot@example.com".to_string())
			.await
			.unwrap();
		let store = repo.query_store_user(user_data.clone()).await;
		assert!(store.is_ok());
		let fetched = repo.query_get_stored_user(user.email.clone()).await;
		assert!(fetched.is_ok());
		assert_eq!(fetched.unwrap().email, user.email);
	}

	#[tokio::test]
	async fn test_delete_stored_user() {
		let state = create_mock_app_state().await;
		let auth_repo = AuthRepository::new(&state);
		let email = "delete_me@example.com".to_string();
		let mock_user = UsersDetailQueryDto {
			id: make_thing(&ResourceEnum::UsersCache.to_string(), &email),
			fullname: "Test User".into(),
			email: email.clone(),
			avatar: None,
			phone_number: "08123456789".into(),
			is_active: true,
			gender: None,
			birthdate: None,
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
			created_at: get_iso_date(),
			updated_at: get_iso_date(),
		};
		let _: Option<UsersDetailQueryDto> = state
			.surrealdb_mem
			.create((ResourceEnum::UsersCache.to_string(), email.clone()))
			.content(mock_user)
			.await
			.unwrap();
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
		let app_state = create_mock_app_state().await;
		let repo = AuthRepository::new(&app_state);
		let email = "otp_user@example.com".to_string();
		let otp = 123456;
		let stored = repo.query_store_otp(email.clone(), otp).await;
		assert!(stored.is_ok());
		let fetched = repo.query_get_stored_otp(email.clone()).await;
		assert!(fetched.is_ok());
		assert_eq!(fetched.unwrap(), otp);
	}

	#[tokio::test]
	async fn test_delete_stored_otp() {
		let app_state = create_mock_app_state().await;
		let repo = AuthRepository::new(&app_state);
		let email = "otp_del@example.com".to_string();
		let otp = 654321;
		repo.query_store_otp(email.clone(), otp).await.unwrap();
		let deleted = repo.query_delete_stored_otp(email.clone()).await;
		assert!(deleted.is_ok());
		let fetched = repo.query_get_stored_otp(email.clone()).await;
		assert!(fetched.is_err());
	}

	#[tokio::test]
	async fn test_expired_otp() {
		let app_state = create_mock_app_state().await;
		let repo = AuthRepository::new(&app_state);
		let email = "expired_otp@example.com".to_string();
		let otp = 789012;
		let table = ResourceEnum::OtpCache.to_string();
		let expires_at = Utc::now() - Duration::seconds(1);
		let _: Option<AuthOtpSchema> = repo
			.state
			.surrealdb_mem
			.create((table.clone(), email.as_str()))
			.content(AuthOtpSchema { otp, expires_at })
			.await
			.unwrap();
		let result = repo.query_get_stored_otp(email.clone()).await;
		assert!(result.is_err());
	}

	#[tokio::test]
	async fn test_get_non_existent_stored_user_should_fail() {
		let app_state = create_mock_app_state().await;
		let repo = AuthRepository::new(&app_state);
		let result = repo
			.query_get_stored_user("not_found@example.com".into())
			.await;
		assert!(result.is_err());
	}

	#[tokio::test]
	async fn test_delete_non_existent_user_should_fail() {
		let app_state = create_mock_app_state().await;
		let repo = AuthRepository::new(&app_state);
		let result = repo
			.query_delete_stored_user("ghost@example.com".into())
			.await;
		assert!(result.is_err());
	}

	#[tokio::test]
	async fn test_get_expired_otp_should_fail() {
		use chrono::Duration;
		let app_state = create_mock_app_state().await;
		let repo = AuthRepository::new(&app_state);
		let email = "expired_otp@example.com";
		let expired_time = chrono::Utc::now() - Duration::seconds(10);
		let otp = 123456;
		let _: Option<AuthOtpSchema> = repo
			.state
			.surrealdb_mem
			.create((ResourceEnum::OtpCache.to_string(), email))
			.content(AuthOtpSchema {
				otp,
				expires_at: expired_time,
			})
			.await
			.unwrap();

		let result = repo.query_get_stored_otp(email.into()).await;
		assert!(result.is_err());
	}

	#[tokio::test]
	async fn test_store_and_get_valid_otp() {
		let app_state = create_mock_app_state().await;
		let repo = AuthRepository::new(&app_state);
		let email = "valid_otp@example.com";
		let otp = 654321;
		let store_result = repo.query_store_otp(email.into(), otp).await;
		assert!(store_result.is_ok());
		let get_result = repo.query_get_stored_otp(email.into()).await;
		assert_eq!(get_result.unwrap(), otp);
	}
}
