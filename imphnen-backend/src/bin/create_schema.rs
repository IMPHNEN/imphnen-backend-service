#![allow(clippy::all)]

use imphnen_entities::seaorm::{auth, common, gacha};
use imphnen_libs::postgres::PostgresConfig;
use sea_orm::sea_query::Table;
use sea_orm::{ConnectionTrait, Database, DbBackend, EntityTrait, Schema};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("🛠️  Creating database schema...");

	let config = PostgresConfig::from_env()?;
	let db = Database::connect(&config.database_url).await?;
	let builder = db.get_database_backend();

	println!("   Database connected. Creating/updating tables...");

	drop_and_create_table(&db, builder, "app_roles", auth::roles::Entity).await?;
	drop_and_create_table(&db, builder, "app_permissions", auth::permissions::Entity)
		.await?;
	drop_and_create_table(&db, builder, "app_users", auth::users::Entity).await?;
	drop_and_create_table(
		&db,
		builder,
		"app_roles_permissions",
		auth::roles_permissions::Entity,
	)
	.await?;
	drop_and_create_table(&db, builder, "app_mentors", auth::mentors::Entity).await?;
	drop_and_create_table(&db, builder, "app_sessions", auth::sessions::Entity)
		.await?;

	drop_and_create_table(&db, builder, "events", common::events::Entity).await?;
	drop_and_create_table(&db, builder, "testimonials", common::testimonials::Entity)
		.await?;
	drop_and_create_table(&db, builder, "audit_logs", common::audit_log::Entity)
		.await?;
	drop_and_create_table(&db, builder, "rate_limits", common::rate_limit::Entity)
		.await?;

	drop_and_create_table(&db, builder, "gacha_credits", gacha::gacha_credits::Entity)
		.await?;
	drop_and_create_table(&db, builder, "gacha_items", gacha::gacha_items::Entity)
		.await?;
	drop_and_create_table(&db, builder, "gacha_rolls", gacha::gacha_rolls::Entity)
		.await?;
	drop_and_create_table(&db, builder, "gacha_claims", gacha::gacha_claims::Entity)
		.await?;

	println!("✅ Schema creation completed.");
	Ok(())
}

async fn drop_and_create_table<E>(
	db: &sea_orm::DatabaseConnection,
	builder: DbBackend,
	name: &str,
	entity: E,
) -> Result<(), Box<dyn std::error::Error>>
where
	E: EntityTrait,
{
	let schema = Schema::new(builder);

	let drop_stmt = Table::drop().table(entity).if_exists().cascade().to_owned();
	db.execute(builder.build(&drop_stmt)).await?;
	println!("   Dropped table if exists: {}", name);

	let mut create_stmt = schema.create_table_from_entity(entity);
	create_stmt.if_not_exists();

	db.execute(builder.build(&create_stmt)).await?;
	println!("   ✅ Created table: {}", name);

	Ok(())
}
