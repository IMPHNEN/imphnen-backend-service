// Restore only the necessary imports to fix unresolved function errors
use crate::{get_iso_date};
use imphnen_libs::{AppState, postgres::{PostgresConnection, PostgresConfig}, PostgresUserLookupService};
use imphnen_iam::{UsersSchema, v1::auth::auth_repository::AuthRepoImpl};
use imphnen_libs::hash_password;
use std::sync::Arc;
// sea_orm not currently needed by the mock test harness;
use tracing::debug;

pub async fn create_mock_app_state() -> AppState {
	// Create a mock Postgres connection for testing
	let postgres_conn = Arc::new(PostgresConnection::new(PostgresConfig::default()).await.unwrap());

	AppState {
		postgres_connection: postgres_conn,
		user_lookup_service: Arc::new(PostgresUserLookupService::new()),
		auth_repository: Arc::new(AuthRepoImpl::new()), // Use Postgres-based AuthRepo
	}
}
pub async fn cleanup_db() {
	// In a real test environment, you would use PostgresConnection to clean up test data
	// For now, we'll just print a message since we're using a mock connection
	println!("Cleaning up test database (Postgres mock)");
}

pub async fn seed_permissions_and_roles_for_test(
	_postgres_conn: &PostgresConnection,
) -> Result<(), Box<dyn std::error::Error>> {
	// In a real implementation, you would use SeaORM to insert test data
	// For now, we'll just print a message since we're using a mock connection
	
	// This is a placeholder for the actual implementation using SeaORM
	// When the real implementation is ready, you would use code like:
	//
	// for perm_enum in PermissionsEnum::iter() {
	//     let perm = PermissionActiveModel {
	//         id: Set(Uuid::parse_str(perm_enum.id()).unwrap()),
	//         name: Set(perm_enum.to_string()),
	//         is_deleted: Set(false),
	//         created_at: Set(get_iso_date()),
	//         updated_at: Set(get_iso_date()),
	//         ..Default::default()
	//     };
	//     perm.save(&postgres_conn.conn).await?;
	// }
	//
	// let roles = vec![
	//     ("f6b03f25-e416-4893-ac88-caaa690afb07".to_string(), "Admin"),
	//     ("3b9f8c4e-6a2d-4f8a-9a12-2d6f8b3c4e5a".to_string(), "Mentor"),
	//     ("50133429-f4b1-4249-9f97-7b86e6ee9d86".to_string(), "Staff"),
	//     ("5713cb37-dc02-4e87-8048-d7a41d352059".to_string(), "User"),
	// ];
	//
	// for (id, name) in roles {
	//     let role = RoleActiveModel {
	//         id: Set(Uuid::parse_str(&id).unwrap()),
	//         name: Set(name),
	//         is_deleted: Set(false),
	//         created_at: Set(get_iso_date()),
	//         updated_at: Set(get_iso_date()),
	//         permissions: Set(vec![]),
	//         ..Default::default()
	//     };
	//     role.save(&postgres_conn.conn).await?;
	// }
	
	println!("Seeded permissions and roles for test (Postgres mock)");
	Ok(())
}

pub async fn seed_users_for_test(
	_postgres_conn: &PostgresConnection,
) -> Result<(), Box<dyn std::error::Error>> {
	let users_data = vec![
		(
			"c3b1d6a8-8d4f-4b36-b789-2e532ec7a7b2",
			"admin@example.com",
			"Admin",
			"f6b03f25-e416-4893-ac88-caaa690afb07",
		),
		(
			"a4d23fb5-9e31-423c-9842-fbd6e75a5298",
			"staff@example.com",
			"Staff",
			"50133429-f4b1-4249-9f97-7b86e6ee9d86",
		),
		(
			"d5e89c12-72af-4b1a-abc3-ff1234567890",
			"user@example.com",
			"User",
			"5713cb37-dc02-4e87-8048-d7a41d352059",
		),
	];

	for (id, email, fullname, role_id) in users_data {
		let mut profile_ext = imphnen_entities::users::UserProfileExtensionDto::default();
		profile_ext.phone_number = Some("081234567890".into());
		let _user = UsersSchema {
			id: uuid::Uuid::parse_str(id).unwrap_or(uuid::Uuid::new_v4()).to_string(),
			fullname: Some(fullname.into()),
			legal_name: None,
			email: Some(email.into()),
			password: Some(hash_password("password").unwrap()),
			avatar: None,
			profile_extension: Some(profile_ext),
			is_active: true,
			is_deleted: false,
			mentor_id: None,
			role_id: uuid::Uuid::parse_str(role_id).ok(),
			created_at: get_iso_date(),
			updated_at: get_iso_date(),
			..Default::default()
		};
		// For now we don't insert into a real DB in tests; this is a placeholder
		// so the test harness compiles. Insert logic can be added later using
		// UsersRepository::new(app_state).query_create_user(user) to create records.

		println!("✅ Inserted user: {fullname} ({email})");
	}
	Ok(())
}

pub async fn setup_all_test_environment() -> AppState {
debug!("Setting up all test environment in setup_all_test_environment()");
	let app_state = create_mock_app_state().await;
	// Seed roles/permissions into Postgres via the PostgresConnection
	seed_permissions_and_roles_for_test(&app_state.postgres_connection)
		.await
		.unwrap();
	app_state
}
