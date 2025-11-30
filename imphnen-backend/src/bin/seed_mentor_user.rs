#![allow(clippy::all)]

use imphnen_libs::hash_password;
use serde_json::json;
use std::error::Error;
use imphnen_libs::postgres::{PostgresConfig, PostgresConnection};
use imphnen_entities::seaorm::auth::users::ActiveModel as UsersActiveModel;
use imphnen_entities::seaorm::auth::mentors::ActiveModel as MentorsActiveModel;
use imphnen_entities::seaorm::auth::roles::{Entity as RoleEntity, Column as RoleColumn};
use sea_orm::{ActiveModelTrait, ConnectionTrait, ActiveValue::Set, EntityTrait, QueryFilter, ColumnTrait};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let config = PostgresConfig::from_env()?;
	let pg_conn = PostgresConnection::new(config).await?;
	let db = &pg_conn.conn;

	let _ = pg_conn.execute(sea_orm::Statement::from_string(db.get_database_backend(), "DELETE FROM app_mentors WHERE id = 'e6f78d23-83bf-5c2b-bcd4-001345678901'".to_string())).await.ok();
	let _ = pg_conn.execute(sea_orm::Statement::from_string(db.get_database_backend(), "DELETE FROM app_users WHERE email = 'mentor@example.com'".to_string())).await.ok();

	// Find Mentor role
	let role = RoleEntity::find()
		.filter(RoleColumn::Name.eq("Mentor"))
		.one(db)
		.await?
		.ok_or("Role 'Mentor' not found")?;

	// Insert user with Mentor role
	let user_id = Uuid::new_v4();
	let mut user_model: UsersActiveModel = Default::default();
	user_model.id = Set(user_id);
	user_model.email = Set("mentor@example.com".to_string());
	user_model.password_hash = Set(hash_password("password").unwrap());
	user_model.username = Set("mentor@example.com".to_string());
	user_model.first_name = Set(Some("Mentor".to_string()));
	user_model.last_name = Set(Some("User".to_string()));
	user_model.avatar_url = Set(Some("https://example.com/avatar.jpg".to_string()));
	user_model.is_active = Set(true);
	user_model.is_verified = Set(true);
	user_model.role_id = Set(Some(role.id));
	user_model.created_at = Set(chrono::Utc::now());
	user_model.updated_at = Set(chrono::Utc::now());
	user_model.insert(db).await?;

	// Insert mentor
	let mentor_id = Uuid::new_v4();
	let mut mentor_model: MentorsActiveModel = Default::default();
	mentor_model.id = Set(mentor_id);
	mentor_model.user_id = Set(user_id);
	mentor_model.industries = Set(Some(json!( ["Software", "Education"] )));
	mentor_model.expertise = Set(Some(json!( ["Rust", "Microservices"] )));
	mentor_model.languages = Set(Some(json!( ["Indonesian", "English"] )));
	mentor_model.current_company = Set(Some("PT Contoh".to_string()));
	mentor_model.current_role = Set(Some("Senior Backend Engineer".to_string()));
	mentor_model.years_of_experience = Set(Some(5));
	mentor_model.topics_of_interest = Set(Some(json!( ["Rust Programming", "Backend Development"] )));
	mentor_model.preferred_mentee_level = Set(Some("beginner".to_string()));
	mentor_model.preferred_mentoring_formats = Set(Some(json!( ["online", "offline"] )));
	mentor_model.availability_commitment = Set(Some("2 jam per minggu untuk mentoring online dan offline".to_string()));
	mentor_model.mentoring_rate = Set(Some(100000.0));
	mentor_model.status = Set(Some("verified".to_string()));
	mentor_model.is_deleted = Set(false);
	mentor_model.created_at = Set(chrono::Utc::now());
	mentor_model.updated_at = Set(chrono::Utc::now());
	mentor_model.insert(db).await?;
	println!("Mentor created successfully!");

	println!("✅ Inserted mentor user: mentor@example.com");
	println!("✅ Mentor user seeded");
	Ok(())
}
