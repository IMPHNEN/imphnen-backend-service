use imphnen_cms::v1::landing::events::events_schema::EventsSchema;
use imphnen_cms::v1::landing::testimonials::testimonials_schema::TestimonialsSchema;
use imphnen_dimentorin::v1::mentors::mentors_schema::MentorSchema;
use imphnen_dimentorin::v1::mentors::mentors_dto::MentoringRate;
use imphnen_hackathon::v1::hackathon::hackathon_schema::{
    HackathonSchema, HackathonEventsSchema, HackathonTimelineSchema,
    HackathonStatus, HackathonEventType, HackathonPhase
};
use imphnen_utils::get_iso_date;
use std::error::Error;
use surrealdb::{opt::auth::Root, sql::Thing};
use chrono::Utc;

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

    // Seed Events - handle existing data
    let event = EventsSchema {
        id: Thing::from(("app_events", "1")),
        name: "Test Event".to_string(),
        description: "Test event description".to_string(),
        detail_link: "https://example.com/event".to_string(),
        price: 50.0,
        is_online: true,
        start_date: get_iso_date(),
        end_date: get_iso_date(),
        location: None,
        is_deleted: false,
        created_at: get_iso_date(),
        updated_at: get_iso_date(),
    };
    match db.create::<Option<EventsSchema>>(("app_events", "1"))
        .content(event)
        .await {
        Ok(_) => println!("✅ Inserted test event"),
        Err(_) => println!("⚠️  Test event already exists, skipping"),
    };

    // Seed Testimonials - handle existing data
    let testimonial = TestimonialsSchema {
        id: Thing::from(("app_testimonials", "1")),
        user: Thing::from(("app_users", "c3b1d6a8-8d4f-4b36-b789-2e532ec7a7b2")),
        role: "Student".to_string(),
        content: "This is a great platform!".to_string(),
        is_deleted: false,
        created_at: get_iso_date(),
        updated_at: get_iso_date(),
    };
    match db.create::<Option<TestimonialsSchema>>(("app_testimonials", "1"))
        .content(testimonial)
        .await {
        Ok(_) => println!("✅ Inserted test testimonial"),
        Err(_) => println!("⚠️  Test testimonial already exists, skipping"),
    };

    // Seed Hackathon - handle existing data
    let hackathon = HackathonSchema {
        id: Thing::from(("app_hackathons", "1")),
        name: "Test Hackathon".to_string(),
        description: "Test hackathon description".to_string(),
        start_date: Utc::now() + chrono::Duration::days(30),
        end_date: Utc::now() + chrono::Duration::days(37),
        registration_deadline: Utc::now() + chrono::Duration::days(25),
        max_participants: Some(100),
        status: HackathonStatus::Draft,
        theme: Some("Technology".to_string()),
        rules: Some("Follow the rules".to_string()),
        prizes: Some(vec![]),
        previous_winners: Some(vec![]),
        organizers: vec!["c3b1d6a8-8d4f-4b36-b789-2e532ec7a7b2".to_string()],
        is_deleted: false,
        created_at: Some(get_iso_date()),
        updated_at: Some(get_iso_date()),
    };
    match db.create::<Option<HackathonSchema>>(("app_hackathons", "1"))
        .content(hackathon)
        .await {
        Ok(_) => println!("✅ Inserted test hackathon"),
        Err(_) => println!("⚠️  Test hackathon already exists, skipping"),
    };

    // Seed Hackathon Event
    let hackathon_event = HackathonEventsSchema {
        id: Thing::from(("app_hackathon_events", "test-event-001")),
        hackathon_id: Thing::from(("app_hackathons", "1")),
        title: "Test Event".to_string(),
        description: Some("Test hackathon event description".to_string()),
        event_type: HackathonEventType::Workshop,
        start_time: Utc::now() + chrono::Duration::days(30),
        end_time: Utc::now() + chrono::Duration::days(30) + chrono::Duration::hours(6),
        location: Some("Online".to_string()),
        virtual_link: None,
        max_attendees: Some(50),
        is_mandatory: false,
        is_deleted: false,
        created_at: Some(get_iso_date()),
        updated_at: Some(get_iso_date()),
    };
    // Try to create hackathon event, skip if already exists
    match db.create::<Option<HackathonEventsSchema>>(("app_hackathon_events", "test-event-001"))
        .content(hackathon_event)
        .await {
        Ok(_) => println!("✅ Inserted test hackathon event"),
        Err(_) => println!("⚠️  Test hackathon event already exists, skipping"),
    };

    // Seed Hackathon Timeline
    let hackathon_timeline = HackathonTimelineSchema {
        id: Thing::from(("app_hackathon_timeline", "test-timeline-001")),
        hackathon_id: Thing::from(("app_hackathons", "1")),
        phase: HackathonPhase::Registration,
        title: "Test Timeline".to_string(),
        description: Some("Test hackathon timeline description".to_string()),
        start_date: Utc::now(),
        end_date: Utc::now() + chrono::Duration::days(7),
        is_active: true,
        order: 1,
        is_deleted: false,
        created_at: Some(get_iso_date()),
        updated_at: Some(get_iso_date()),
    };
    
    // Try to create hackathon timeline, skip if already exists
    match db.create::<Option<HackathonTimelineSchema>>(("app_hackathon_timeline", "test-timeline-001"))
        .content(hackathon_timeline)
        .await {
        Ok(_) => println!("✅ Inserted test hackathon timeline"),
        Err(_) => println!("⚠️  Test hackathon timeline already exists, skipping"),
    };

    // Seed Mentor - handle existing data
    let mentor = MentorSchema {
        id: Thing::from(("app_mentors", "e6f78d23-83bf-5c2b-bcd4-001345678901")),
        user_id: Some(Thing::from(("app_users", "e6f78d23-83bf-5c2b-bcd4-001345678901"))),
        industries: vec!["Technology".to_string(), "Education".to_string()],
        expertise: vec!["Software Development".to_string()],
        languages: vec!["English".to_string(), "Indonesian".to_string()],
        current_company: "Tech Corp".to_string(),
        current_role: "Senior Engineer".to_string(),
        years_of_experience: 5,
        topics_of_interest: vec!["Rust".to_string(), "Web Development".to_string()],
        preferred_mentee_level: vec!["Beginner".to_string()],
        preferred_mentoring_formats: vec!["1:1".to_string(), "Group".to_string()],
        availability_commitment: "Weekly".to_string(),
        mentoring_rate: MentoringRate {
            amount: 100,
            currency: "IDR".to_string(),
            per_duration: "hour".to_string(),
        },
        status: "active".to_string(),
        is_deleted: false,
        created_at: get_iso_date(),
        updated_at: get_iso_date(),
    };
    match db.create::<Option<MentorSchema>>(("app_mentors", "e6f78d23-83bf-5c2b-bcd4-001345678901"))
        .content(mentor)
        .await {
        Ok(_) => println!("✅ Inserted test mentor"),
        Err(_) => println!("⚠️  Test mentor already exists, skipping"),
    };

    println!("✅ All test data seeded successfully");
    Ok(())
}