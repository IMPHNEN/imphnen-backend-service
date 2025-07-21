use crate::{
	get_iso_date, make_thing,
	permissions::{
		permissions_repository::PermissionsRepository,
		permissions_schema::PermissionsSchema,
	},
	roles::{
		roles_dto::{RolesRequestCreateDto, RolesRequestUpdateDto},
		roles_repository::RolesRepository,
	},
	setup_all_test_environment, ResourceEnum,
};
use surrealdb::Uuid;

fn generate_unique_name(prefix: &str) -> String {
	format!("{}_{}", prefix, Uuid::new_v4())
}

#[tokio::test]
async fn test_query_create_role_should_succeed() {
	let state = setup_all_test_environment().await;
	let perm_repo = PermissionsRepository::new(&state);
	let role_repo = RolesRepository::new(&state);
	let perm_id = Uuid::new_v4().to_string();
	let permission = PermissionsSchema {
		id: make_thing(&ResourceEnum::Permissions.to_string(), &perm_id),
		name: generate_unique_name("read_quiz"),
		is_deleted: false,
		created_at: Some(get_iso_date()),
		updated_at: Some(get_iso_date()),
	};
	let perm_res = perm_repo.query_create_permission(permission).await;
	assert!(
		perm_res.is_ok(),
		"Failed to create permission: {:?}",
		perm_res.err()
	);
	let payload = RolesRequestCreateDto {
		name: generate_unique_name("user"),
		permissions: vec![perm_id.clone()],
	};
	let result = role_repo.query_create_role(payload).await;
	assert!(result.is_ok(), "Failed to create role: {:?}", result.err());
}

#[tokio::test]
async fn test_query_role_by_name_should_return_data() {
	let state = setup_all_test_environment().await;
	let role_repo = RolesRepository::new(&state);
	let name = generate_unique_name("viewer");
	let payload = RolesRequestCreateDto {
		name: name.clone(),
		permissions: vec![],
	};
	let create_res = role_repo.query_create_role(payload.clone()).await;
	assert!(
		create_res.is_ok(),
		"Failed to create role: {:?}",
		create_res.err()
	);
	let role = role_repo.query_role_by_name(name.clone()).await;
	assert!(role.is_ok(), "Failed to get role by name: {:?}", role.err());
	let role = role.unwrap();
	assert_eq!(role.name, name.clone());
}

#[tokio::test]
async fn test_query_role_by_id_should_return_data() {
	let state = setup_all_test_environment().await;
	let role_repo = RolesRepository::new(&state);

	let name = generate_unique_name("tester");

	let payload = RolesRequestCreateDto {
		name: name.clone(),
		permissions: vec![],
	};

	let create_res = role_repo.query_create_role(payload.clone()).await;

	assert!(
		create_res.is_ok(),
		"Failed to create role: {:?}",
		create_res.err()
	);

	let role = role_repo.query_role_by_name(name.clone()).await;

	assert!(role.is_ok(), "Failed to get role by name: {:?}", role.err());
	let role = role.unwrap();

	let result = role_repo.query_role_by_id(role.id.clone()).await;

	assert!(
		result.is_ok(),
		"Failed to get role by id: {:?}",
		result.err()
	);
	let result_role = result.unwrap();

	assert_eq!(result_role.name, name.clone());
}

#[tokio::test]
async fn test_query_update_role_should_update_name_and_permissions() {
	let state = setup_all_test_environment().await;
	let repo = RolesRepository::new(&state);
	let perm_repo = PermissionsRepository::new(&state);
	let original_perm_id = Uuid::new_v4().to_string();
	let original_perm = PermissionsSchema {
		id: make_thing(&ResourceEnum::Permissions.to_string(), &original_perm_id),
		name: generate_unique_name("original_permission"),
		is_deleted: false,
		created_at: Some(crate::get_iso_date()),
		updated_at: Some(crate::get_iso_date()),
	};
	let perm_res = perm_repo.query_create_permission(original_perm).await;
	assert!(
		perm_res.is_ok(),
		"Failed to create original permission: {:?}",
		perm_res.err()
	);
	let role_upadate_name = generate_unique_name("role_for_update");
	let create_payload = RolesRequestCreateDto {
		name: role_upadate_name.clone(),
		permissions: vec![original_perm_id.clone()],
	};
	let create_res = repo.query_create_role(create_payload).await;
	assert!(
		create_res.is_ok(),
		"Failed to create role: {:?}",
		create_res.err()
	);
	let existing_role = repo.query_role_by_name(role_upadate_name.clone()).await;
	assert!(
		existing_role.is_ok(),
		"Failed to get role by name: {:?}",
		existing_role.err()
	);
	let existing_role = existing_role.unwrap();
	let existing_role_id = existing_role.id.clone();
	let new_perm_id = Uuid::new_v4().to_string();
	let new_perm = PermissionsSchema {
		id: make_thing(&ResourceEnum::Permissions.to_string(), &new_perm_id),
		name: "New Permission".into(),
		is_deleted: false,
		created_at: Some(crate::get_iso_date()),
		updated_at: Some(crate::get_iso_date()),
	};
	let new_role_name = generate_unique_name("updated_role_name");
	let perm_res = perm_repo.query_create_permission(new_perm).await;
	assert!(
		perm_res.is_ok(),
		"Failed to create new permission: {:?}",
		perm_res.err()
	);
	let update_payload = RolesRequestUpdateDto {
		name: Some(new_role_name.clone()),
		permissions: Some(vec![new_perm_id.clone()]),
		overwrite: None,
	};
	let update_result = repo
		.query_update_role(existing_role_id.clone(), update_payload)
		.await;
	assert!(
		update_result.is_ok(),
		"Failed to update role: {:?}",
		update_result.err()
	);
	let updated = repo.query_role_by_id(existing_role_id.clone()).await;
	assert!(
		updated.is_ok(),
		"Failed to get updated role: {:?}",
		updated.err()
	);
	assert_eq!(updated.unwrap().name, new_role_name.clone());
}

