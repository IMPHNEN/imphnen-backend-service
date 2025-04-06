use crate::{create_mock_app_state, create_test_user, RolesRepository};
use crate::{
	MetaRequestDto, UsersActiveInactiveSchema, UsersRepository,
	UsersSetNewPasswordSchema,
};

async fn get_role_id(state: &crate::AppState) -> String {
	RolesRepository::new(state)
		.query_role_by_name("Student".into())
		.await
		.expect("Role not found")
		.id
}

#[tokio::test]
async fn test_create_and_get_user() {
	let app_state = create_mock_app_state().await;
	let repo = UsersRepository::new(&app_state);
	let user = create_test_user(
		"testuser@example.com",
		"Test User",
		true,
		&get_role_id(&app_state).await,
	);
	let create_result = repo.query_create_user(user.clone()).await;
	assert!(create_result.is_ok());
	let fetched = repo
		.query_user_by_email("testuser@example.com".into())
		.await;
	assert!(fetched.is_ok());
	assert_eq!(fetched.unwrap().email, "testuser@example.com");
}

#[tokio::test]
async fn test_update_password_user() {
	let app_state = create_mock_app_state().await;
	let repo = UsersRepository::new(&app_state);
	let role_repo = RolesRepository::new(&app_state);
	let role_id = role_repo
		.query_role_by_name("Student".into())
		.await
		.expect("Role not found")
		.id;
	let email = "changepass@example.com";
	let user = create_test_user(email, "Change Password", true, &role_id);
	repo.query_create_user(user).await.unwrap();
	let result = repo
		.query_update_password_user(
			email.to_string(),
			UsersSetNewPasswordSchema {
				password: "newpass".into(),
			},
		)
		.await;
	assert!(
		result.is_ok(),
		"Update password failed with error: {:?}",
		result.err()
	);
	dbg!(result.unwrap());
}

#[tokio::test]
async fn test_query_user_list_with_pagination_and_filter() {
	let app_state = create_mock_app_state().await;
	let repo = UsersRepository::new(&app_state);
	for i in 0..10 {
		let email = format!("user{}@example.com", i);
		let fullname = format!("User {}", i);
		let is_active = i % 2 == 0;
		let user =
			create_test_user(&email, &fullname, is_active, &get_role_id(&app_state).await);
		repo.query_create_user(user).await.unwrap();
	}
	let meta = MetaRequestDto {
		page: Some(1),
		per_page: Some(5),
		search: None,
		sort_by: Some("email".into()),
		order: Some("ASC".into()),
		filter: Some("true".into()),
		filter_by: Some("is_active".into()),
	};
	let result = repo.query_user_list(meta).await.unwrap();
	assert!(result.data.len() <= 5);
	assert!(result.data.iter().all(|u| u.is_active));
	assert!(result.meta.as_ref().unwrap().total.is_some());
}

#[tokio::test]
async fn test_query_user_list_basic() {
	let app_state = create_mock_app_state().await;
	let repo = UsersRepository::new(&app_state);
	for i in 0..10 {
		let email = format!("basic{}@example.com", i);
		let user = create_test_user(
			&email,
			&format!("Basic User {}", i),
			true,
			&get_role_id(&app_state).await,
		);
		repo.query_create_user(user).await.unwrap();
	}
	let meta = MetaRequestDto {
		page: Some(1),
		per_page: Some(5),
		search: None,
		sort_by: None,
		order: None,
		filter: None,
		filter_by: None,
	};
	let result = repo.query_user_list(meta).await.unwrap();
	assert!(result.meta.as_ref().unwrap().total.unwrap() >= 1);
	assert_eq!(result.meta.as_ref().unwrap().page.unwrap(), 1);
	assert_eq!(result.meta.as_ref().unwrap().per_page.unwrap(), 5);
}

#[tokio::test]
async fn test_query_active_inactive_user() {
	let app_state = create_mock_app_state().await;
	let repo = UsersRepository::new(&app_state);
	let email = "inactive@example.com";
	let user = create_test_user(
		email,
		"Inactive User",
		false,
		&get_role_id(&app_state).await,
	);
	repo.query_create_user(user).await.unwrap();
	let result = repo
		.query_active_inactive_user(
			email.into(),
			UsersActiveInactiveSchema { is_active: true },
		)
		.await;
	assert!(result.is_ok());
	let updated = repo
		.query_user_by_email("inactive@example.com".into())
		.await
		.unwrap();
	assert_eq!(updated.is_active, true);
}

#[tokio::test]
async fn test_query_user_by_invalid_email_should_fail() {
	let app_state = create_mock_app_state().await;
	let repo = UsersRepository::new(&app_state);
	let result = repo
		.query_user_by_email("nonexistent@example.com".into())
		.await;
	assert!(result.is_err());
}

#[tokio::test]
async fn test_query_update_password_for_nonexistent_user_should_fail() {
	let app_state = create_mock_app_state().await;
	let repo = UsersRepository::new(&app_state);
	let result = repo
		.query_update_password_user(
			"ghost@example.com".into(),
			UsersSetNewPasswordSchema {
				password: "secret".into(),
			},
		)
		.await;
	assert!(result.is_err());
}

#[tokio::test]
async fn test_query_delete_user() {
	let app_state = create_mock_app_state().await;
	let repo = UsersRepository::new(&app_state);
	let email = "deleteuser@example.com";
	let user =
		create_test_user(email, "Delete User", true, &get_role_id(&app_state).await);
	repo.query_create_user(user.clone()).await.unwrap();
	let user_detail = repo
		.query_user_by_email(email.to_string().clone())
		.await
		.unwrap();
	let delete_result = repo
		.query_delete_user(user_detail.id.id.to_raw().clone())
		.await;
	assert!(delete_result.is_ok());
	let fetch_result = repo
		.query_user_by_email(user_detail.id.id.to_raw().clone())
		.await;
	assert!(fetch_result.is_err());
}

#[tokio::test]
async fn test_delete_non_existent_user_should_fail() {
	let app_state = create_mock_app_state().await;
	let repo = UsersRepository::new(&app_state);
	let result = repo.query_delete_user("lklklklk".to_string()).await;
	assert!(result.is_err());
	assert_eq!(result.unwrap_err().to_string(), "User not found");
}

#[tokio::test]
async fn test_delete_user_twice_should_fail_on_second_attempt() {
	let app_state = create_mock_app_state().await;
	let repo = UsersRepository::new(&app_state);
	let email = "twice@example.com";
	let user =
		create_test_user(email, "Delete Twice", true, &get_role_id(&app_state).await);
	repo.query_create_user(user.clone()).await.unwrap();
	let first = repo.query_delete_user(user.id.id.to_raw()).await;
	assert!(first.is_ok());
	let second = repo.query_delete_user(user.id.id.to_raw()).await;
	assert!(second.is_err());
	assert_eq!(second.unwrap_err().to_string(), "User already deleted");
}

#[tokio::test]
async fn test_query_update_user_should_succeed() {
	let state = create_mock_app_state().await;
	let repo = UsersRepository::new(&state);
	let mut user = create_test_user(
		"update@example.com",
		"Old Name",
		true,
		&get_role_id(&state).await,
	);
	repo.query_create_user(user.clone()).await.unwrap();
	user.fullname = "Updated Name".into();
	user.phone_number = "089876543210".into();
	let result = repo.query_update_user(user.clone()).await;
	assert!(result.is_ok(), "Update failed: {:?}", result.err());
	let updated = repo.query_user_by_id(user.id.id.to_raw()).await.unwrap();
	assert_eq!(updated.fullname, "Updated Name");
	assert_eq!(updated.phone_number, "089876543210");
}
