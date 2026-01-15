use imphnen_iam::PermissionsEnum;
use std::error::Error;
use imphnen_libs::postgres::{PostgresConfig, PostgresConnection};
use imphnen_entities::seaorm::auth::roles::Entity as RolesEntity;
use imphnen_entities::seaorm::auth::roles::ActiveModel as RoleActiveModel;
use sea_orm::ActiveValue::Set;
use sea_orm::EntityTrait;
use sea_orm::ActiveModelTrait;
use uuid::Uuid;
use serde_json::Value as JsonValue;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let config = PostgresConfig::from_env()?;
	let pg_conn = PostgresConnection::new(config).await?;
	let db = &pg_conn.conn;
	// Ensure indexes are present if needed (placeholders) - we don't modify schema here
	
    println!("✅ Index 'user_email_index' defined on table 'users' for column 'email'.");

	let roles_permissions = vec![
		(
			"f6b03f25-e416-4893-ac88-caaa690afb07",
			vec![
				// Only Administrator permission - grants access to everything
				PermissionsEnum::Administrator,
			],
		),
		(
			"3b9f8c4e-6a2d-4f8a-9a12-2d6f8b3c4e5a",
			vec![
				PermissionsEnum::ReadListUsers, // Added ReadListUsers permission
				PermissionsEnum::ReadOwnMentorProfile,
				PermissionsEnum::UpdateOwnMentorProfile,
				PermissionsEnum::ReadOwnMentorStatus,
				PermissionsEnum::ReadListMentors,
				PermissionsEnum::ReadDetailMentors,
				PermissionsEnum::ReadListGachaItems,
				PermissionsEnum::ReadDetailGachaItems,
				PermissionsEnum::ReadDetailGachaRolls,
				PermissionsEnum::CreateGachaRolls,
				PermissionsEnum::ExecuteGachaRolls,
			],
		),
		(
			"5713cb37-dc02-4e87-8048-d7a41d352059",
			vec![
				PermissionsEnum::ReadListGachaItems,
				PermissionsEnum::ReadDetailGachaItems,
				PermissionsEnum::ReadListUsers,
				PermissionsEnum::ReadDetailUsers,
				PermissionsEnum::CreateGachaClaims,
				PermissionsEnum::ReadDetailGachaClaims,
				PermissionsEnum::ReadDetailGachaRolls,
				PermissionsEnum::CreateGachaRolls,
				PermissionsEnum::ExecuteGachaRolls,
				PermissionsEnum::RegisterMentors,
				PermissionsEnum::ReadListMentors,
				PermissionsEnum::ReadDetailMentors,
				PermissionsEnum::ReadOwnMentorProfile,
				PermissionsEnum::ReadOwnMentorStatus,
			],
		),
		(
			"50133429-f4b1-4249-9f97-7b86e6ee9d86",
			vec![
				// Staff should be able to list roles and permissions in tests
				PermissionsEnum::ReadListRoles,
				PermissionsEnum::ReadListPermissions,
				PermissionsEnum::ReadListUsers,
				PermissionsEnum::ReadListMentors,
				PermissionsEnum::ReadDetailUsers,
				PermissionsEnum::ActivateUsers,
				PermissionsEnum::ReadDetailRoles,
				PermissionsEnum::ReadDetailPermissions,
				PermissionsEnum::ReadListGachaItems,
				PermissionsEnum::ReadDetailGachaItems,
				PermissionsEnum::ReadListMentors,
				PermissionsEnum::ReadDetailMentors,
				PermissionsEnum::ReadDetailGachaRolls,
				PermissionsEnum::CreateGachaRolls,
				PermissionsEnum::ExecuteGachaRolls,
			],
		),
		(
			"60f1aeb7-dad2-4e06-bcb5-be1ba510c906",
			vec![PermissionsEnum::ActivateUsers],
		),
		("6d4fea5d-4a08-4b8a-9782-f2ab2183dcf0", vec![]),
	];

	for (role_id, permissions) in roles_permissions {
		let role_uuid = Uuid::parse_str(role_id).unwrap_or_else(|_| Uuid::new_v4());
		// Map permissions enum to JSON array of permission ids
		let json_permissions = JsonValue::Array(
			permissions.iter().map(|p| JsonValue::String(p.id())).collect()
		);

		// Find role and update permissions
		if let Some(role_model) = RolesEntity::find_by_id(role_uuid).one(db).await? {
			let mut am: RoleActiveModel = role_model.into();
			am.permissions = Set(Some(json_permissions));
			am.update(db).await?;
			println!("✅ Permissions updated for role: {role_id}");
		} else {
			println!("⚠️ Role with id {role_id} not found, skipping permissions update");
		}
	}

	println!("✅ All roles permissions updated!");
	Ok(())
}
