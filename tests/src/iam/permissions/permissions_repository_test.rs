use crate::{
	get_iso_date, // Import the new setup function and get_iso_date
	permissions::{PermissionsRepository, PermissionsSchema},
	setup_all_test_environment,
};
use chrono::Utc;

fn create_dummy_permission(name: &str) -> PermissionsSchema {
	PermissionsSchema {
		name: name.to_string(),
		created_at: Some(get_iso_date()), // Ensure created_at is always set
		updated_at: Some(get_iso_date()), // Ensure updated_at is always set
		..Default::default()
	}
}

#[tokio::test]
async fn test_create_permission_should_succeed() {
	let state = setup_all_test_environment().await; // Use the new setup function
	let repo = PermissionsRepository::new(&state);
	let permission = create_dummy_permission("Test Permission");
	let result = repo.query_create_permission(permission).await;
	assert!(result.is_ok(), "Create failed: {:?}", result.err());
}

#[tokio::test]
async fn test_query_permission_list_should_return_data() {
	let state = setup_all_test_environment().await; // Use the new setup function
	let repo = PermissionsRepository::new(&state);

	let _ = repo
		.query_create_permission(create_dummy_permission("View"))
		.await;
	let meta = crate::MetaRequestDto {
		page: Some(1),
		per_page: Some(10),
		search: None,
		sort_by: None,
		order: None,
		filter: None,
		filter_by: None,
	};

	let result = repo.query_permission_list(meta).await;
	assert!(result.is_ok(), "List failed: {:?}", result.err());
	assert!(!result.unwrap().data.is_empty(), "Data should not be empty");
}

#[tokio::test]
async fn test_query_permission_by_id_should_succeed() {
	let state = setup_all_test_environment().await; // Use the new setup function
	let repo = PermissionsRepository::new(&state);
	let permission = create_dummy_permission("Detail");
	let _ = repo.query_create_permission(permission.clone()).await;
	let id = permission.id.id.to_raw();
	let result = repo.query_permission_by_id(id).await;
	assert!(result.is_ok(), "Get by id failed: {:?}", result.err());
}

#[tokio::test]
async fn test_update_permission_should_succeed() {
	let state = setup_all_test_environment().await; // Use the new setup function
	let repo = PermissionsRepository::new(&state);
	let mut permission = create_dummy_permission("Update This");
	let _ = repo.query_create_permission(permission.clone()).await;
	permission.name = "Updated Name".into();
	permission.updated_at = Some(Utc::now().to_rfc3339());
	let result = repo.query_update_permission(permission).await;
	assert!(result.is_ok(), "Update failed: {:?}", result.err());
}

#[tokio::test]
async fn test_delete_permission_should_succeed() {
	let state = setup_all_test_environment().await; // Use the new setup function
	let repo = PermissionsRepository::new(&state);
	let permission = create_dummy_permission("To Be Deleted");
	let _ = repo.query_create_permission(permission.clone()).await;
	let id = permission.id.id.to_raw();
	let result = repo.query_delete_permission(id).await;
	assert!(result.is_ok(), "Delete failed: {:?}", result.err());
}

#[tokio::test]
async fn test_delete_permission_should_fail_if_already_deleted() {
	let state = setup_all_test_environment().await; // Use the new setup function
	let repo = PermissionsRepository::new(&state);
	let permission = create_dummy_permission("Delete Twice");
	let _ = repo.query_create_permission(permission.clone()).await;
	let id = permission.id.id.to_raw();
	let delete_result = repo.query_delete_permission(id.clone()).await;
	assert!(
		delete_result.is_ok(),
		"Initial delete failed: {:?}",
		delete_result.err()
	);
	let second_delete_result = repo.query_delete_permission(id).await;
	assert!(
		second_delete_result.is_err(),
		"Should fail on second delete"
	);
	if let Some(err) = second_delete_result.err() {
		assert!(
			err.to_string().contains("Permission not found"),
			"Expected 'Permission not found' error, got: {err}"
		);
	}
}

#[tokio::test]
async fn test_update_permission_should_fail_if_deleted() {
	let state = setup_all_test_environment().await; // Use the new setup function
	let repo = PermissionsRepository::new(&state);
	let mut permission = create_dummy_permission("To Be Updated Then Deleted");
	let _ = repo.query_create_permission(permission.clone()).await;
	let id = permission.id.id.to_raw();
	let delete_result = repo.query_delete_permission(id.clone()).await;
	assert!(
		delete_result.is_ok(),
		"Initial delete failed: {:?}",
		delete_result.err()
	);
	permission.name = "Try Update".into();
	let result = repo.query_update_permission(permission).await;
	assert!(result.is_err(), "Update on deleted should fail");
	if let Some(err) = result.err() {
		assert!(
			err.to_string().contains("Permission not found"),
			"Expected 'Permission not found' error, got: {err}"
		);
	}
}

#[tokio::test]
async fn test_query_permission_by_id_should_fail_if_not_found() {
	let state = setup_all_test_environment().await; // Use the new setup function
	let repo = PermissionsRepository::new(&state);
	let result = repo.query_permission_by_id("non-existent-id".into()).await;
	assert!(result.is_err(), "Expected error for not found id");
	if let Some(err) = result.err() {
		assert!(
			err.to_string().contains("Permission not found"),
			"Expected 'Permission not found' error, got: {err}"
		);
	}
}
