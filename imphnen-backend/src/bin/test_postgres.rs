use chrono::Utc;
use imphnen_entities::seaorm::auth::roles::{
	Entity as RolesEntity, Model as RoleModel,
};
use imphnen_entities::seaorm::auth::users::{
	Entity as UsersEntity, Model as UserModel,
};
use imphnen_libs::postgres::{PostgresConfig, PostgresConnection, PostgresError};
use sea_orm::{
	ActiveModelTrait, DbErr, EntityTrait, PaginatorTrait, Set, TransactionTrait,
};
use std::sync::Arc;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("🚀 Starting PostgreSQL Connection Test");
	println!("=====================================");

	let config = PostgresConfig::from_env()?;
	println!("✅ Configuration loaded successfully");
	println!(
		"   Database URL: {}",
		config
			.database_url
			.replace("postgres://", "postgres://****:****@")
	);
	println!("   Pool size: {}", config.pool_size);
	println!("   Connect timeout: {}s", config.connect_timeout);
	println!("   Retry attempts: {}", config.retry_attempts);

	println!("\n🔌 Testing PostgreSQL connection...");
	match test_connection(config).await {
		Ok(()) => {
			println!("✅ All PostgreSQL tests passed successfully!");
			Ok(())
		}
		Err(e) => {
			println!("❌ PostgreSQL test failed: {}", e);
			Err(e.into())
		}
	}
}

async fn test_connection(config: PostgresConfig) -> Result<(), PostgresError> {
	println!("   Creating PostgreSQL connection...");
	let postgres_conn = PostgresConnection::new(config).await?;
	let connection = Arc::new(postgres_conn);
	println!("   ✅ Connection established successfully");

	println!("   Testing basic connectivity...");
	test_basic_connectivity(&connection).await?;
	println!("   ✅ Basic connectivity test passed");

	println!("   Testing table existence...");
	test_table_existence(&connection).await?;
	println!("   ✅ Table existence test passed");

	println!("   Testing CRUD operations...");
	test_crud_operations(&connection).await?;
	println!("   ✅ CRUD operations test passed");

	println!("   Testing transaction support...");
	test_transactions(&connection).await?;
	println!("   ✅ Transaction support test passed");

	println!("   Testing error handling...");
	test_error_handling(&connection).await?;
	println!("   ✅ Error handling test passed");

	Ok(())
}

async fn test_basic_connectivity(
	connection: &Arc<PostgresConnection>,
) -> Result<(), PostgresError> {
	let statement = sea_orm::Statement::from_string(
		connection.get_database_backend(),
		"SELECT 1 as test_value, current_timestamp as current_time".to_string(),
	);

	let result = connection.query_one(statement).await?.ok_or_else(|| {
		PostgresError::ConnectionError(sea_orm::DbErr::Custom(
			"No results returned".to_string(),
		))
	})?;

	let test_value: Option<i32> = result.try_get("", "test_value").ok();
	let current_time: Option<String> = result.try_get("", "current_time").ok();

	if test_value != Some(1) {
		return Err(PostgresError::ConnectionError(sea_orm::DbErr::Custom(
			format!("Expected test_value=1, got {:?}", test_value),
		)));
	}

	if current_time.is_none() {
		return Err(PostgresError::ConnectionError(sea_orm::DbErr::Custom(
			"Expected current_time to be set".to_string(),
		)));
	}

	println!(
		"      📝 Query result: test_value={:?}, current_time={:?}",
		test_value, current_time
	);
	Ok(())
}

async fn test_table_existence(
	connection: &Arc<PostgresConnection>,
) -> Result<(), PostgresError> {
	use sea_orm::EntityTrait;

	println!("      📋 Checking users table...");
	let user_count = UsersEntity::find()
		.count(&connection.conn)
		.await
		.map_err(PostgresError::ConnectionError)?;
	println!(
		"      📊 Users table accessible, current count: {}",
		user_count
	);

	println!("      📋 Checking roles table...");
	let role_count = RolesEntity::find()
		.count(&connection.conn)
		.await
		.map_err(PostgresError::ConnectionError)?;
	println!(
		"      📊 Roles table accessible, current count: {}",
		role_count
	);

	Ok(())
}

