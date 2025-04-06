use crate::{
	apps::v1::users_router, create_mock_app_state, RolesEnum, RolesRepository,
	UsersActiveInactiveRequestDto, UsersCreateRequestDto, UsersRepository,
	UsersUpdateRequestDto,
};
use axum::{http::StatusCode, Extension};
use axum_test::TestServer;
use surrealdb::Uuid;

#[tokio::test]
async fn test_get_user_list_should_return_200() {
	let state = create_mock_app_state().await;
	let app = axum::Router::new()
		.nest("/v1/users", users_router())
		.layer(Extension(state));
	let server = TestServer::new(app).unwrap();
	let res = server.get("/v1/users?page=1&per_page=10").await;
	let status = res.status_code();
	assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn test_list_users_should_fail_with_invalid_per_page() {
	let state = create_mock_app_state().await;
	let app = axum::Router::new()
		.nest("/v1/users", users_router())
		.layer(Extension(state));
	let server = TestServer::new(app).unwrap();
	let res = server.get("/v1/users?page=1&per_page=0").await;
	assert_eq!(res.status_code(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_list_users_should_fail_with_invalid_page() {
	let state = create_mock_app_state().await;
	let app = axum::Router::new()
		.nest("/v1/users", users_router())
		.layer(Extension(state));
	let server = TestServer::new(app).unwrap();
	let res = server.get("/v1/users?page=0&per_page=10").await;
	dbg!(res.text());
	dbg!(res.status_code());
	assert_eq!(res.status_code(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_list_users_should_ignore_invalid_sort_field() {
	let state = create_mock_app_state().await;
	let app = axum::Router::new()
		.nest("/v1/users", users_router())
		.layer(Extension(state));
	let server = TestServer::new(app).unwrap();
	let res = server
		.get("/v1/users?page=1&per_page=10&sort_by=invalid_field")
		.await;
	assert_eq!(res.status_code(), StatusCode::OK);
}

#[tokio::test]
async fn test_list_users_with_search_no_match_should_return_empty() {
	let state = create_mock_app_state().await;
	let app = axum::Router::new()
		.nest("/v1/users", users_router())
		.layer(Extension(state));
	let server = TestServer::new(app).unwrap();
	let res = server
		.get("/v1/users?page=1&per_page=10&search=nonexistinguserxyz")
		.await;
	let body: serde_json::Value = res.json();
	assert_eq!(res.status_code(), StatusCode::OK);
	assert_eq!(body["data"].as_array().unwrap().len(), 0);
}

#[tokio::test]
async fn test_list_users_should_return_empty_on_invalid_filter() {
	let state = create_mock_app_state().await;
	let app = axum::Router::new()
		.nest("/v1/users", users_router())
		.layer(Extension(state));
	let server = TestServer::new(app).unwrap();
	let res = server
		.get("/v1/users?page=1&per_page=10&filter_by=is_active&filter=maybe")
		.await;
	let body: serde_json::Value = res.json();
	assert_eq!(res.status_code(), StatusCode::OK);
	assert_eq!(body["data"].as_array().unwrap().len(), 0);
}

#[tokio::test]
async fn test_get_user_list_with_search_should_return_200() {
	let state = create_mock_app_state().await;
	let app = axum::Router::new()
		.nest("/v1/users", users_router())
		.layer(Extension(state));
	let server = TestServer::new(app).unwrap();
	let res = server
		.get("/v1/users?page=1&per_page=10&search=maulana")
		.await;
	let status = res.status_code();
	dbg!(res.text());
	assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn test_create_user_should_return_201() {
	let state = create_mock_app_state().await;
	let role_repo = RolesRepository::new(&state);
	let role = role_repo
		.query_role_by_name(RolesEnum::User.to_string())
		.await
		.unwrap();
	let role_id = role.id;
	let app = axum::Router::new()
		.nest("/v1/users", users_router())
		.layer(Extension(state));
	let server = TestServer::new(app).unwrap();
	let payload = UsersCreateRequestDto {
		fullname: "Create Data #1".into(),
		email: format!("test-{}@create.com", Uuid::new_v4()).into(),
		role_id,
		password: "Password1!".into(),
		phone_number: "081234567890".into(),
		is_active: true,
	};
	let res = server.post("/v1/users/create").json(&payload).await;
	assert_eq!(res.status_code(), StatusCode::CREATED);
}

#[tokio::test]
async fn test_get_user_detail_should_return_200() {
	let state = create_mock_app_state().await;
	let role_repo = RolesRepository::new(&state);
	let role = role_repo
		.query_role_by_name(RolesEnum::User.to_string())
		.await
		.unwrap();
	let role_id = role.id;
	let repo = UsersRepository::new(&state);
	let app = axum::Router::new()
		.nest("/v1/users", users_router())
		.layer(Extension(state.clone()));
	let server = TestServer::new(app).unwrap();
	let payload = UsersCreateRequestDto {
		fullname: "Detail User".into(),
		email: "detail@test.com".into(),
		role_id,
		password: "Password1!".into(),
		phone_number: "081234567890".into(),
		is_active: true,
	};
	server.post("/v1/users/create").json(&payload).await;
	let user = repo.query_user_by_email(payload.email).await.unwrap();
	let user_id = user.id.id.to_raw();
	let res = server.get(&format!("/v1/users/detail/{}", user_id)).await;
	assert_eq!(res.status_code(), StatusCode::OK);
}

#[tokio::test]
async fn test_delete_user_should_return_200() {
	let state = create_mock_app_state().await;
	let role_repo = RolesRepository::new(&state);
	let role = role_repo
		.query_role_by_name(RolesEnum::User.to_string())
		.await
		.expect("Failed to get role");
	let repo = UsersRepository::new(&state);
	let app = axum::Router::new()
		.nest("/v1/users", users_router())
		.layer(Extension(state.clone()));
	let server = TestServer::new(app).unwrap();
	let payload = UsersCreateRequestDto {
		fullname: "Test Delete Me".into(),
		email: "test@delete-me.com".into(),
		password: "Password1!".into(),
		role_id: role.id.clone(),
		phone_number: "081234567890".into(),
		is_active: true,
	};
	let create_res = server.post("/v1/users/create").json(&payload).await;
	assert_eq!(create_res.status_code(), StatusCode::CREATED);
	let detail_res = repo.query_user_by_email(payload.email.clone()).await;
	assert!(detail_res.is_ok(), "User not found after creation");
	let user = detail_res.unwrap();
	let user_id_for_delete = user.id.id.to_raw();
	let delete_res = server
		.delete(&format!("/v1/users/delete/{}", user_id_for_delete))
		.await;
	assert_eq!(delete_res.status_code(), StatusCode::OK);
}

#[tokio::test]
async fn test_activate_user_should_return_200() {
	let state = create_mock_app_state().await;
	let role_repo = RolesRepository::new(&state);
	let role = role_repo
		.query_role_by_name(RolesEnum::User.to_string())
		.await
		.unwrap();
	let role_id = role.id;
	let repo = UsersRepository::new(&state);
	let app = axum::Router::new()
		.nest("/v1/users", users_router())
		.layer(Extension(state.clone()));
	let server = TestServer::new(app).unwrap();
	let payload = UsersCreateRequestDto {
		fullname: "Inactive User".into(),
		email: format!("inactive-{}@test.com", Uuid::new_v4()).into(),
		password: "Password1!".into(),
		role_id,
		phone_number: "081234567890".into(),
		is_active: false,
	};
	let res_create = server.post("/v1/users/create").json(&payload).await;
	dbg!(res_create.text());
	let user = repo.query_user_by_email(payload.email).await.unwrap();
	dbg!(user.fullname);
	let user_id = user.id.id.to_raw();
	let res = server
		.put(&format!("/v1/users/activate/{}", user_id))
		.json(&UsersActiveInactiveRequestDto { is_active: true })
		.await;
	dbg!(res.text());
	assert_eq!(res.status_code(), StatusCode::OK);
}

#[tokio::test]
async fn test_update_user_should_return_200() {
	let state = create_mock_app_state().await;
	let role_repo = RolesRepository::new(&state);
	let role = role_repo
		.query_role_by_name(RolesEnum::User.to_string())
		.await
		.unwrap();
	let role_id = role.id.clone();
	let repo = UsersRepository::new(&state);
	let app = axum::Router::new()
		.nest("/v1/users", users_router())
		.layer(Extension(state.clone()));
	let server = TestServer::new(app).unwrap();
	let unique_email = format!("update_{}@test.com", Uuid::new_v4());
	let payload = UsersCreateRequestDto {
		fullname: "Old Name".into(),
		email: unique_email.clone(),
		password: "Password1!".into(),
		role_id: role_id.clone(),
		phone_number: "081234567890".into(),
		is_active: true,
	};
	let create_res = server.post("/v1/users/create").json(&payload).await;
	assert_eq!(create_res.status_code(), StatusCode::CREATED);
	let user = repo.query_user_by_email(unique_email).await.unwrap();
	let user_id = user.id.id.to_raw();
	let update_payload = UsersUpdateRequestDto {
		fullname: "Updated Name".into(),
		email: payload.email.clone(),
		phone_number: "081234567890".into(),
		role_id: user.role.id.id.to_raw(),
		is_active: true,
		gender: Some("Laki-laki".into()),
		birthdate: Some("2000-01-01".into()),
		avatar: None,
	};
	let res = server
		.put(&format!("/v1/users/update/{}", user_id))
		.json(&update_payload)
		.await;
	let status = res.status_code();
	let body = res.text();
	assert_eq!(status, StatusCode::OK, "Response body: {}", body);
}

#[tokio::test]
async fn test_create_user_should_fail_if_email_taken() {
	let state = create_mock_app_state().await;
	let role_repo = RolesRepository::new(&state);
	let role = role_repo
		.query_role_by_name(RolesEnum::User.to_string())
		.await
		.unwrap();
	let role_id = role.id.clone();
	let app = axum::Router::new()
		.nest("/v1/users", users_router())
		.layer(Extension(state.clone()));
	let server = TestServer::new(app).unwrap();
	let email = format!("test_{}@example.com", Uuid::new_v4());
	let payload = UsersCreateRequestDto {
		fullname: "User Satu".into(),
		email: email.clone(),
		password: "Password1!".into(),
		role_id,
		phone_number: "081234567890".into(),
		is_active: true,
	};
	let res1 = server.post("/v1/users/create").json(&payload).await;
	assert_eq!(res1.status_code(), StatusCode::CREATED);
	let res2 = server.post("/v1/users/create").json(&payload).await;
	let body2 = res2.text();
	assert_eq!(res2.status_code(), StatusCode::BAD_REQUEST);
	assert!(body2.contains("User already exists"));
}

#[tokio::test]
async fn test_get_user_detail_should_fail_if_not_found() {
	let state = create_mock_app_state().await;
	let app = axum::Router::new()
		.nest("/v1/users", users_router())
		.layer(Extension(state));
	let server = TestServer::new(app).unwrap();
	let res = server.get("/v1/users/detail/non-existent-id").await;
	assert_eq!(res.status_code(), StatusCode::NOT_FOUND);
	let body = res.text();
	assert!(body.contains("User not found"));
}

#[tokio::test]
async fn test_delete_user_should_fail_if_already_deleted() {
	let state = create_mock_app_state().await;
	let role_repo = RolesRepository::new(&state);
	let role = role_repo
		.query_role_by_name(RolesEnum::User.to_string())
		.await
		.unwrap();
	let role_id = role.id.clone();
	let repo = UsersRepository::new(&state);
	let app = axum::Router::new()
		.nest("/v1/users", users_router())
		.layer(Extension(state.clone()));
	let server = TestServer::new(app).unwrap();
	let payload = UsersCreateRequestDto {
		fullname: "To Be Deleted".into(),
		email: "delete-me@example.com".into(),
		role_id,
		password: "Password1!".into(),
		phone_number: "081234567890".into(),
		is_active: true,
	};
	server.post("/v1/users/create").json(&payload).await;
	let user = repo.query_user_by_email(payload.email).await.unwrap();
	let id = user.id.id.to_raw();
	let res1 = server.delete(&format!("/v1/users/delete/{}", id)).await;
	assert_eq!(res1.status_code(), StatusCode::OK);
	let res2 = server.delete(&format!("/v1/users/delete/{}", id)).await;
	assert_eq!(res2.status_code(), StatusCode::BAD_REQUEST);
	let body = res2.text();
	assert!(body.contains("User already deleted"));
}

#[tokio::test]
async fn test_update_user_should_fail_if_user_not_found() {
	let state = create_mock_app_state().await;
	let role_repo = RolesRepository::new(&state);
	let role = role_repo
		.query_role_by_name(RolesEnum::User.to_string())
		.await
		.unwrap();
	let role_id = role.id.clone();
	let app = axum::Router::new()
		.nest("/v1/users", users_router())
		.layer(Extension(state));
	let server = TestServer::new(app).unwrap();
	let update_payload = UsersUpdateRequestDto {
		fullname: "Does Not Exist".into(),
		email: "nonexistent@test.com".into(),
		phone_number: "081234567890".into(),
		role_id,
		is_active: true,
		gender: Some("Laki-laki".into()),
		birthdate: Some("2000-01-01".into()),
		avatar: None,
	};
	let res = server
		.put("/v1/users/update/non-existent-id")
		.json(&update_payload)
		.await;
	assert_eq!(res.status_code(), StatusCode::BAD_REQUEST);
	let body = res.text();
	assert!(body.contains("User not found"));
}

#[tokio::test]
async fn test_activate_user_should_fail_if_user_not_found() {
	let state = create_mock_app_state().await;
	let app = axum::Router::new()
		.nest("/v1/users", users_router())
		.layer(Extension(state));
	let server = TestServer::new(app).unwrap();
	let payload = UsersActiveInactiveRequestDto { is_active: true };
	let res = server
		.put("/v1/users/activate/non-existent-id")
		.json(&payload)
		.await;
	assert_eq!(res.status_code(), StatusCode::BAD_REQUEST);
	let body = res.text();
	assert!(body.contains("User not found"));
}

#[tokio::test]
async fn test_create_user_should_fail_if_payload_invalid() {
	let state = create_mock_app_state().await;
	let app = axum::Router::new()
		.nest("/v1/users", users_router())
		.layer(Extension(state));
	let server = TestServer::new(app).unwrap();
	let invalid_payload = serde_json::json!({});
	let res = server.post("/v1/users/create").json(&invalid_payload).await;
	assert_eq!(res.status_code(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn test_list_users_should_fail_with_invalid_pagination() {
	let state = create_mock_app_state().await;
	let app = axum::Router::new()
		.nest("/v1/users", users_router())
		.layer(Extension(state));
	let server = TestServer::new(app).unwrap();
	let res = server.get("/v1/users?page=0&per_page=0").await;
	assert_eq!(res.status_code(), StatusCode::BAD_REQUEST);
	let body = res.text();
	assert!(
		body.contains("Invalid pagination") || body.contains("per_page"),
		"Expected pagination error, got: {body}"
	);
}

#[tokio::test]
async fn test_list_users_should_fallback_on_invalid_order() {
	let state = create_mock_app_state().await;
	let app = axum::Router::new()
		.nest("/v1/users", users_router())
		.layer(Extension(state));
	let server = TestServer::new(app).unwrap();
	let res = server
		.get("/v1/users?page=1&per_page=10&sort_by=email&order=invalid")
		.await;
	assert_eq!(res.status_code(), StatusCode::OK);
}

#[tokio::test]
async fn test_list_users_with_invalid_filter_by_should_return_empty() {
	let state = create_mock_app_state().await;
	let app = axum::Router::new()
		.nest("/v1/users", users_router())
		.layer(Extension(state));
	let server = TestServer::new(app).unwrap();
	let res = server
		.get("/v1/users?page=1&per_page=10&filter_by=unknown_field&filter=value")
		.await;
	assert_eq!(res.status_code(), StatusCode::OK);
	let body: serde_json::Value = res.json();
	assert_eq!(body["data"].as_array().unwrap().len(), 0);
}

#[tokio::test]
async fn test_user_detail_should_fail_with_invalid_id_format() {
	let state = create_mock_app_state().await;
	let app = axum::Router::new()
		.nest("/v1/users", users_router())
		.layer(Extension(state));
	let server = TestServer::new(app).unwrap();
	let res = server.get("/v1/users/detail/!@#invalid-id").await;
	assert_eq!(res.status_code(), StatusCode::NOT_FOUND);
	let body = res.text();
	assert!(body.contains("User not found") || body.contains("not found"));
}

#[tokio::test]
async fn test_user_detail_should_fail_if_user_not_found() {
	let state = create_mock_app_state().await;
	let app = axum::Router::new()
		.nest("/v1/users", users_router())
		.layer(Extension(state));
	let server = TestServer::new(app).unwrap();
	let res = server.get("/v1/users/detail/random-id-123456").await;
	assert_eq!(res.status_code(), StatusCode::NOT_FOUND);
	let body = res.text();
	assert!(body.contains("User not found"));
}

#[tokio::test]
async fn test_user_detail_should_fail_if_user_is_soft_deleted() {
	let state = create_mock_app_state().await;
	let role_repo = RolesRepository::new(&state);
	let role = role_repo
		.query_role_by_name(RolesEnum::User.to_string())
		.await
		.unwrap();
	let role_id = role.id.clone();
	let repo = UsersRepository::new(&state);
	let app = axum::Router::new()
		.nest("/v1/users", users_router())
		.layer(Extension(state.clone()));
	let server = TestServer::new(app).unwrap();
	let payload = UsersCreateRequestDto {
		role_id,
		fullname: "Soft Deleted".into(),
		email: "softdeleted@test.com".into(),
		password: "Password1!".into(),
		phone_number: "081234567890".into(),
		is_active: true,
	};
	server.post("/v1/users/create").json(&payload).await;
	let user = repo.query_user_by_email(payload.email).await.unwrap();
	let user_id = user.id.id.to_raw();
	let _ = repo.query_delete_user(user_id.clone()).await.unwrap();
	let res = server.get(&format!("/v1/users/detail/{}", user_id)).await;
	dbg!(res.text());
	assert_eq!(res.status_code(), StatusCode::NOT_FOUND);
	let body = res.text();
	assert!(body.contains("User not found"));
}

#[tokio::test]
async fn test_update_user_should_fail_if_user_is_deleted() {
	let state = create_mock_app_state().await;
	let role_repo = RolesRepository::new(&state);
	let role = role_repo
		.query_role_by_name(RolesEnum::User.to_string())
		.await
		.unwrap();
	let role_id = role.id.clone();
	let repo = UsersRepository::new(&state);
	let app = axum::Router::new()
		.nest("/v1/users", users_router())
		.layer(Extension(state.clone()));
	let server = TestServer::new(app).unwrap();
	let payload = UsersCreateRequestDto {
		role_id: role_id.clone(),
		fullname: "To Be Deleted".into(),
		email: "deleteduser@test.com".into(),
		password: "Password1!".into(),
		phone_number: "081234567890".into(),
		is_active: true,
	};
	server.post("/v1/users/create").json(&payload).await;
	let user = repo.query_user_by_email(payload.email).await.unwrap();
	let user_id = user.id.id.to_raw();
	let _ = repo.query_delete_user(user_id.clone()).await.unwrap();
	let update_payload = UsersUpdateRequestDto {
		role_id: role_id.clone(),
		fullname: "Should Fail".into(),
		email: "deleteduser@test.com".into(),
		phone_number: "081234567890".into(),
		is_active: true,
		gender: Some("Laki-laki".into()),
		birthdate: Some("2000-01-01".into()),
		avatar: None,
	};
	let res = server
		.put(&format!("/v1/users/update/{}", user_id))
		.json(&update_payload)
		.await;
	assert_eq!(res.status_code(), StatusCode::BAD_REQUEST);
	let body = res.text();
	assert!(body.contains("User already deleted"));
}

#[tokio::test]
async fn test_update_user_should_fail_if_payload_invalid() {
	let state = create_mock_app_state().await;
	let role_repo = RolesRepository::new(&state);
	let role = role_repo
		.query_role_by_name(RolesEnum::User.to_string())
		.await
		.unwrap();
	let role_id = role.id.clone();
	let repo = UsersRepository::new(&state);
	let app = axum::Router::new()
		.nest("/v1/users", users_router())
		.layer(Extension(state.clone()));
	let server = TestServer::new(app).unwrap();
	let payload = UsersCreateRequestDto {
		fullname: "Invalid Payload".into(),
		email: "invalid@test.com".into(),
		role_id: role_id.clone(),
		password: "Password1!".into(),
		phone_number: "081234567890".into(),
		is_active: true,
	};
	server.post("/v1/users/create").json(&payload).await;
	let user = repo.query_user_by_email(payload.email).await.unwrap();
	let user_id = user.id.id.to_raw();
	let update_payload = UsersUpdateRequestDto {
		fullname: "".into(),
		email: "invalid@test.com".into(),
		phone_number: "081234567890".into(),
		is_active: true,
		role_id: role_id.clone(),
		gender: Some("Laki-laki".into()),
		birthdate: Some("2000-01-01".into()),
		avatar: None,
	};
	let res = server
		.put(&format!("/v1/users/update/{}", user_id))
		.json(&update_payload)
		.await;
	assert_eq!(res.status_code(), StatusCode::BAD_REQUEST);
	let body = res.text();
	assert!(
		body.contains("Fullname") || body.contains("at least have 2 character"),
		"Gagal validasi fullname: {body}"
	);
}
