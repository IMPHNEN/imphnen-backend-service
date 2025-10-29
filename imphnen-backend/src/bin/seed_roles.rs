use imphnen_utils::{get_iso_date};
use serde_json::json;
use std::error::Error;
use surrealdb::engine::any;
use surrealdb::opt::auth::Root;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let env = &imphnen_libs::environment::ENV;
	let db = any::connect(&env.surrealdb_url).await?;
	db.signin(Root {
		username: &env.surrealdb_username,
		password: &env.surrealdb_password,
	})
	.await?;
	db.use_ns(env.surrealdb_namespace.clone())
		.use_db(env.surrealdb_dbname.clone())
		.await?;

	let roles = vec![
		(
			"50133429-f4b1-4249-9f97-7b86e6ee9d86",
			"Staf",
			Some("2025-02-24T16:52:27.630453+00"),
			Some("2025-02-24T16:52:27.630461+00"),
		),
		(
			"5713cb37-dc02-4e87-8048-d7a41d352059",
			"User",
			Some("2025-02-28T14:53:58.576688+00"),
			Some("2025-02-28T14:53:58.576688+00"),
		),
		(
			"60f1aeb7-dad2-4e06-bcb5-be1ba510c906",
			"Staff Aktivasi User",
			Some("2025-02-20T02:47:09.660640+00"),
			Some("2025-02-20T02:48:30.083283+00"),
		),
		(
			"6d4fea5d-4a08-4b8a-9782-f2ab2183dcf0",
			"Admin Pembayaran",
			Some("2025-01-29T05:39:28.562667+00"),
			Some("2025-03-12T22:56:29.597416+00"),
		),
		(
			"f6b03f25-e416-4893-ac88-caaa690afb07",
			"Admin",
			Some("2025-02-22T15:38:39.868306+00"),
			Some("2025-02-22T15:38:39.868306+00"),
		),
		(
			"3b9f8c4e-6a2d-4f8a-9a12-2d6f8b3c4e5a",
			"Mentor",
			Some("2025-07-06T10:00:00.000000+00"),
			Some("2025-07-06T10:00:00.000000+00"),
		),
	];

	for (id, name, _created_at, _updated_at) in roles {
		db.query("DELETE type::thing('app_roles', $id)")
			.bind(("id", id))
			.await?;
		db.query("CREATE type::thing('app_roles', $id) CONTENT $data")
			.bind(("id", id))
			.bind((
				"data",
				json!({
						"name": name,
						"permissions": [],
						"is_deleted": false,
						"created_at": get_iso_date(),
						"updated_at": get_iso_date(),
				}),
			))
			.await?;
		println!("✅ Inserted role: {name}");
	}
	println!("✅ All Roles seeded");
	Ok(())
}
