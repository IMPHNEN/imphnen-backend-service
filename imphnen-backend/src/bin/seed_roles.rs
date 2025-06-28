use imphnen_utils::{get_iso_date, Env};
use serde_json::json;
use std::error::Error;
use surrealdb::opt::auth::Root;
use imphnen_libs::enviroment::load_env;
use surrealdb::engine::any;
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
			None,
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
			None,
			Some("2025-02-22T15:38:39.868306+00"),
		),
	];

	for (id, name, _created_at, _updated_at) in roles {
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
		println!("✅ Inserted role: {}", name);
	}
	println!("✅ All Roles seeded");
	Ok(())
}
