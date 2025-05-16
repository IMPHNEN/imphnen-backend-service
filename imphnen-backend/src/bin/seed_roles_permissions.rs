use imphnen_utils::{get_iso_date, make_thing, Env};
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

	let role_permissions = vec![
		(
			"50133429-f4b1-4249-9f97-7b86e6ee9d86",
			vec![
				"7c15e31d-36e2-49f9-97db-138c03fb0cf6",
				"319ee593-ff0a-4f29-bbaf-9feb3174a3a6",
			],
		),
		(
			"f6b03f25-e416-4893-ac88-caaa690afb07",
			vec![
				"023e2dfe-93c3-4008-94a8-b5dff403f73b",
				"96df0689-2ae9-4894-bf00-837c19415e5c",
				"98b3dc4c-0124-461f-afcd-166637c5e6e8",
				"319ee593-ff0a-4f29-bbaf-9feb3174a3a6",
				"7c15e31d-36e2-49f9-97db-138c03fb0cf6",
				"9164ca6e-c7e3-4238-a15f-f36ab9577e7e",
				"319ee593-ff0a-4f29-bbaf-9feb3174a3a2",
				"a00d5608-4c48-4542-845c-dfe004687022",
				"35b0d992-65c8-4b62-b030-e6e0320e4048",
			],
		),
	];
	for (role_id, permission_ids) in role_permissions {
		let permission_refs: Vec<_> = permission_ids
			.into_iter()
			.map(|perm_id| make_thing("app_permissions", perm_id))
			.collect();

		db.query("UPDATE type::thing('app_roles', $role_id) SET permissions = $permissions, updated_at = $updated_at")
    .bind(("role_id", role_id))
    .bind(("permissions", permission_refs))
    .bind(("updated_at", get_iso_date()))
    .await?;
		println!("✅ Updated permissions for role ID: {}", role_id);
	}
	println!("✅ All Roles permissions seeded");

	Ok(())
}
