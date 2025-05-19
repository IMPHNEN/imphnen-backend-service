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
	let permissions = vec![
		(
			"023e2dfe-93c3-4008-94a8-b5dff403f73b",
			"Create Users",
			Some("2025-01-29T06:08:23.838311+00"),
			Some("2025-01-29T06:08:23.838312+00"),
		),
		(
			"0269ed71-0ae0-4c43-ad29-e3d861d8f9a0",
			"Create Permissions",
			Some("2025-01-29T05:11:01.265+00"),
			Some("2025-01-29T05:11:01.265001+00"),
		),
		(
			"299cb4d5-6556-4cc9-b6c1-32e6d31e0f9b",
			"Update Permissions",
			Some("2025-01-29T05:11:01.265+00"),
			Some("2025-01-29T05:11:01.265001+00"),
		),
		(
			"319ee593-ff0a-4f29-bbaf-9feb3174a3a2",
			"Create Roles",
			Some("2025-01-29T05:11:01.265+00"),
			Some("2025-01-29T05:11:01.265001+00"),
		),
		(
			"319ee593-ff0a-4f29-bbaf-9feb3174a3a6",
			"Read Detail Users",
			Some("2025-01-29T05:11:01.265+00"),
			Some("2025-01-29T05:11:01.265001+00"),
		),
		(
			"35b0d992-65c8-4b62-b030-e6e0320e4048",
			"Delete Roles",
			Some("2025-01-29T05:34:40.621554+00"),
			Some("2025-01-29T05:34:40.621555+00"),
		),
		(
			"4da8b434-89f9-4d91-85ae-eebd63cdbeda",
			"Update Activate Users",
			Some("2025-02-01T12:38:09.741726+00"),
			Some("2025-02-01T12:38:09.741727+00"),
		),
		(
			"73888d18-b3e9-4f62-95a5-ba2c0d69fccb",
			"Read Detail Roles",
			Some("2025-01-29T05:13:06.445925+00"),
			Some("2025-01-29T10:31:46.408564+00"),
		),
		(
			"7c15e31d-36e2-49f9-97db-138c03fb0cf6",
			"Read List Users",
			Some("2025-01-28T15:02:41.772931+00"),
			Some("2025-01-28T15:02:41.772933+00"),
		),
		(
			"7d4b1379-4960-416a-b045-98cd82c0cac9",
			"Read Detail Sessions",
			Some("2025-02-24T16:52:26.886664+00"),
			Some("2025-02-24T16:52:26.886673+00"),
		),
		(
			"8195eeb8-e64f-4172-aa57-596492c84a72",
			"Read List Permissions",
			Some("2025-01-28T15:05:28.6299+00"),
			Some("2025-01-28T15:05:28.629901+00"),
		),
		(
			"81eba91d-b8ab-44b9-bbfe-4e6da2f98952",
			"Read List Tests",
			Some("2025-02-24T16:52:27.179542+00"),
			Some("2025-02-24T16:52:27.179551+00"),
		),
		(
			"9164ca6e-c7e3-4238-a15f-f36ab9577e7e",
			"Read List Roles",
			Some("2025-01-29T05:34:40.621554+00"),
			Some("2025-01-29T05:34:40.621555+00"),
		),
		(
			"96df0689-2ae9-4894-bf00-837c19415e5c",
			"Delete Users",
			Some("2025-02-02T06:52:05.195565+00"),
			Some("2025-02-02T06:52:05.195565+00"),
		),
		(
			"98b3dc4c-0124-461f-afcd-166637c5e6e8",
			"Update Users",
			Some("2025-01-29T05:34:40.621554+00"),
			Some("2025-01-29T05:34:40.621555+00"),
		),
		(
			"a00d5608-4c48-4542-845c-dfe004687022",
			"Update Roles",
			Some("2025-01-29T05:34:40.621554+00"),
			Some("2025-01-29T05:34:40.621555+00"),
		),
		(
			"b2dc3928-86ba-4c59-a03d-0b57d5183ebc",
			"Delete Permissions",
			Some("2025-01-29T05:14:22.511084+00"),
			Some("2025-01-29T05:14:22.511085+00"),
		),
		(
			"dad435cf-042c-41bd-a946-cea61ed2ffbc",
			"Read Detail Permissions",
			Some("2025-01-28T15:07:10.990214+00"),
			Some("2025-01-28T15:07:10.990214+00"),
		),
	];
	for (id, name, _created_at, _updated_at) in permissions {
		db.query("CREATE type::thing('app_permissions', $id) CONTENT $data")
			.bind(("id", id))
			.bind((
				"data",
				json!({
						"name": name,
						"is_deleted": false,
						"created_at": get_iso_date(),
						"updated_at": get_iso_date()
				}),
			))
			.await?;
		println!("✅ Inserted: {}", name);
	}
	println!("✅ All Permissions seeded");
	Ok(())
}
