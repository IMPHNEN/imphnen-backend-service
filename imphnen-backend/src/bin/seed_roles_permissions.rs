use imphnen_iam::{get_iso_date, make_thing, PermissionsEnum};
use std::error::Error;
use surrealdb::engine::any;
use surrealdb::opt::auth::Root;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let env = &imphnen_libs::enviroment::ENV;
	let db = any::connect(&env.surrealdb_url).await?;
	db.signin(Root {
		username: &env.surrealdb_username,
		password: &env.surrealdb_password,
	})
	.await?;
	db.use_ns(env.surrealdb_namespace.clone())
		.use_db(env.surrealdb_dbname.clone())
		.await?;
    db.query("DEFINE INDEX user_email_index ON TABLE users COLUMNS email UNIQUE;")
	
        .await?;
    db.query("DEFINE INDEX role_name_idx ON TABLE roles COLUMNS name UNIQUE;")
	
        .await?;
	
    println!("✅ Index 'user_email_index' defined on table 'users' for column 'email'.");

	let roles_permissions = vec![
		(
			"f6b03f25-e416-4893-ac88-caaa690afb07",
			vec![
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
				PermissionsEnum::UpdateMentors,
				PermissionsEnum::VerifyMentors,
				PermissionsEnum::DeleteMentors,
			],
		),
		(
			"3b9f8c4e-6a2d-4f8a-9a12-2d6f8b3c4e5a",
			vec![
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
				PermissionsEnum::ReadListUsers,
				PermissionsEnum::ReadListMentors,
				PermissionsEnum::ReadDetailUsers,
				PermissionsEnum::ActivateUsers,
				PermissionsEnum::ReadListRoles,
				PermissionsEnum::ReadDetailRoles,
				PermissionsEnum::ReadListPermissions,
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
		let permission_refs: Vec<_> = permissions
			.iter()
			.map(|perm| make_thing("app_permissions", perm.id()))
			.collect();

		db.query("UPDATE type::thing('app_roles', $role_id) SET permissions = $permissions, updated_at = $updated_at WHERE is_deleted = false")
            .bind(("role_id", role_id))
            .bind(("permissions", permission_refs))
            .bind(("updated_at", get_iso_date()))
            .await?;
		println!("✅ Permissions updated for role: {role_id}");
	}

	println!("✅ All roles permissions updated!");
	Ok(())
}