#[tokio::test]
async fn test_query_delete_role_should_soft_delete() {
	let state = setup_all_test_environment().await;
	let role_repo = RolesRepository::new(&state);
	let name = generate_unique_name("temporary");
	let payload = RolesRequestCreateDto {
		name: name.clone(),
		permissions: vec![],
	};
	let create_res = role_repo.query_create_role(payload.clone()).await;
	assert!(
		create_res.is_ok(),
		"Failed to create role: {:?}",
		create_res.err()
	);
	let role = role_repo.query_role_by_name(name.clone()).await;
	assert!(role.is_ok(), "Failed to get role by name: {:?}", role.err());
	let role = role.unwrap();
	let result = role_repo.query_delete_role(role.id.clone()).await;
	assert!(result.is_ok(), "Failed to delete role: {:?}", result.err());
	let deleted = role_repo.query_role_by_id(role.id).await;
	assert!(
		deleted.is_err(),
		"Role should be deleted, but got: {deleted:?}"
	);
	if let Some(err) = deleted.err() {
		assert!(
			err.to_string().contains("Role not found"),
			"Expected 'Role not found' error, got: {err}"
		);
	}
}

#[tokio::test]
async fn test_query_update_role_should_fallback_to_existing_permissions_if_none_provided(
) {
	let state = setup_all_test_environment().await;
	let repo = RolesRepository::new(&state);
	let perm_repo = PermissionsRepository::new(&state);
	let perm_id = Uuid::new_v4().to_string();
	let permission = PermissionsSchema {
		id: make_thing(&ResourceEnum::Permissions.to_string(), &perm_id),
		name: "Permission for Fallback".into(),
		is_deleted: false,
		created_at: Some(crate::get_iso_date()),
		updated_at: Some(crate::get_iso_date()),
	};
	let perm_res = perm_repo.query_create_permission(permission).await;
	assert!(
		perm_res.is_ok(),
		"Failed to create permission: {:?}",
		perm_res.err()
	);
	let create_payload = RolesRequestCreateDto {
		name: "Role With Permission".into(),
		permissions: vec![perm_id.clone()],
	};
	let create_res = repo.query_create_role(create_payload).await;
	assert!(
		create_res.is_ok(),
		"Failed to create role: {:?}",
		create_res.err()
	);
	let existing = repo.query_role_by_name("Role With Permission".into()).await;
	assert!(
		existing.is_ok(),
		"Failed to get role by name: {:?}",
		existing.err()
	);
	let existing = existing.unwrap();
	let existing_id = existing.id.clone();
	let update_payload = RolesRequestUpdateDto {
		name: Some("Updated Role Name".into()),
		permissions: None,
		overwrite: None,
	};
	let update_res = repo
		.query_update_role(existing_id.clone(), update_payload)
		.await;
	assert!(
		update_res.is_ok(),
		"Failed to update role (fallback): {:?}",
		update_res.err()
	);
}

#[tokio::test]
async fn test_query_role_by_name_should_fail_if_not_found() {
	let state = setup_all_test_environment().await;
	let role_repo = RolesRepository::new(&state);
	let result = role_repo.query_role_by_name("ghost-role".into()).await;
	assert!(result.is_err());
	if let Some(err) = result.err() {
		assert!(
			err.to_string().contains("Role not found"),
			"Expected 'Role not found' error, got: {err}"
		);
	}
}

#[tokio::test]
async fn test_query_delete_role_should_fail_if_already_deleted() {
	let state = setup_all_test_environment().await;
	let role_repo = RolesRepository::new(&state);
	let name = generate_unique_name("soft_delete_test");
	let payload = RolesRequestCreateDto {
		name: name.clone(),
		permissions: vec![],
	};
	let create_res = role_repo.query_create_role(payload.clone()).await;
	assert!(
		create_res.is_ok(),
		"Failed to create role: {:?}",
		create_res.err()
	);
	let role = role_repo.query_role_by_name(name.clone()).await;
	assert!(role.is_ok(), "Failed to get role by name: {:?}", role.err());
	let role = role.unwrap();
	let del_res = role_repo.query_delete_role(role.id.clone()).await;
	assert!(
		del_res.is_ok(),
		"Failed to delete role: {:?}",
		del_res.err()
	);
	let result_fut = role_repo.query_delete_role(role.id);
	let result_val = result_fut.await;
	assert!(
		result_val.is_err(),
		"Role should already be deleted, but got: {result_val:?}"
	);
	if let Some(err) = result_val.err() {
		assert!(
			err.to_string().contains("Role not found"),
			"Expected 'Role not found' error, got: {err}"
		);
	}
}
