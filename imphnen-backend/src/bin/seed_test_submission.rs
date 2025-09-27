use chrono::{DateTime, Utc};
use imphnen_hackathon::v1::hackathon::hackathon_schema::{
    HackathonSchema, HackathonTimelineSchema, HackathonSubmissionsSchema,
    HackathonStatus, HackathonPhase, SubmissionStatus, Prize
};
use imphnen_iam::{UsersSchema, v1::teams::TeamsSchema};
use imphnen_utils::{get_iso_date, hash_password};
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

    // Test users for submission testing
    let test_users = vec![
        (
            "test-user-001",
            "testuser1@example.com",
            "Test User 1",
            "5713cb37-dc02-4e87-8048-d7a41d352059", // User role
        ),
        (
            "test-user-002",
            "testuser2@example.com",
            "Test User 2",
            "5713cb37-dc02-4e87-8048-d7a41d352059",
        ),
        (
            "test-user-003",
            "testuser3@example.com",
            "Test User 3",
            "5713cb37-dc02-4e87-8048-d7a41d352059",
        ),
    ];

    // Test teams for submission testing
    let test_teams = vec![
        (
            "test-team-001",
            "Test Team Alpha",
            Some("Team for testing hackathon submissions".to_string()),
            "test-user-001", // team leader
            true,
            Some(5),
            Some(vec!["JavaScript".to_string(), "React".to_string()]),
            Some("Remote".to_string()),
        ),
        (
            "test-team-002",
            "Test Team Beta",
            Some("Another team for testing submissions".to_string()),
            "test-user-002",
            true,
            Some(4),
            Some(vec!["Python".to_string(), "Django".to_string()]),
            Some("Remote".to_string()),
        ),
    ];

    // Test hackathon for submission testing
    let test_hackathons = vec![
        (
            "test-hackathon-001",
            "Test Hackathon 2025",
            "Hackathon for testing submission functionality.",
            "2025-12-01T09:00:00Z",
            "2025-12-03T18:00:00Z",
            "2025-11-25T23:59:59Z",
            Some(50),
            HackathonStatus::RegistrationOpen,
            Some("Testing & Development".to_string()),
            Some("1. Test all submission features\n2. Teams can have 2-5 members\n3. Submit by deadline".to_string()),
            Some(vec![
                Prize { position: 1, title: "Test Winner".to_string(), description: Some("Best test submission".to_string()), value: Some("$1,000".to_string()) },
                Prize { position: 2, title: "Test Runner-up".to_string(), description: Some("Second best submission".to_string()), value: Some("$500".to_string()) },
            ]),
            vec!["c3b1d6a8-8d4f-4b36-b789-2e532ec7a7b2".to_string()], // admin user
        ),
    ];

    // Test hackathon timeline
    let test_timeline = vec![
        (
            "test-hackathon-001",
            HackathonPhase::Registration,
            "Registration Phase",
            Some("Register your team for the test hackathon".to_string()),
            "2025-11-20T00:00:00Z",
            "2025-11-25T23:59:59Z",
            true,
            1,
        ),
        (
            "test-hackathon-001",
            HackathonPhase::Development,
            "Development Phase",
            Some("Build your test project".to_string()),
            "2025-12-01T00:00:00Z",
            "2025-12-02T23:59:59Z",
            false,
            2,
        ),
        (
            "test-hackathon-001",
            HackathonPhase::Submission,
            "Submission Phase",
            Some("Submit your test project".to_string()),
            "2025-12-03T00:00:00Z",
            "2025-12-03T12:00:00Z",
            false,
            3,
        ),
    ];

    // Test submissions
    let test_submissions = vec![
        (
            "test-hackathon-001",
            "test-team-001",
            "Test Project Alpha",
            "A comprehensive test project demonstrating all features.",
            Some("https://github.com/test-team-alpha/test-project".to_string()),
            Some("https://demo.test-project-alpha.com".to_string()),
            Some("https://slides.test-project-alpha.com".to_string()),
            vec!["JavaScript".to_string(), "React".to_string(), "Node.js".to_string()],
            SubmissionStatus::Draft,
            "2025-12-02T10:00:00Z",
        ),
        (
            "test-hackathon-001",
            "test-team-002",
            "Test Project Beta",
            "Another test project with different technologies.",
            Some("https://github.com/test-team-beta/test-project".to_string()),
            Some("https://demo.test-project-beta.com".to_string()),
            None,
            vec!["Python".to_string(), "Django".to_string(), "PostgreSQL".to_string()],
            SubmissionStatus::Submitted,
            "2025-12-03T09:30:00Z",
        ),
    ];

    // Seed test users
    for (id, email, fullname, role_id) in test_users {
        db.query("DELETE type::thing('app_users', $id)")
            .bind(("id", id))
            .await?;

        let user = UsersSchema {
            id: Thing::from(("app_users", id)),
            fullname: fullname.into(),
            legal_name: Some(format!("{} Legal Name", fullname)),
            email: email.into(),
            password: hash_password("password").unwrap(),
            avatar: Some("https://example.com/avatar.jpg".into()),
            phone_number: "081234567890".into(),
            phone_for_verification: Some("081234567890".into()),
            is_active: true,
            is_deleted: false,
            mentor_id: None,
            gender: Some("male".into()),
            birthdate: Some("1995-01-01".into()),
            domicile: Some("Jakarta, Indonesia".into()),
            bio: Some(format!("{} is a test user for hackathon submissions.", fullname)),
            last_education: Some("S1 Computer Science".into()),
            linkedin_url: Some("https://linkedin.com/in/testuser".into()),
            github_url: Some("https://github.com/testuser".into()),
            cv_url: Some("https://example.com/cv.pdf".into()),
            portfolio_url: Some("https://example.com/portfolio".into()),
            website_url: Some("https://example.com/website".into()),
            twitter_url: Some("https://twitter.com/testuser".into()),
            location: Some("Jakarta, Indonesia".into()),
            skills: Some(vec!["JavaScript".to_string(), "Python".to_string()]),
            experience: None,
            education: None,
            career_status: Some("Developer".into()),
            role: Thing::from(("app_roles", role_id)),
            created_at: get_iso_date(),
            updated_at: get_iso_date(),
        };

        db.create::<Option<UsersSchema>>(("app_users", id))
            .content(user)
            .await?;

        println!("✅ Inserted test user: {fullname} ({email})");
    }

    // Seed test teams
    for (id, name, description, leader_id, is_open, max_members, skills_required, location) in test_teams {
        db.query("DELETE type::thing('app_teams', $id)")
            .bind(("id", id))
            .await?;

        let team = TeamsSchema {
            id: Thing::from(("app_teams", id)),
            name: name.into(),
            description,
            leader_id: Thing::from(("app_users", leader_id)),
            is_open,
            max_members,
            skills_required,
            location,
            avatar: None,
            website_url: None,
            github_url: None,
            is_active: true,
            is_deleted: false,
            created_at: get_iso_date(),
            updated_at: get_iso_date(),
        };

        db.create::<Option<TeamsSchema>>(("app_teams", id))
            .content(team)
            .await?;

        println!("✅ Inserted test team: {name}");
    }

    // Seed test hackathons
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
    ) in test_hackathons {
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

        println!("✅ Inserted test hackathon: {name}");
    }

    // Seed test hackathon timeline
    for (
        hackathon_id,
        phase,
        title,
        description,
        start_date,
        end_date,
        is_active,
        order,
    ) in test_timeline {
        let timeline_id = format!("test-timeline-{}-{}", hackathon_id, order);

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

        println!("✅ Inserted test hackathon timeline: {title}");
    }

    // Seed test hackathon submissions
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
    ) in test_submissions {
        let submission_id = format!("test-submission-{}-{}", hackathon_id, team_id);

        db.query("DELETE type::thing('app_hackathon_submissions', $id)")
            .bind(("id", submission_id.clone()))
            .await?;

        let submission = HackathonSubmissionsSchema {
            id: Thing::from(("app_hackathon_submissions", submission_id.as_str())),
            hackathon_id: Thing::from(("app_hackathons", hackathon_id)),
            team_id: Thing::from(("app_teams", team_id)),
            project_name: project_name.into(),
            description: description.into(),
            repository_url,
            demo_url,
            slides_url,
            technologies,
            submission_status,
            submitted_at: DateTime::parse_from_rfc3339(submitted_at)?.with_timezone(&Utc),
            is_deleted: false,
            created_at: Some(get_iso_date()),
            updated_at: Some(get_iso_date()),
        };

        db.create::<Option<HackathonSubmissionsSchema>>(("app_hackathon_submissions", submission_id))
            .content(submission)
            .await?;

        println!("✅ Inserted test hackathon submission: {project_name}");
    }

    println!("✅ All test submission data seeded");
    Ok(())
}