use imphnen_cms::v1::landing::events::events_schema::EventsSchema;
use imphnen_utils::{get_iso_date};
use std::error::Error;
use surrealdb::engine::any;
use surrealdb::{opt::auth::Root, sql::Thing, Uuid}; // Added Uuid

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let env = &imphnen_libs::enviroment::ENV;
	let db = any::connect(&env.surrealdb_url).await?;
	db.signin(Root {
		username: &env.surrealdb_username,
		password: &env.surrealdb_password,
	})
	.await?;
	db.use_ns(env.surrealdb_namespace.clone())
		.use_db(env.surrealdb_dbname.clone())
		.await?;

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
	];

	for (
		name,
		description,
		detail_link,
		price,
		location,
		is_online,
		start_date,
		end_date,
	) in events
	{
        let uuid = Uuid::new_v4().to_string(); // Generate new UUID
		let event = EventsSchema {
			id: Thing::from(("app_events", uuid.as_str())), // Use generated UUID
			name: name.into(),
			description: description.into(),
			detail_link: detail_link.into(),
			price,
			location,
			is_online,
			is_deleted: false,
			start_date: start_date.into(),
			end_date: end_date.into(),
			created_at: get_iso_date(),
			updated_at: get_iso_date(),
		};

		db.create::<Option<EventsSchema>>(("app_events", uuid.as_str())) // Use generated UUID
			.content(event)
			.await?;

		println!(
			"✅ Inserted event: {} ({})",
			name,
			if is_online { "Online" } else { "In-person" }
		);
	}

	println!("✅ All Events seeded");
	Ok(())
}
