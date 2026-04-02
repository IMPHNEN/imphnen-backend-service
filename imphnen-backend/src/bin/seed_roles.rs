use chrono::Utc;
use imphnen_entities::seaorm::auth::roles::{Entity as RoleEntity, RoleBuilder};
use imphnen_libs::postgres::{PostgresConfig, PostgresConnection};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait};
use std::error::Error;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let config = PostgresConfig::from_env()?;
	let pg_conn = PostgresConnection::new(config).await?;
	let db = &pg_conn.conn;

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

	for (id, name, _created_at_str, _updated_at_str) in roles {
		let uuid = Uuid::parse_str(id).unwrap_or_else(|_| Uuid::new_v4());

		let existing = RoleEntity::find_by_id(uuid).one(db).await?;
		if existing.is_some() {
			println!("ℹ️  Skipping (already exists): {name}");
			continue;
		}

		let role_model = RoleBuilder::new()
			.name(name.to_string())
			.description("System generated role".to_string())
			.permissions(vec![])
			.is_default(false)
			.build()?;
		let mut role_model = role_model;
		role_model.id = Set(uuid);
		role_model.is_system_role = Set(true);
		role_model.created_at = Set(Utc::now());
		role_model.updated_at = Set(Utc::now());

		role_model.insert(db).await?;
		println!("✅ Inserted role: {name}");
	}
	println!("✅ All Roles seeded");
	Ok(())
}
