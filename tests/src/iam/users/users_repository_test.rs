use crate::{MetaRequestDto, UsersRepository, generate_unique_email, get_role_id};
use crate::{create_mock_app_state, create_test_user};

#[tokio::test]
async fn test_create_and_get_user() {
	let app_state = create_mock_app_state().await;
	let repo = UsersRepository::new(&app_state);
	let email = generate_unique_email("testuser");
	let user =
		create_test_user(&email, "Test User", true, &get_role_id(&app_state).await);
	let create_result = repo.query_create_user(user.clone()).await;
	assert!(create_result.is_ok());
	let fetched = repo.query_user_by_email(email.clone().into()).await;
	assert!(fetched.is_ok());
	assert_eq!(fetched.unwrap().email, email.clone());
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
async fn test_query_delete_user() {
	let app_state = create_mock_app_state().await;
	let repo = UsersRepository::new(&app_state);
	let email = &generate_unique_email("deleteuser");
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
	let fetch_result = repo.query_user_by_email(user_detail.email.clone()).await;
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
	assert_eq!(second.unwrap_err().to_string(), "User not found");
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
