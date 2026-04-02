#![allow(clippy::all)]

use chrono::Utc;
use imphnen_entities::seaorm::auth::permissions::ActiveModel as PermissionActiveModel;
use imphnen_entities::seaorm::auth::permissions::Entity as PermissionEntity;
use imphnen_iam::PermissionsEnum;
use imphnen_libs::postgres::{PostgresConfig, PostgresConnection};
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use std::error::Error;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let config = PostgresConfig::from_env()?;
	let pg_conn = PostgresConnection::new(config).await?;
	let db = &pg_conn.conn;

	for permission in [
		PermissionsEnum::ReadListUsers,
		PermissionsEnum::ReadDetailUsers,
		PermissionsEnum::CreateUsers,
		PermissionsEnum::DeleteUsers,
		PermissionsEnum::UpdateUsers,
		PermissionsEnum::ActivateUsers,
		PermissionsEnum::ReadListRoles,
		PermissionsEnum::ReadDetailRoles,
		PermissionsEnum::CreateRoles,
		PermissionsEnum::DeleteRoles,
		PermissionsEnum::UpdateRoles,
		PermissionsEnum::ReadListPermissions,
		PermissionsEnum::ReadDetailPermissions,
		PermissionsEnum::CreatePermissions,
		PermissionsEnum::DeletePermissions,
		PermissionsEnum::UpdatePermissions,
		PermissionsEnum::CreateGachaClaims,
		PermissionsEnum::ReadDetailGachaClaims,
		PermissionsEnum::ReadListGachaItems,
		PermissionsEnum::ReadDetailGachaItems,
		PermissionsEnum::CreateGachaItems,
		PermissionsEnum::DeleteGachaItems,
		PermissionsEnum::UpdateGachaItems,
		PermissionsEnum::ReadDetailGachaRolls,
		PermissionsEnum::CreateGachaRolls,
		PermissionsEnum::ExecuteGachaRolls,
		PermissionsEnum::ReadListMentors,
		PermissionsEnum::ReadDetailMentors,
		PermissionsEnum::RegisterMentors,
		PermissionsEnum::ReadOwnMentorProfile,
		PermissionsEnum::UpdateOwnMentorProfile,
		PermissionsEnum::ReadOwnMentorStatus,
		PermissionsEnum::UpdateMentors,
		PermissionsEnum::VerifyMentors,
		PermissionsEnum::DeleteMentors,
		PermissionsEnum::Administrator,
	] {
		let parsed_id =
			Uuid::parse_str(&permission.id()).unwrap_or_else(|_| Uuid::new_v4());

		let existing = PermissionEntity::find_by_id(parsed_id).one(db).await?;
		if existing.is_some() {
			println!("ℹ️  Skipping (already exists): {permission}");
			continue;
		}

		let mut perm_model: PermissionActiveModel = Default::default();
		perm_model.id = Set(parsed_id);
		perm_model.name = Set(permission.to_string());
		perm_model.is_deleted = Set(false);
		perm_model.created_at = Set(Utc::now());
		perm_model.updated_at = Set(Utc::now());
		perm_model.insert(db).await?;
		println!("✅ Inserted: {permission}");
	}

	println!("✅ All Permissions seeded");

	Ok(())
}