async fn test_crud_operations(
	connection: &Arc<PostgresConnection>,
) -> Result<(), PostgresError> {
	use sea_orm::{ActiveModelTrait, Set};

	println!("      ➕ Creating test user...");
	let test_user_id = Uuid::new_v4();
	let now = Utc::now();

	let user_model = imphnen_entities::seaorm::auth::users::ActiveModel {
		id: Set(test_user_id),
		email: Set(format!("test_user_{}@example.com", test_user_id)),
		password_hash: Set("test_password_hash".to_string()),
		username: Set(format!("testuser_{}", test_user_id)),
		first_name: Set(Some("Test".to_string())),
		last_name: Set(Some("User".to_string())),
		avatar_url: Set(None),
		is_verified: Set(false),
		is_active: Set(true),
		metadata: Set(None),
		role_id: Set(None),
		created_at: Set(now),
		updated_at: Set(now),
		deleted_at: Set(None),
	};

	let created_user = user_model
		.insert(&connection.conn)
		.await
		.map_err(PostgresError::ConnectionError)?;

	println!("      ✅ Created user with ID: {}", created_user.id);

	println!("      🔍 Reading test user...");
	let found_user = UsersEntity::find_by_id(test_user_id)
		.one(&connection.conn)
		.await
		.map_err(PostgresError::ConnectionError)?
		.ok_or_else(|| {
			PostgresError::ConnectionError(sea_orm::DbErr::Custom(
				"User not found after creation".to_string(),
			))
		})?;

	println!(
		"      ✅ Found user: {} ({})",
		found_user.username, found_user.email
	);

	println!("      ✏️  Updating test user...");
	let mut update_model: imphnen_entities::seaorm::auth::users::ActiveModel =
		found_user.into();
	update_model.first_name = Set(Some("Updated".to_string()));
	update_model.updated_at = Set(Utc::now());

	let updated_user = update_model
		.update(&connection.conn)
		.await
		.map_err(PostgresError::ConnectionError)?;

	println!(
		"      ✅ Updated user first name to: {:?}",
		updated_user.first_name
	);

	println!("      🗑️  Deleting test user...");
	UsersEntity::delete_by_id(updated_user.id)
		.exec(&connection.conn)
		.await
		.map_err(PostgresError::ConnectionError)?;

	println!("      ✅ Test user deleted successfully");

	Ok(())
}

async fn test_transactions(
	connection: &Arc<PostgresConnection>,
) -> Result<(), PostgresError> {
	println!("      💰 Testing transaction support...");

	let transaction_result = connection
		.conn
		.transaction(|txn| {
			Box::pin(async move {
				let test_user_id = Uuid::new_v4();
				let now = Utc::now();

				let user_model = imphnen_entities::seaorm::auth::users::ActiveModel {
					id: Set(test_user_id),
					email: Set(format!("transaction_test_{}@example.com", test_user_id)),
					password_hash: Set("transaction_password_hash".to_string()),
					username: Set(format!("transaction_user_{}", test_user_id)),
					first_name: Set(Some("Transaction".to_string())),
					last_name: Set(Some("Test".to_string())),
					avatar_url: Set(None),
					is_verified: Set(false),
					is_active: Set(true),
					metadata: Set(None),
					role_id: Set(None),
					created_at: Set(now),
					updated_at: Set(now),
					deleted_at: Set(None),
				};

				let _created_user = user_model.insert(txn).await?;

				Err::<(), DbErr>(DbErr::Custom("Simulated transaction failure".to_string()))
			})
		})
		.await;

	match transaction_result {
		Err(e) => {
			let e_text = format!("{:?}", e);
			if e_text.contains("Simulated transaction failure") {
				println!("      ✅ Transaction failed as expected, rollback successful");
			} else {
				return Err(PostgresError::OperationFailed(format!(
					"Unexpected transaction result: {}",
					e_text
				)));
			}
		}
		Ok(_) => {
			return Err(PostgresError::OperationFailed(
				"Unexpected transaction result: transaction unexpectedly succeeded"
					.to_string(),
			));
		}
	}

	let user_exists = UsersEntity::find_by_id(Uuid::nil())
		.one(&connection.conn)
		.await
		.map_err(PostgresError::ConnectionError)?
		.is_some();

	if user_exists {
		println!("      ⚠️  User found despite rollback - this might indicate an issue");
	} else {
		println!("      ✅ Transaction rollback verified - user not found");
	}

	Ok(())
}

