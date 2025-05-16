use crate::AppState;
use crate::{
	create_mock_app_state, roles_router, v1::roles_repository::RolesRepository,
	RolesRequestCreateDto, RolesRequestUpdateDto,
};
use axum::http::StatusCode;
use axum::Extension;
use axum::Router;
use axum_test::TestServer;

pub fn create_test_app(state: AppState) -> TestServer {
	let app = Router::new()
		.nest("/v1/roles", roles_router())
		.layer(Extension(state));

	TestServer::new(app).unwrap()
}

async fn delete_dummy_role(state: AppState, id: String) {
	let repo = RolesRepository::new(&state);
	let _ = repo.query_delete_role(id).await;
}

#[tokio::test]
async fn test_get_role_list_should_return_200() {
	let state = create_mock_app_state().await;
	let app = axum::Router::new()
		.nest("/v1/roles", roles_router())
		.layer(axum::Extension(state.clone()));
	let server = TestServer::new(app).unwrap();
	let res = server.get("/v1/roles?page=1&per_page=10").await;
	assert_eq!(res.status_code(), StatusCode::OK);
}

#[tokio::test]
async fn test_post_create_role_should_return_201() {
	let state = create_mock_app_state().await;
	let app = axum::Router::new()
		.nest("/v1/roles", roles_router())
		.layer(axum::Extension(state.clone()));
	let server = TestServer::new(app).unwrap();
	let payload = RolesRequestCreateDto {
		name: "Test Role #1".into(),
		permissions: vec![],
	};
	let res = server.post("/v1/roles/create").json(&payload).await;
	let repo = RolesRepository::new(&state);
	assert_eq!(res.status_code(), StatusCode::CREATED);
	let id = repo
		.query_role_by_name(payload.name.clone())
		.await
		.unwrap()
		.id;
	delete_dummy_role(state, id).await;
}

#[tokio::test]
async fn test_put_update_role_should_return_200() {
	let state = create_mock_app_state().await;
	let repo = RolesRepository::new(&state);
	let _ = repo
		.query_create_role(RolesRequestCreateDto {
			name: "Role To Update".into(),
			permissions: vec![],
		})
		.await
		.unwrap();
	let existing = repo
		.query_role_by_name("Role To Update".into())
		.await
		.unwrap();
	let id = existing.id.clone();
	let server = create_test_app(state.clone());
	let payload = RolesRequestUpdateDto {
		name: Some("Updated Role".into()),
		permissions: Some(vec![]),
	};
	let res = server
		.put(&format!("/v1/roles/update/{}", id))
		.json(&payload)
		.await;
	dbg!(res.text());
	assert_eq!(res.status_code(), StatusCode::OK);
	let id = repo
		.query_role_by_name(payload.name.clone().unwrap())
		.await
		.unwrap()
		.id;
	delete_dummy_role(state.clone(), id).await;
}

#[tokio::test]
async fn test_delete_role_should_return_200() {
	let state = create_mock_app_state().await;
	let repo = RolesRepository::new(&state);
	let _ = repo
		.query_create_role(RolesRequestCreateDto {
			name: "Role To Delete".into(),
			permissions: vec![],
		})
		.await
		.unwrap();
	let existing = repo
		.query_role_by_name("Role To Delete".into())
		.await
		.unwrap();
	let id = existing.id;
	let server = create_test_app(state);
	let res = server.delete(&format!("/v1/roles/delete/{}", id)).await;
	dbg!(res.text());
	assert_eq!(res.status_code(), StatusCode::OK);
}

#[tokio::test]
async fn test_post_create_role_with_empty_name_should_return_400() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state);
	let payload = RolesRequestCreateDto {
		name: "".into(),
		permissions: vec![],
	};
	let res = server.post("/v1/roles/create").json(&payload).await;
	dbg!(res.text());
	assert_eq!(res.status_code(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_put_update_nonexistent_role_should_return_404() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state);
	let payload = RolesRequestUpdateDto {
		name: Some("Does Not Exist".into()),
		permissions: Some(vec![]),
	};
	let res = server
		.put("/v1/roles/update/nonexistent-id")
		.json(&payload)
		.await;
	dbg!(res.text());
	assert_eq!(res.status_code(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_delete_nonexistent_role_should_return_404() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state);
	let res = server.delete("/v1/roles/delete/nonexistent-id").await;
	dbg!(res.text());
	assert_eq!(res.status_code(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_role_list_with_invalid_page_params_should_return_400() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state);
	let res = server.get("/v1/roles?page=abc&per_page=xyz").await;
	dbg!(res.text());
	assert_eq!(res.status_code(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_create_duplicate_role_should_return_409() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state);
	let payload = RolesRequestCreateDto {
		name: "Admin".into(),
		permissions: vec![],
	};
	let res = server.post("/v1/roles/create").json(&payload).await;
	dbg!(res.text());
	assert_eq!(res.status_code(), StatusCode::CONFLICT);
}
