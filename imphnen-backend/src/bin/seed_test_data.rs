#![allow(clippy::all)]

use chrono::Utc;
use imphnen_entities::seaorm::auth::mentors::ActiveModel as MentorsActiveModel;
use imphnen_entities::seaorm::common::events::ActiveModel as EventsActiveModel;
use imphnen_entities::seaorm::common::testimonials::ActiveModel as TestimonialsActiveModel;
use imphnen_libs::postgres::{PostgresConfig, PostgresConnection};
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use serde_json::json;
use std::error::Error;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let config = PostgresConfig::from_env()?;
	let pg_conn = PostgresConnection::new(config).await?;
	let db = &pg_conn.conn;

	let uuid = Uuid::new_v4().to_string();
	let mut event_model: EventsActiveModel = Default::default();
	event_model.id = Set(Uuid::parse_str(&uuid)?);
	event_model.name = Set("Test Event".to_string());
	event_model.description = Set("Test event description".to_string());
	event_model.detail_link = Set("https://example.com/event".to_string());
	event_model.price = Set(50.0);
	event_model.is_online = Set(true);
	event_model.start_date = Set(Utc::now());
	event_model.end_date = Set(Utc::now() + chrono::Duration::days(1));
	event_model.location = Set(None);
	event_model.is_deleted = Set(false);
	match event_model.insert(db).await {
		Ok(_) => println!("✅ Inserted test event"),
		Err(_) => {
			println!("⚠️  Test event already exists or could not be inserted, skipping")
		}
	};

	let mut testimonial_model: TestimonialsActiveModel = Default::default();
	testimonial_model.id =
		Set(Uuid::parse_str("00000000-0000-0000-0000-000000000001")?);
	testimonial_model.user_id =
		Set(Uuid::parse_str("c3b1d6a8-8d4f-4b36-b789-2e532ec7a7b2")?);
	testimonial_model.role = Set("Student".to_string());
	testimonial_model.content = Set("This is a great platform!".to_string());
	testimonial_model.is_deleted = Set(false);
	match testimonial_model.insert(db).await {
		Ok(_) => println!("✅ Inserted test testimonial"),
		Err(_) => println!(
			"⚠️  Test testimonial already exists or could not be inserted, skipping"
		),
	};

	let mentor_id = Uuid::new_v4();
	let mut mentor_model: MentorsActiveModel = Default::default();
	mentor_model.id = Set(mentor_id);
	mentor_model.user_id =
		Set(Uuid::parse_str("c3b1d6a8-8d4f-4b36-b789-2e532ec7a7b2")?);
	mentor_model.industries = Set(Some(json!(["Technology", "Education"])));
	mentor_model.expertise = Set(Some(json!(["Software Development"])));
	mentor_model.languages = Set(Some(json!(["English", "Indonesian"])));
	mentor_model.current_company = Set(Some("Tech Corp".to_string()));
	mentor_model.current_role = Set(Some("Senior Engineer".to_string()));
	mentor_model.years_of_experience = Set(Some(5));
	mentor_model.topics_of_interest = Set(Some(json!(["Rust", "Web Development"])));
	mentor_model.preferred_mentee_level = Set(Some("Beginner".to_string()));
	mentor_model.preferred_mentoring_formats = Set(Some(json!(["1:1", "Group"])));
	mentor_model.availability_commitment = Set(Some("Weekly".to_string()));
	mentor_model.mentoring_rate = Set(Some(100.0));
	mentor_model.status = Set(Some("active".to_string()));
	mentor_model.is_deleted = Set(false);
	mentor_model.created_at = Set(chrono::Utc::now());
	mentor_model.updated_at = Set(chrono::Utc::now());
	mentor_model.insert(db).await?;
	println!("✅ Inserted test mentor via SeaORM");

	println!("✅ All test data seeded successfully");
	Ok(())
}
