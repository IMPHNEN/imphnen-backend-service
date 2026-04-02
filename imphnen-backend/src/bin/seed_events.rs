#![allow(clippy::all)]

use chrono::Utc;
use imphnen_entities::seaorm::common::events::{
	ActiveModel as EventsActiveModel, Entity as EventEntity,
};
use imphnen_libs::postgres::{PostgresConfig, PostgresConnection};
use sea_orm::{
	ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter,
};
use std::error::Error;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let postgres_config = PostgresConfig::from_env()?;
	let pg_conn = PostgresConnection::new(postgres_config).await?;
	let db = &pg_conn.conn;

	let events = vec![
		(
			"Tech Conference 2025",
			"Annual technology conference featuring the latest innovations in software development, AI, and cloud computing.",
			"https://techconf2025.example.com",
			150.0,
			Some("Jakarta Convention Center".to_string()),
			false,
			"2025-06-15T09:00:00Z",
			"2025-06-17T18:00:00Z",
		),
		(
			"Online Web Development Workshop",
			"Comprehensive workshop covering modern web development frameworks including React, Vue, and Angular.",
			"https://webdev-workshop.example.com",
			75.0,
			None,
			true,
			"2025-07-10T14:00:00Z",
			"2025-07-10T17:00:00Z",
		),
		(
			"Startup Pitch Competition",
			"Exciting competition where emerging startups present their innovative ideas to a panel of expert judges and investors.",
			"https://startup-pitch.example.com",
			25.0,
			Some("Innovation Hub Surabaya".to_string()),
			false,
			"2025-08-05T10:00:00Z",
			"2025-08-05T16:00:00Z",
		),
		(
			"Digital Marketing Masterclass",
			"Learn advanced digital marketing strategies, social media optimization, and data-driven marketing techniques.",
			"https://digital-marketing.example.com",
			100.0,
			None,
			true,
			"2025-09-20T13:00:00Z",
			"2025-09-22T15:00:00Z",
		),
        (
            "Rust Programming Bootcamp",
            "Intensive 3-day bootcamp to master Rust fundamentals and advanced concepts.",
            "https://rust-bootcamp.example.com",
            200.0,
            Some("Bandung Digital Valley".to_string()),
            false,
            "2025-10-01T09:00:00Z",
            "2025-10-03T17:00:00Z",
        ),
        (
            "AI & Machine Learning Summit",
            "Global summit discussing the future of AI and its impact on industries.",
            "https://ai-summit.example.com",
            300.0,
            Some("Bali Nusa Dua Convention Center".to_string()),
            false,
            "2025-11-15T08:00:00Z",
            "2025-11-17T18:00:00Z",
        ),
        (
            "Cybersecurity Awareness Webinar",
            "Free webinar on best practices for personal and corporate cybersecurity.",
            "https://cybersecurity-webinar.example.com",
            0.0,
            None,
            true,
            "2025-12-05T14:00:00Z",
            "2025-12-05T16:00:00Z",
        ),
        (
            "Cloud Computing Workshop",
            "Hands-on workshop on deploying scalable applications using AWS and Azure.",
            "https://cloud-workshop.example.com",
            120.0,
            None,
            true,
            "2026-01-20T10:00:00Z",
            "2026-01-22T15:00:00Z",
        ),
        (
            "Blockchain for Finance",
            "Exploring the applications of blockchain technology in the financial sector.",
            "https://blockchain-finance.example.com",
            180.0,
            Some("Jakarta Ritz-Carlton".to_string()),
            false,
            "2026-02-10T09:00:00Z",
            "2026-02-11T17:00:00Z",
        ),
        (
            "Game Development Jam",
            "48-hour game development marathon for indie developers.",
            "https://game-jam.example.com",
            50.0,
            Some("Yogyakarta Creative Hub".to_string()),
            false,
            "2026-03-15T18:00:00Z",
            "2026-03-17T18:00:00Z",
        ),
        (
            "UX/UI Design Principles",
            "Masterclass on creating intuitive and user-friendly interfaces.",
            "https://uxui-design.example.com",
            90.0,
            None,
            true,
            "2026-04-05T13:00:00Z",
            "2026-04-07T16:00:00Z",
        ),
        (
            "Data Science Fundamentals",
            "Introduction to data analysis, visualization, and statistical modeling.",
            "https://data-science.example.com",
            110.0,
            None,
            true,
            "2026-05-12T10:00:00Z",
            "2026-05-14T15:00:00Z",
        ),
        (
            "IoT Innovation Expo",
            "Showcase of the latest Internet of Things devices and solutions.",
            "https://iot-expo.example.com",
            50.0,
            Some("Surabaya Expo Center".to_string()),
            false,
            "2026-06-20T09:00:00Z",
            "2026-06-22T18:00:00Z",
        ),
	];

	for (
		name,
		description,
		detail_link,
		price,
		location,
		is_online,
		start_date_str,
		end_date_str,
	) in events
	{
		let existing = EventEntity::find()
			.filter(<EventEntity as EntityTrait>::Column::Name.eq(name))
			.one(db)
			.await?;
		if existing.is_some() {
			println!("ℹ️  Skipping (already exists): {name}");
			continue;
		}

		let uuid = Uuid::new_v4();
		let mut event_model: EventsActiveModel = Default::default();
		event_model.id = Set(uuid);
		event_model.name = Set(name.to_string());
		event_model.description = Set(description.to_string());
		event_model.detail_link = Set(detail_link.to_string());
		event_model.price = Set(price);
		event_model.is_online = Set(is_online);
		event_model.location = Set(location.clone());
		event_model.start_date = Set(
			chrono::DateTime::parse_from_rfc3339(start_date_str)?
				.with_timezone(&chrono::Utc),
		);
		event_model.end_date = Set(
			chrono::DateTime::parse_from_rfc3339(end_date_str)?
				.with_timezone(&chrono::Utc),
		);
		event_model.is_deleted = Set(false);
		event_model.created_at = Set(Utc::now());
		event_model.updated_at = Set(Utc::now());

		event_model.insert(db).await?;

		println!(
			"✅ Inserted event: {} ({})",
			name,
			if is_online { "Online" } else { "In-person" }
		);
	}

	println!("✅ All Events seeded");
	Ok(())
}
