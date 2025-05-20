use imphnen_iam::PermissionsEnum;
use imphnen_utils::{get_iso_date, Env};
use serde_json::json;
use std::error::Error;
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let env = Env::new();
	let db = Surreal::new::<Ws>(env.surrealdb_url).await?;
	db.signin(Root {
		username: &env.surrealdb_username,
		password: &env.surrealdb_password,
	})
	.await?;
	db.use_ns(env.surrealdb_namespace)
		.use_db(env.surrealdb_dbname)
		.await?;

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
	] {
		db.query("CREATE type::thing('app_permissions', $id) CONTENT $data")
			.bind(("id", permission.id()))
			.bind((
				"data",
				json!({
					"name": permission.to_string(),
					"is_deleted": false,
					"created_at": get_iso_date(),
					"updated_at": get_iso_date()
				}),
			))
			.await?;
		println!("✅ Inserted: {}", permission.to_string());
	}

	println!("✅ All Permissions seeded");
	Ok(())
}
