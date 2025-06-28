use imphnen_iam::{get_iso_date, make_thing, Env, PermissionsEnum};
use std::error::Error;
use surrealdb::opt::auth::Root;
use surrealdb::engine::any;
use imphnen_libs::enviroment::load_env;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	load_env();
	let env = Env::new();
	let db = any::connect(&env.surrealdb_url).await?;
	db.signin(Root {
		username: &env.surrealdb_username,
		password: &env.surrealdb_password,
	})
	.await?;
	db.use_ns(env.surrealdb_namespace)
		.use_db(env.surrealdb_dbname)
		.await?;
	let permission_refs_admin: Vec<_> = [
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
	]
	.iter()
	.map(|perm| make_thing("app_permissions", perm.id()))
	.collect();
	let admin_role_id = "f6b03f25-e416-4893-ac88-caaa690afb07";
	db.query("UPDATE type::thing('app_roles', $role_id) SET permissions = $permissions, updated_at = $updated_at WHERE is_deleted = false")
		.bind(("role_id", admin_role_id))
		.bind(("permissions", permission_refs_admin))
		.bind(("updated_at", get_iso_date()))
		.await?;
	println!("✅ All permissions successfully added to Admin role");
	Ok(())
}
