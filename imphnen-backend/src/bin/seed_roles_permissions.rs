use imphnen_iam::{get_iso_date, make_thing, Env};
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

	let admin_permissions = vec![
		"023e2dfe-93c3-4008-94a8-b5dff403f73b",
		"0269ed71-0ae0-4c43-ad29-e3d861d8f9a0",
		"299cb4d5-6556-4cc9-b6c1-32e6d31e0f9b",
		"319ee593-ff0a-4f29-bbaf-9feb3174a3a2",
		"319ee593-ff0a-4f29-bbaf-9feb3174a3a6",
		"35b0d992-65c8-4b62-b030-e6e0320e4048",
		"4da8b434-89f9-4d91-85ae-eebd63cdbeda",
		"73888d18-b3e9-4f62-95a5-ba2c0d69fccb",
		"7c15e31d-36e2-49f9-97db-138c03fb0cf6",
		"7d4b1379-4960-416a-b045-98cd82c0cac9",
		"8195eeb8-e64f-4172-aa57-596492c84a72",
		"81eba91d-b8ab-44b9-bbfe-4e6da2f98952",
		"9164ca6e-c7e3-4238-a15f-f36ab9577e7e",
		"96df0689-2ae9-4894-bf00-837c19415e5c",
		"98b3dc4c-0124-461f-afcd-166637c5e6e8",
		"a00d5608-4c48-4542-845c-dfe004687022",
		"b2dc3928-86ba-4c59-a03d-0b57d5183ebc",
		"dad435cf-042c-41bd-a946-cea61ed2ffbc",
	];

	let admin_role_id = "f6b03f25-e416-4893-ac88-caaa690afb07";

	let permission_refs_admin: Vec<_> = admin_permissions
		.into_iter()
		.map(|perm_id| make_thing("app_permissions", perm_id))
		.collect();

	db.query("UPDATE type::thing('app_roles', $role_id) SET permissions = $permissions, updated_at = $updated_at WHERE is_deleted = false")
		.bind(("role_id", admin_role_id))
		.bind(("permissions", permission_refs_admin))
		.bind(("updated_at", get_iso_date()))
		.await?;

	println!("✅ All permissions successfully added to each role");

	Ok(())
}