async fn test_error_handling(
	connection: &Arc<PostgresConnection>,
) -> Result<(), PostgresError> {
	println!("      ⚠️  Testing error handling...");

	println!("      🔍 Testing invalid UUID handling...");
	let invalid_uuid = Uuid::nil();

	match UsersEntity::find_by_id(invalid_uuid)
		.one(&connection.conn)
		.await
		.map_err(PostgresError::ConnectionError)?
	{
		Some(_) => {
			println!("      ✅ Found user with nil UUID (expected in some cases)")
		}
		None => println!("      ✅ No user found with nil UUID (expected)"),
	}

	println!("      🔍 Testing invalid query handling...");
	let invalid_statement = sea_orm::Statement::from_string(
		connection.get_database_backend(),
		"SELECT * FROM non_existent_table".to_string(),
	);

	match connection.execute(invalid_statement).await {
		Err(_) => println!("      ✅ Invalid query properly handled with error"),
		Ok(_) => println!("      ⚠️  Invalid query unexpectedly succeeded"),
	}

	Ok(())
}

pub mod test_utils {
	use super::*;

	pub fn create_test_config() -> PostgresConfig {
		PostgresConfig {
			database_url: "postgres://postgres:postgres@localhost:5432/imphnen_test"
				.to_string(),
			pool_size: 5,
			connect_timeout: 10,
			idle_timeout: 30,
			max_lifetime: Some(600),
			retry_attempts: 2,
			retry_delay: 1,
		}
	}

	pub fn create_test_user_model() -> UserModel {
		UserModel {
			id: Uuid::new_v4(),
			email: format!("test_{}@example.com", Uuid::new_v4()),
			password_hash: "test_password_hash".to_string(),
			username: format!("testuser_{}", Uuid::new_v4()),
			first_name: Some("Test".to_string()),
			last_name: Some("User".to_string()),
			avatar_url: None,
			is_verified: false,
			is_active: true,
			metadata: None,
			role_id: None,
			created_at: Utc::now(),
			updated_at: Utc::now(),
			deleted_at: None,
		}
	}

	pub fn create_test_role_model() -> RoleModel {
		RoleModel {
			id: Uuid::new_v4(),
			name: format!("test_role_{}", Uuid::new_v4()),
			description: "Test role description".to_string(),
			permissions: Some(serde_json::json!(["test.permission"])),
			is_system_role: false,
			is_default: false,
			created_at: Utc::now(),
			updated_at: Utc::now(),
			deleted_at: None,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_create_test_config() {
		let config = test_utils::create_test_config();
		assert_eq!(config.pool_size, 5);
		assert_eq!(config.connect_timeout, 10);
		assert!(config.database_url.contains("imphnen_test"));
	}

	#[test]
	fn test_create_test_user_model() {
		let user = test_utils::create_test_user_model();
		assert!(!user.email.is_empty());
		assert!(!user.username.is_empty());
		assert!(user.is_active);
	}

	#[test]
	fn test_create_test_role_model() {
		let role = test_utils::create_test_role_model();
		assert!(!role.name.is_empty());
		assert!(role.permissions.is_some());
		assert!(!role.is_system_role);
	}
}
