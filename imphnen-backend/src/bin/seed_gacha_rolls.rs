#![allow(clippy::all)]

use std::error::Error;
use imphnen_libs::postgres::{PostgresConfig, PostgresConnection};
use imphnen_entities::seaorm::gacha::gacha_items::ActiveModel as GachaItemActiveModel;
use imphnen_entities::seaorm::gacha::gacha_rolls::ActiveModel as GachaRollActiveModel;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use uuid::Uuid;
use sea_orm::ConnectionTrait;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let config = PostgresConfig::from_env()?;
	let pg_conn = PostgresConnection::new(config).await?;
	let db = &pg_conn.conn;

	// Check if gacha item already exists
	let check_item_sql = "SELECT id FROM app_gacha_items WHERE item_code = 'ITEM_TEST_1' LIMIT 1";
	let item_result = pg_conn.query_one(sea_orm::Statement::from_string(db.get_database_backend(), check_item_sql)).await?;
	let gacha_item_uuid = if let Some(ref row) = item_result {
		// Item exists, get its ID
		row.try_get("", "id")?
	} else {
		// Item doesn't exist, create it
		// Note: We can't easily delete by a fixed ID since it's a UUID, but the insert will fail if there's a conflict
		let _ = pg_conn.execute(sea_orm::Statement::from_string(db.get_database_backend(), "DELETE FROM app_gacha_items WHERE item_code = 'ITEM_TEST_1'".to_string())).await.ok();
		
		// Create gacha item via SeaORM
		let new_uuid = Uuid::new_v4();
		let mut item_model: GachaItemActiveModel = Default::default();
		item_model.id = Set(new_uuid);
		item_model.item_code = Set("ITEM_TEST_1".to_string());
		item_model.name = Set("Test Gacha Item".to_string());
		item_model.description = Set("Test item for gacha".to_string());
		item_model.rarity = Set("common".to_string());
		item_model.type_ = Set("item".to_string());
		item_model.category = Set("test".to_string());
		item_model.value = Set(1);
		item_model.weight = Set(1.0);
		item_model.stock = Set(10);
		item_model.is_limited = Set(false);
		item_model.created_at = Set(chrono::Utc::now());
		item_model.updated_at = Set(chrono::Utc::now());
		item_model.insert(db).await?;
		println!("Gacha Item seeded successfully!");
		new_uuid
	};

	// Always try to insert the roll, relying on the database constraints to prevent duplicates if needed
	let gacha_roll_id = Uuid::new_v4();
	let mut roll_model: GachaRollActiveModel = Default::default();
	roll_model.id = Set(gacha_roll_id);
	roll_model.user_id = Set(Uuid::parse_str("c3b1d6a8-8d4f-4b36-b789-2e532ec7a7b2")?);
	roll_model.gacha_id = Set(Uuid::new_v4().to_string());
	roll_model.item_id = Set(gacha_item_uuid);
	roll_model.weight = Set(1.0);
	roll_model.quantity = Set(10);
	roll_model.is_deleted = Set(false);
	roll_model.created_at = Set(Some(chrono::Utc::now().naive_utc()));
	roll_model.updated_at = Set(Some(chrono::Utc::now().naive_utc()));
	roll_model.insert(db).await?;
	println!("Gacha Roll seeded successfully!");
		let gacha_roll_id = Uuid::new_v4();
		let mut roll_model: GachaRollActiveModel = Default::default();
		roll_model.id = Set(gacha_roll_id);
		roll_model.user_id = Set(Uuid::parse_str("c3b1d6a8-8d4f-4b36-b789-2e532ec7a7b2")?);
		roll_model.gacha_id = Set(Uuid::new_v4().to_string());
		roll_model.item_id = Set(gacha_item_uuid);
		roll_model.weight = Set(1.0);
		roll_model.quantity = Set(10);
		roll_model.is_deleted = Set(false);
		roll_model.created_at = Set(Some(chrono::Utc::now().naive_utc()));
		roll_model.updated_at = Set(Some(chrono::Utc::now().naive_utc()));
		roll_model.insert(db).await?;
	println!("✅ Gacha items and rolls seeded.");
	Ok(())
}
