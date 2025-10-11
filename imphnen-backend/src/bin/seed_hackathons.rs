use chrono::{DateTime, Utc};
use imphnen_hackathon::v1::hackathon::hackathon_schema::{
    HackathonSchema, HackathonEventsSchema, HackathonTimelineSchema, HackathonSubmissionsSchema,
    HackathonStatus, HackathonEventType, HackathonPhase, SubmissionStatus, Prize
};
use imphnen_utils::get_iso_date;
use std::error::Error;
use surrealdb::{opt::auth::Root, sql::Thing};

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

    // Sample hackathon data
    let hackathons = vec![
        (
            "hackathon-001",
            "AI Innovation Challenge 2025",
            "Build the next generation of AI-powered applications that solve real-world problems.",
            "2025-10-15T09:00:00Z",
            "2025-10-17T18:00:00Z",
            "2025-10-01T23:59:59Z",
            Some(100),
            HackathonStatus::RegistrationOpen,
            Some("Artificial Intelligence & Machine Learning".to_string()),
            Some("1. All code must be original\n2. Teams can have 2-5 members\n3. Projects must use AI/ML technologies".to_string()),
            Some(vec![
                Prize { position: 1, title: "Grand Prize".to_string(), description: Some("Winner gets full scholarship".to_string()), value: Some("$10,000".to_string()) },
                Prize { position: 2, title: "Second Place".to_string(), description: Some("Runner-up prize".to_string()), value: Some("$5,000".to_string()) },
                Prize { position: 3, title: "Third Place".to_string(), description: Some("Third place prize".to_string()), value: Some("$2,500".to_string()) },
            ]),
            vec!["c3b1d6a8-8d4f-4b36-b789-2e532ec7a7b2".to_string()], // admin user
        ),
        (
            "hackathon-002",
            "Green Tech Hackathon",
            "Develop sustainable technology solutions for environmental challenges.",
            "2025-11-20T10:00:00Z",
            "2025-11-22T17:00:00Z",
            "2025-11-05T23:59:59Z",
            Some(75),
            HackathonStatus::Draft,
            Some("Sustainability & Green Technology".to_string()),
            Some("Focus on renewable energy, waste reduction, and environmental monitoring.".to_string()),
            Some(vec![
                Prize { position: 1, title: "Eco Champion".to_string(), description: Some("Best environmental impact".to_string()), value: Some("$7,500".to_string()) },
                Prize { position: 2, title: "Innovation Award".to_string(), description: Some("Most innovative solution".to_string()), value: Some("$3,500".to_string()) },
            ]),
            vec!["c3b1d6a8-8d4f-4b36-b789-2e532ec7a7b2".to_string()],
        ),
    ];

    // Sample hackathon events
    let hackathon_events = vec![
        (
            "hackathon-001",
            "event-001",
            "Opening Ceremony",
            Some("Welcome and kickoff event for the AI Innovation Challenge".to_string()),
            HackathonEventType::Ceremony,
            "2025-10-15T09:00:00Z",
            "2025-10-15T10:00:00Z",
            Some("Main Auditorium".to_string()),
            None,
            Some(150),
            true,
        ),
        (
            "hackathon-001",
            "event-002",
            "AI Workshop: Getting Started",
            Some("Introduction to AI frameworks and tools".to_string()),
            HackathonEventType::Workshop,
            "2025-10-15T14:00:00Z",
            "2025-10-15T16:00:00Z",
            None,
            Some("https://zoom.us/meeting/ai-workshop".to_string()),
            Some(80),
            false,
        ),
        (
            "hackathon-001",
            "event-003",
            "Judging Session",
            Some("Final project presentations and judging".to_string()),
            HackathonEventType::Judging,
            "2025-10-17T14:00:00Z",
            "2025-10-17T17:00:00Z",
            Some("Innovation Lab".to_string()),
            None,
            Some(100),
            true,
        ),
    ];

    // Sample hackathon timeline
    let hackathon_timeline = vec![
        (
            "hackathon-001",
            HackathonPhase::Registration,
            "Registration Phase",
            Some("Register your team and submit initial project ideas".to_string()),
            "2025-10-01T00:00:00Z",
            "2025-10-10T23:59:59Z",
            true,
            1,
        ),
        (
            "hackathon-001",
            HackathonPhase::Ideation,
            "Ideation & Planning",
            Some("Brainstorm and plan your AI solution".to_string()),
            "2025-10-11T00:00:00Z",
            "2025-10-14T23:59:59Z",
            false,
            2,
        ),
        (
            "hackathon-001",
            HackathonPhase::Development,
            "Development Sprint",
            Some("Build your AI-powered application".to_string()),
            "2025-10-15T00:00:00Z",
            "2025-10-16T23:59:59Z",
            false,
            3,
        ),
        (
            "hackathon-001",
            HackathonPhase::Submission,
            "Project Submission",
            Some("Submit your final project and demo video".to_string()),
            "2025-10-17T00:00:00Z",
            "2025-10-17T12:00:00Z",
            false,
            4,
        ),
        (
            "hackathon-001",
            HackathonPhase::Judging,
            "Judging & Awards",
            Some("Presentations and prize ceremony".to_string()),
            "2025-10-17T13:00:00Z",
            "2025-10-17T18:00:00Z",
            false,
            5,
        ),
    ];

    // Sample hackathon submissions
    let hackathon_submissions = vec![
        (
            "hackathon-001",
            "team-dev-001",
            "AI-Powered Health Monitor",
            "A machine learning application that predicts health risks using wearable device data.",
            Some("https://github.com/team-dev/ai-health-monitor".to_string()),
            Some("https://demo.ai-health-monitor.com".to_string()),
            None,
            vec!["Python".to_string(), "TensorFlow".to_string(), "React".to_string()],
            SubmissionStatus::Submitted,
            "2025-10-17T11:30:00Z",
        ),
        (
            "hackathon-001",
            "team-design-001",
            "Smart City Traffic Optimizer",
            "AI system that optimizes traffic flow using computer vision and predictive analytics.",
            Some("https://github.com/team-design/smart-traffic".to_string()),
            Some("https://demo.smart-traffic.com".to_string()),
            Some("https://slides.smart-traffic.com/presentation".to_string()),
            vec!["JavaScript".to_string(), "Node.js".to_string(), "OpenCV".to_string()],
            SubmissionStatus::UnderReview,
            "2025-10-17T10:45:00Z",
        ),
    ];

    // Seed hackathons
    for (
        id,
        name,
        description,
        start_date,
        end_date,
        registration_deadline,
        max_participants,
        status,
        theme,
        rules,
        prizes,
        organizers,
    ) in hackathons {
        db.query("DELETE type::thing('app_hackathons', $id)")
            .bind(("id", id))
            .await?;

        let hackathon = HackathonSchema {
            id: Thing::from(("app_hackathons", id)),
            name: name.into(),
            description: description.into(),
            start_date: DateTime::parse_from_rfc3339(start_date)?.with_timezone(&Utc),
            end_date: DateTime::parse_from_rfc3339(end_date)?.with_timezone(&Utc),
            registration_deadline: DateTime::parse_from_rfc3339(registration_deadline)?.with_timezone(&Utc),
            max_participants,
            status,
            theme,
            rules,
            prizes,
            previous_winners: None,
            organizers,
            is_deleted: false,
            created_at: Some(get_iso_date()),
            updated_at: Some(get_iso_date()),
        };

        db.create::<Option<HackathonSchema>>(("app_hackathons", id))
            .content(hackathon)
            .await?;

        println!("✅ Inserted hackathon: {name}");
    }

    // Seed hackathon events
    for (
        hackathon_id,
        event_id,
        title,
        description,
        event_type,
        start_time,
        end_time,
        location,
        virtual_link,
        max_attendees,
        is_mandatory,
    ) in hackathon_events {
        db.query("DELETE type::thing('app_hackathon_events', $id)")
            .bind(("id", event_id))
            .await?;

        let event = HackathonEventsSchema {
            id: Thing::from(("app_hackathon_events", event_id)),
            hackathon_id: Thing::from(("app_hackathons", hackathon_id)),
            title: title.into(),
            description,
            event_type,
            start_time: DateTime::parse_from_rfc3339(start_time)?.with_timezone(&Utc),
            end_time: DateTime::parse_from_rfc3339(end_time)?.with_timezone(&Utc),
            location,
            virtual_link,
            max_attendees,
            is_mandatory,
            is_deleted: false,
            created_at: Some(get_iso_date()),
            updated_at: Some(get_iso_date()),
        };

        db.create::<Option<HackathonEventsSchema>>(("app_hackathon_events", event_id))
            .content(event)
            .await?;

        println!("✅ Inserted hackathon event: {title}");
    }

    // Seed hackathon timeline
    for (
        hackathon_id,
        phase,
        title,
        description,
        start_date,
        end_date,
        is_active,
        order,
    ) in hackathon_timeline {
        let timeline_id = format!("timeline-{}-{}", hackathon_id, order);

        db.query("DELETE type::thing('app_hackathon_timeline', $id)")
            .bind(("id", timeline_id.clone()))
            .await?;

        let timeline = HackathonTimelineSchema {
            id: Thing::from(("app_hackathon_timeline", timeline_id.as_str())),
            hackathon_id: Thing::from(("app_hackathons", hackathon_id)),
            phase,
            title: title.into(),
            description,
            start_date: DateTime::parse_from_rfc3339(start_date)?.with_timezone(&Utc),
            end_date: DateTime::parse_from_rfc3339(end_date)?.with_timezone(&Utc),
            is_active,
            order,
            is_deleted: false,
            created_at: Some(get_iso_date()),
            updated_at: Some(get_iso_date()),
        };

        db.create::<Option<HackathonTimelineSchema>>(("app_hackathon_timeline", timeline_id))
            .content(timeline)
            .await?;

        println!("✅ Inserted hackathon timeline: {title}");
    }

    // Seed hackathon submissions
    for (
        hackathon_id,
        team_id,
        project_name,
        description,
        repository_url,
        demo_url,
        slides_url,
        technologies,
        submission_status,
        submitted_at,
    ) in hackathon_submissions {
        let submission_id = format!("submission-{}-{}", hackathon_id, team_id);

        db.query("DELETE type::thing('app_hackathon_submissions', $id)")
            .bind(("id", submission_id.clone()))
            .await?;

        let submission = HackathonSubmissionsSchema {
            id: Thing::from(("app_hackathon_submissions", submission_id.as_str())),
            hackathon_id: Thing::from(("app_hackathons", hackathon_id)),
            judge_feedback: None,
            team_id: Some(Thing::from(("app_teams", team_id))),
            project_name: Some(project_name.into()),
            description: Some(description.into()),
            repository_url,
            demo_url,
            slides_url,
            technologies: Some(technologies),
            submission_status: Some(submission_status),
            submitted_at: Some(DateTime::parse_from_rfc3339(submitted_at)?.with_timezone(&Utc)),
            is_deleted: false,
            created_at: Some(get_iso_date()),
            updated_at: Some(get_iso_date()),
        };

        db.create::<Option<HackathonSubmissionsSchema>>(("app_hackathon_submissions", submission_id))
            .content(submission)
            .await?;

        println!("✅ Inserted hackathon submission: {project_name}");
    }

    println!("✅ All Hackathons seeded");
    Ok(())
}