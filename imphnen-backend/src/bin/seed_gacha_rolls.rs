use imphnen_utils::{get_iso_date};
use std::error::Error;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let env = &imphnen_libs::environment::ENV;
	use surrealdb::engine::any;
	let db = any::connect(&env.surrealdb_url).await?;
	db.signin(Root {
		username: &env.surrealdb_username,
		password: &env.surrealdb_password,
	})
	.await?;
	db.use_ns(env.surrealdb_namespace.clone())
		.use_db(env.surrealdb_dbname.clone())
		.await?;

	db.query("DELETE type::thing('app_gacha_items', $id)")
		.bind(("id", "gacha_item_test_id"))
		.await?;
	db.query("DELETE type::thing('app_gacha_rolls', $id)")
		.bind(("id", "gacha_roll_test_id"))
		.await?;

	let gacha_item_id = "gacha_item_test_id";
	db.query("CREATE type::thing('app_gacha_items', $id) SET name = $name, image_url = $image_url, is_deleted = $is_deleted, created_at = $created_at, updated_at = $updated_at")
        .bind(("id", gacha_item_id))
        .bind(("name", "Test Gacha Item"))
        .bind(("image_url", "https://example.com/gacha_item.png"))
        .bind(("is_deleted", false))
        .bind(("created_at", get_iso_date()))
        .bind(("updated_at", get_iso_date()))
        .await?;
	println!("Gacha Item seeded successfully!");

	let gacha_roll_id = "gacha_roll_test_id";
	db.query("CREATE type::thing('app_gacha_rolls', $id) SET item = $item, quantity = $quantity, weight = $weight, is_deleted = $is_deleted, created_at = $created_at, updated_at = $updated_at")
        .bind(("id", gacha_roll_id))
        .bind(("item", Thing::from(("app_gacha_items", gacha_item_id))))
        .bind(("quantity", 10))
        .bind(("weight", 1.0))
        .bind(("is_deleted", false))
        .bind(("created_at", get_iso_date()))
        .bind(("updated_at", get_iso_date()))
        .await?;
	println!("Gacha Roll seeded successfully!");

	println!("✅ Gacha items and rolls seeded.");
	Ok(())
}
