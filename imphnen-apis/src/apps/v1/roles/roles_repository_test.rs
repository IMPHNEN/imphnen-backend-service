use crate::{
	apps::v1::{
		permissions::{
			permissions_repository::PermissionsRepository,
			permissions_schema::PermissionsSchema,
		},
		roles::{
			roles_dto::{RolesRequestCreateDto, RolesRequestUpdateDto},
			roles_repository::RolesRepository,
		},
	},
	create_mock_app_state, get_iso_date, make_thing, ResourceEnum,
};
use surrealdb::sql::Thing;
use surrealdb::Uuid;

#[tokio::test]
async fn test_query_create_role_should_succeed() {
	let state = create_mock_app_state().await;
	let perm_repo = PermissionsRepository::new(&state);
	let role_repo = RolesRepository::new(&state);
	let perm_id = Uuid::new_v4().to_string();
	let permission = PermissionsSchema {
		id: Thing::from((ResourceEnum::Permissions.to_string(), perm_id.clone())),
		name: "Read Quiz".into(),
		is_deleted: false,
		created_at: Some(get_iso_date()),
		updated_at: None,
	};
	perm_repo.query_create_permission(permission).await.unwrap();
	let payload = RolesRequestCreateDto {
		name: "Student".into(),
		permissions: vec![perm_id.clone()],
	};
	let result = role_repo.query_create_role(payload).await;
	assert!(result.is_ok());
}

#[tokio::test]
async fn test_query_role_by_name_should_return_data() {
	let state = create_mock_app_state().await;
	let role_repo = RolesRepository::new(&state);
	let payload = RolesRequestCreateDto {
		name: "Viewer".into(),
		permissions: vec![],
	};
	role_repo.query_create_role(payload.clone()).await.unwrap();
	let role = role_repo.query_role_by_name("Viewer".into()).await.unwrap();
	assert_eq!(role.name, "Viewer");
}

#[tokio::test]
async fn test_query_role_by_id_should_return_data() {
	let state = create_mock_app_state().await;
	let role_repo = RolesRepository::new(&state);
	let payload = RolesRequestCreateDto {
		name: "Tester".into(),
		permissions: vec![],
	};
	role_repo.query_create_role(payload.clone()).await.unwrap();
	let role = role_repo.query_role_by_name("Tester".into()).await.unwrap();
	let result = role_repo.query_role_by_id(role.id).await.unwrap();
	assert_eq!(result.name, "Tester");
}

#[tokio::test]
async fn test_query_update_role_should_update_name_and_permissions() {
	let state = create_mock_app_state().await;
	let repo = RolesRepository::new(&state);
	let perm_repo = PermissionsRepository::new(&state);
	let original_perm_id = Uuid::new_v4().to_string();
	let original_perm = PermissionsSchema {
		id: make_thing(&ResourceEnum::Permissions.to_string(), &original_perm_id),
		name: "Original Permission".into(),
		is_deleted: false,
		created_at: Some(crate::get_iso_date()),
		updated_at: None,
	};
	perm_repo
		.query_create_permission(original_perm)
		.await
		.unwrap();
	let create_payload = RolesRequestCreateDto {
		name: "Role For Update".into(),
		permissions: vec![original_perm_id.clone()],
	};
	repo.query_create_role(create_payload).await.unwrap();
	let existing_role = repo
		.query_role_by_name("Role For Update".into())
		.await
		.unwrap();
	let existing_role_id = existing_role.id.clone();
	let new_perm_id = Uuid::new_v4().to_string();
	let new_perm = PermissionsSchema {
		id: make_thing(&ResourceEnum::Permissions.to_string(), &new_perm_id),
		name: "New Permission".into(),
		is_deleted: false,
		created_at: Some(crate::get_iso_date()),
		updated_at: None,
	};
	perm_repo.query_create_permission(new_perm).await.unwrap();
	let update_payload = RolesRequestUpdateDto {
		name: Some("Updated Role Name".into()),
		permissions: Some(vec![new_perm_id.clone()]),
	};
	let update_result = repo
		.query_update_role(existing_role_id.clone(), update_payload)
		.await;
	assert!(update_result.is_ok());
	let updated = repo
		.query_role_by_id(existing_role_id.clone())
		.await
		.unwrap();
	assert_eq!(updated.name, "Updated Role Name");
}

#[tokio::test]
async fn test_query_delete_role_should_soft_delete() {
	let state = create_mock_app_state().await;
	let role_repo = RolesRepository::new(&state);
	let payload = RolesRequestCreateDto {
		name: "Temporary".into(),
		permissions: vec![],
	};
	role_repo.query_create_role(payload.clone()).await.unwrap();
	let role = role_repo
		.query_role_by_name("Temporary".into())
		.await
		.unwrap();
	let result = role_repo.query_delete_role(role.id.clone()).await;
	assert!(result.is_ok());
	let deleted = role_repo.query_role_by_id(role.id).await;
	assert!(deleted.is_err());
}
#[tokio::test]
async fn test_query_update_role_should_fallback_to_existing_permissions_if_none_provided(
) {
	let state = create_mock_app_state().await;
	let repo = RolesRepository::new(&state);
	let perm_repo = PermissionsRepository::new(&state);
	let perm_id = Uuid::new_v4().to_string();
	let permission = PermissionsSchema {
		id: make_thing(&ResourceEnum::Permissions.to_string(), &perm_id),
		name: "Permission for Fallback".into(),
		is_deleted: false,
		created_at: Some(crate::get_iso_date()),
		updated_at: None,
	};
	perm_repo.query_create_permission(permission).await.unwrap();
	let create_payload = RolesRequestCreateDto {
		name: "Role With Permission".into(),
		permissions: vec![perm_id.clone()],
	};
	repo.query_create_role(create_payload).await.unwrap();
	let existing = repo
		.query_role_by_name("Role With Permission".into())
		.await
		.unwrap();
	let existing_id = existing.id.clone();
	let update_payload = RolesRequestUpdateDto {
		name: Some("Updated Role Name".into()),
		permissions: None,
	};
	let update_res = repo
		.query_update_role(existing_id.clone(), update_payload)
		.await;
	assert!(update_res.is_ok());
}

#[tokio::test]
async fn test_query_role_by_name_should_fail_if_not_found() {
	let state = create_mock_app_state().await;
	let role_repo = RolesRepository::new(&state);
	let result = role_repo.query_role_by_name("ghost-role".into()).await;
	assert!(result.is_err());
}

#[tokio::test]
async fn test_query_delete_role_should_fail_if_already_deleted() {
	let state = create_mock_app_state().await;
	let role_repo = RolesRepository::new(&state);
	let payload = RolesRequestCreateDto {
		name: "SoftDeleteTest".into(),
		permissions: vec![],
	};
	role_repo.query_create_role(payload.clone()).await.unwrap();
	let role = role_repo
		.query_role_by_name("SoftDeleteTest".into())
		.await
		.unwrap();
	role_repo.query_delete_role(role.id.clone()).await.unwrap();
	let result = role_repo.query_delete_role(role.id);
	assert!(result.await.is_err());
}
