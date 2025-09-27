use super::hackathon_dto::{
    HackathonCreateRequestDto, HackathonEventCreateRequestDto,
    HackathonEventUpdateRequestDto, HackathonSubmissionCreateRequestDto,
    HackathonSubmissionUpdateRequestDto, HackathonTimelineCreateRequestDto,
    HackathonTimelineUpdateRequestDto, HackathonUpdateRequestDto,
};
use super::hackathon_schema::{
    HackathonEventsSchema, HackathonSchema, HackathonSubmissionsSchema, HackathonTimelineSchema,
    Prize,
};
use imphnen_libs::ResourceEnum;
use anyhow::{Result, anyhow, bail};

use imphnen_libs::AppState;
use imphnen_utils::{QueryListBuilder, get_iso_date};

use std::collections::HashMap;
use surrealdb::sql::Thing;
use tracing::{instrument, info};

#[derive(Clone)]
pub struct HackathonRepository<'a> {
    pub state: &'a AppState,
}

impl<'a> HackathonRepository<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self { state }
    }
}

// Hackathon CRUD operations
impl<'a> HackathonRepository<'a> {
    #[instrument(skip(self, hackathon), err)]
    pub async fn create_hackathon(&self, hackathon: HackathonCreateRequestDto) -> Result<HackathonSchema> {
        let table = ResourceEnum::Hackathons.to_string();
        let id = surrealdb::Uuid::new_v4().to_string();

        let prizes: Option<Vec<Prize>> = hackathon.prizes.map(|p| {
            p.into_iter()
                .map(|prize| Prize {
                    position: prize.position,
                    title: prize.title,
                    description: prize.description,
                    value: prize.value,
                })
                .collect()
        });

        let schema = HackathonSchema {
            id: Thing::from((table.clone(), id.clone())),
            name: hackathon.name,
            description: hackathon.description,
            start_date: hackathon.start_date,
            end_date: hackathon.end_date,
            registration_deadline: hackathon.registration_deadline,
            max_participants: hackathon.max_participants,
            status: super::hackathon_schema::HackathonStatus::Draft,
            theme: hackathon.theme,
            rules: hackathon.rules,
            prizes,
            organizers: hackathon.organizers,
            is_deleted: false,
            created_at: Some(get_iso_date()),
            updated_at: Some(get_iso_date()),
        };

        info!(query = %format!("CREATE {}:{}", table, id), "Executing SurrealDB query");
        let record: Option<HackathonSchema> = self
            .state.surrealdb_ws
            .create((table, id))
            .content(schema.clone())
            .await?;

        match record {
            Some(h) => Ok(h),
            None => bail!("Failed to create hackathon"),
        }
    }

    #[instrument(skip(self, id), err)]
    pub async fn get_hackathon_by_id(&self, id: String) -> Result<HackathonSchema> {
        let table = ResourceEnum::Hackathons.to_string();
        info!(query = %format!("SELECT * FROM {} WHERE id = '{}'", table, id), "Executing SurrealDB query");

        let record: Option<HackathonSchema> = self
            .state
            .surrealdb_ws
            .select((table, id))
            .await?;

        match record {
            Some(h) => {
                if h.is_deleted {
                    bail!("Hackathon not found");
                }
                Ok(h)
            }
            None => bail!("Hackathon not found"),
        }
    }

    #[instrument(skip(self, meta), err)]
    pub async fn list_hackathons(&self, meta: imphnen_libs::MetaRequestDto) -> Result<imphnen_libs::ResponseListSuccessDto<Vec<HackathonSchema>>> {
        let table = ResourceEnum::Hackathons.to_string();

        let builder = QueryListBuilder::new(&self.state.surrealdb_ws, &table, &meta)
            .with_condition("is_deleted = false")
            .search_field("name")
            .select_fields(vec!["*"]);

        let result = builder.build().await?;
        Ok(result)
    }

    #[instrument(skip(self, id, updates), err)]
    pub async fn update_hackathon(&self, id: String, updates: HackathonUpdateRequestDto) -> Result<HackathonSchema> {
        let table = ResourceEnum::Hackathons.to_string();

        // First get the existing hackathon
        let mut existing = self.get_hackathon_by_id(id.clone()).await?;

        // Apply updates
        if let Some(name) = updates.name {
            existing.name = name;
        }
        if let Some(description) = updates.description {
            existing.description = description;
        }
        if let Some(start_date) = updates.start_date {
            existing.start_date = start_date;
        }
        if let Some(end_date) = updates.end_date {
            existing.end_date = end_date;
        }
        if let Some(registration_deadline) = updates.registration_deadline {
            existing.registration_deadline = registration_deadline;
        }
        if let Some(max_participants) = updates.max_participants {
            existing.max_participants = Some(max_participants);
        }
        if let Some(theme) = updates.theme {
            existing.theme = Some(theme);
        }
        if let Some(rules) = updates.rules {
            existing.rules = Some(rules);
        }
        if let Some(prizes) = updates.prizes {
            let prizes_schema: Vec<Prize> = prizes
                .into_iter()
                .map(|p| Prize {
                    position: p.position,
                    title: p.title,
                    description: p.description,
                    value: p.value,
                })
                .collect();
            existing.prizes = Some(prizes_schema);
        }
        if let Some(organizers) = updates.organizers {
            existing.organizers = organizers;
        }

        existing.updated_at = Some(get_iso_date());

        info!(query = %format!("UPDATE {} SET ... WHERE id = '{}'", table, id), "Executing SurrealDB query");
        let record: Option<HackathonSchema> = self
            .state.surrealdb_ws
            .update((table, id))
            .content(existing.clone())
            .await?;

        match record {
            Some(h) => Ok(h),
            None => bail!("Failed to update hackathon"),
        }
    }

    #[instrument(skip(self, id), err)]
    pub async fn delete_hackathon(&self, id: String) -> Result<String> {
        let table = ResourceEnum::Hackathons.to_string();

        // Soft delete by setting is_deleted = true
        let updates: HashMap<String, serde_json::Value> = HashMap::from([
            ("is_deleted".to_string(), true.into()),
            ("updated_at".to_string(), get_iso_date().into()),
        ]);

        info!(query = %format!("UPDATE {} SET is_deleted = true WHERE id = '{}'", table, id), "Executing SurrealDB query");
        let record: Option<HackathonSchema> = self
            .state.surrealdb_ws
            .update((table, id))
            .merge(serde_json::to_value(updates)?)
            .await?;

        match record {
            Some(_) => Ok("Hackathon deleted successfully".to_string()),
            None => bail!("Failed to delete hackathon"),
        }
    }
}

// Hackathon Events CRUD operations
impl<'a> HackathonRepository<'a> {
    #[instrument(skip(self, hackathon_id, event), err)]
    pub async fn create_hackathon_event(&self, hackathon_id: String, event: HackathonEventCreateRequestDto) -> Result<HackathonEventsSchema> {
        let table = ResourceEnum::HackathonEvents.to_string();
        let id = surrealdb::Uuid::new_v4().to_string();

        let schema = HackathonEventsSchema {
            id: Thing::from((table.clone(), id.clone())),
            hackathon_id: Thing::from(("app_hackathons".to_string(), hackathon_id)),
            title: event.title,
            description: event.description,
            event_type: event.event_type,
            start_time: event.start_time,
            end_time: event.end_time,
            location: event.location,
            virtual_link: event.virtual_link,
            max_attendees: event.max_attendees,
            is_mandatory: event.is_mandatory,
            is_deleted: false,
            created_at: Some(get_iso_date()),
            updated_at: Some(get_iso_date()),
        };

        info!(query = %format!("CREATE {}:{}", table, id), "Executing SurrealDB query");
        let record: Option<HackathonEventsSchema> = self
            .state.surrealdb_ws
            .create((table, id))
            .content(schema.clone())
            .await?;

        match record {
            Some(e) => Ok(e),
            None => bail!("Failed to create hackathon event"),
        }
    }

    #[instrument(skip(self, meta, hackathon_id), err)]
    pub async fn list_hackathon_events(&self, meta: imphnen_libs::MetaRequestDto, hackathon_id: String) -> Result<imphnen_libs::ResponseListSuccessDto<Vec<HackathonEventsSchema>>> {
        let table = ResourceEnum::HackathonEvents.to_string();

        let builder = QueryListBuilder::new(&self.state.surrealdb_ws, &table, &meta)
            .with_condition("is_deleted = false")
            .with_condition(&format!("hackathon_id = app_hackathons:{}", hackathon_id))
            .search_field("title")
            .select_fields(vec!["*"]);

        let result = builder.build().await?;
        Ok(result)
    }

    #[instrument(skip(self, id, updates), err)]
    pub async fn update_hackathon_event(&self, id: String, updates: HackathonEventUpdateRequestDto) -> Result<HackathonEventsSchema> {
        let table = ResourceEnum::HackathonEvents.to_string();

        // Get existing event
        let existing: Option<HackathonEventsSchema> = self.state.surrealdb_ws.select((table.clone(), id.clone())).await?;
        let mut existing = existing.ok_or_else(|| anyhow!("Event not found"))?;

        if existing.is_deleted {
            bail!("Event not found");
        }

        // Apply updates
        if let Some(title) = updates.title {
            existing.title = title;
        }
        if let Some(description) = updates.description {
            existing.description = Some(description);
        }
        if let Some(event_type) = updates.event_type {
            existing.event_type = event_type;
        }
        if let Some(start_time) = updates.start_time {
            existing.start_time = start_time;
        }
        if let Some(end_time) = updates.end_time {
            existing.end_time = end_time;
        }
        if let Some(location) = updates.location {
            existing.location = Some(location);
        }
        if let Some(virtual_link) = updates.virtual_link {
            existing.virtual_link = Some(virtual_link);
        }
        if let Some(max_attendees) = updates.max_attendees {
            existing.max_attendees = Some(max_attendees);
        }
        if let Some(is_mandatory) = updates.is_mandatory {
            existing.is_mandatory = is_mandatory;
        }

        existing.updated_at = Some(get_iso_date());

        info!(query = %format!("UPDATE {} SET ... WHERE id = '{}'", table, id), "Executing SurrealDB query");
        let record: Option<HackathonEventsSchema> = self
            .state.surrealdb_ws
            .update((table, id))
            .content(existing.clone())
            .await?;

        match record {
            Some(e) => Ok(e),
            None => bail!("Failed to update hackathon event"),
        }
    }

    #[instrument(skip(self, id), err)]
    pub async fn delete_hackathon_event(&self, id: String) -> Result<String> {
        let table = ResourceEnum::HackathonEvents.to_string();

        let updates: HashMap<String, serde_json::Value> = HashMap::from([
            ("is_deleted".to_string(), true.into()),
            ("updated_at".to_string(), get_iso_date().into()),
        ]);

        info!(query = %format!("UPDATE {} SET is_deleted = true WHERE id = '{}'", table, id), "Executing SurrealDB query");
        let record: Option<HackathonEventsSchema> = self
            .state.surrealdb_ws
            .update((table, id))
            .merge(serde_json::to_value(updates)?)
            .await?;

        match record {
            Some(_) => Ok("Event deleted successfully".to_string()),
            None => bail!("Failed to delete event"),
        }
    }
}

// Hackathon Timeline CRUD operations
impl<'a> HackathonRepository<'a> {
    #[instrument(skip(self, hackathon_id, timeline), err)]
    pub async fn create_hackathon_timeline(&self, hackathon_id: String, timeline: HackathonTimelineCreateRequestDto) -> Result<HackathonTimelineSchema> {
        let table = ResourceEnum::HackathonTimeline.to_string();
        let id = surrealdb::Uuid::new_v4().to_string();

        let schema = HackathonTimelineSchema {
            id: Thing::from((table.clone(), id.clone())),
            hackathon_id: Thing::from(("app_hackathons".to_string(), hackathon_id)),
            phase: timeline.phase,
            title: timeline.title,
            description: timeline.description,
            start_date: timeline.start_date,
            end_date: timeline.end_date,
            is_active: timeline.is_active,
            order: timeline.order,
            is_deleted: false,
            created_at: Some(get_iso_date()),
            updated_at: Some(get_iso_date()),
        };

        info!(query = %format!("CREATE {}:{}", table, id), "Executing SurrealDB query");
        let record: Option<HackathonTimelineSchema> = self
            .state.surrealdb_ws
            .create((table, id))
            .content(schema.clone())
            .await?;

        match record {
            Some(t) => Ok(t),
            None => bail!("Failed to create hackathon timeline"),
        }
    }

    #[instrument(skip(self, meta, hackathon_id), err)]
    pub async fn list_hackathon_timeline(&self, meta: imphnen_libs::MetaRequestDto, hackathon_id: String) -> Result<imphnen_libs::ResponseListSuccessDto<Vec<HackathonTimelineSchema>>> {
        let table = ResourceEnum::HackathonTimeline.to_string();

        let builder = QueryListBuilder::new(&self.state.surrealdb_ws, &table, &meta)
            .with_condition("is_deleted = false")
            .with_condition(&format!("hackathon_id = app_hackathons:{}", hackathon_id))
            .search_field("title")
            .select_fields(vec!["*"]);

        let result = builder.build().await?;
        Ok(result)
    }

    #[instrument(skip(self, id, updates), err)]
    pub async fn update_hackathon_timeline(&self, id: String, updates: HackathonTimelineUpdateRequestDto) -> Result<HackathonTimelineSchema> {
        let table = ResourceEnum::HackathonTimeline.to_string();

        let existing: Option<HackathonTimelineSchema> = self.state.surrealdb_ws.select((table.clone(), id.clone())).await?;
        let mut existing = existing.ok_or_else(|| anyhow!("Timeline not found"))?;

        if existing.is_deleted {
            bail!("Timeline not found");
        }

        // Apply updates
        if let Some(phase) = updates.phase {
            existing.phase = phase;
        }
        if let Some(title) = updates.title {
            existing.title = title;
        }
        if let Some(description) = updates.description {
            existing.description = Some(description);
        }
        if let Some(start_date) = updates.start_date {
            existing.start_date = start_date;
        }
        if let Some(end_date) = updates.end_date {
            existing.end_date = end_date;
        }
        if let Some(is_active) = updates.is_active {
            existing.is_active = is_active;
        }
        if let Some(order) = updates.order {
            existing.order = order;
        }

        existing.updated_at = Some(get_iso_date());

        info!(query = %format!("UPDATE {} SET ... WHERE id = '{}'", table, id), "Executing SurrealDB query");
        let record: Option<HackathonTimelineSchema> = self
            .state.surrealdb_ws
            .update((table, id))
            .content(existing.clone())
            .await?;

        match record {
            Some(t) => Ok(t),
            None => bail!("Failed to update hackathon timeline"),
        }
    }

    #[instrument(skip(self, id), err)]
    pub async fn delete_hackathon_timeline(&self, id: String) -> Result<String> {
        let table = ResourceEnum::HackathonTimeline.to_string();

        let updates: HashMap<String, serde_json::Value> = HashMap::from([
            ("is_deleted".to_string(), true.into()),
            ("updated_at".to_string(), get_iso_date().into()),
        ]);

        info!(query = %format!("UPDATE {} SET is_deleted = true WHERE id = '{}'", table, id), "Executing SurrealDB query");
        let record: Option<HackathonTimelineSchema> = self
            .state.surrealdb_ws
            .update((table, id))
            .merge(serde_json::to_value(updates)?)
            .await?;

        match record {
            Some(_) => Ok("Timeline deleted successfully".to_string()),
            None => bail!("Failed to delete timeline"),
        }
    }
}

// Hackathon Submissions CRUD operations
impl<'a> HackathonRepository<'a> {
    #[instrument(skip(self, hackathon_id, team_id, submission), err)]
    pub async fn create_hackathon_submission(&self, hackathon_id: String, team_id: String, submission: HackathonSubmissionCreateRequestDto) -> Result<HackathonSubmissionsSchema> {
        let table = ResourceEnum::HackathonSubmissions.to_string();
        let id = surrealdb::Uuid::new_v4().to_string();

        let schema = HackathonSubmissionsSchema {
            id: Thing::from((table.clone(), id.clone())),
            hackathon_id: Thing::from(("app_hackathons".to_string(), hackathon_id)),
            team_id: Thing::from(("app_teams".to_string(), team_id)),
            project_name: submission.project_name,
            description: submission.description,
            repository_url: submission.repository_url,
            demo_url: submission.demo_url,
            slides_url: submission.slides_url,
            technologies: submission.technologies,
            submission_status: super::hackathon_schema::SubmissionStatus::Draft,
            submitted_at: chrono::Utc::now(),
            is_deleted: false,
            created_at: Some(get_iso_date()),
            updated_at: Some(get_iso_date()),
        };

        info!(query = %format!("CREATE {}:{}", table, id), "Executing SurrealDB query");
        let record: Option<HackathonSubmissionsSchema> = self
            .state.surrealdb_ws
            .create((table, id))
            .content(schema.clone())
            .await?;

        match record {
            Some(s) => Ok(s),
            None => bail!("Failed to create hackathon submission"),
        }
    }

    #[instrument(skip(self, meta, hackathon_id), err)]
    pub async fn list_hackathon_submissions(&self, meta: imphnen_libs::MetaRequestDto, hackathon_id: String) -> Result<imphnen_libs::ResponseListSuccessDto<Vec<HackathonSubmissionsSchema>>> {
        let table = ResourceEnum::HackathonSubmissions.to_string();

        let builder = QueryListBuilder::new(&self.state.surrealdb_ws, &table, &meta)
            .with_condition("is_deleted = false")
            .with_condition(&format!("hackathon_id = app_hackathons:{}", hackathon_id))
            .search_field("project_name")
            .select_fields(vec!["*"]);

        let result = builder.build().await?;
        Ok(result)
    }

    #[instrument(skip(self, id, updates), err)]
    pub async fn update_hackathon_submission(&self, id: String, updates: HackathonSubmissionUpdateRequestDto) -> Result<HackathonSubmissionsSchema> {
        let table = ResourceEnum::HackathonSubmissions.to_string();

        let existing: Option<HackathonSubmissionsSchema> = self.state.surrealdb_ws.select((table.clone(), id.clone())).await?;
        let mut existing = existing.ok_or_else(|| anyhow!("Submission not found"))?;

        if existing.is_deleted {
            bail!("Submission not found");
        }

        // Apply updates
        if let Some(project_name) = updates.project_name {
            existing.project_name = project_name;
        }
        if let Some(description) = updates.description {
            existing.description = description;
        }
        if let Some(repository_url) = updates.repository_url {
            existing.repository_url = Some(repository_url);
        }
        if let Some(demo_url) = updates.demo_url {
            existing.demo_url = Some(demo_url);
        }
        if let Some(slides_url) = updates.slides_url {
            existing.slides_url = Some(slides_url);
        }
        if let Some(technologies) = updates.technologies {
            existing.technologies = technologies;
        }

        existing.updated_at = Some(get_iso_date());

        info!(query = %format!("UPDATE {} SET ... WHERE id = '{}'", table, id), "Executing SurrealDB query");
        let record: Option<HackathonSubmissionsSchema> = self
            .state.surrealdb_ws
            .update((table, id))
            .content(existing.clone())
            .await?;

        match record {
            Some(s) => Ok(s),
            None => bail!("Failed to update hackathon submission"),
        }
    }

    #[instrument(skip(self, id), err)]
    pub async fn submit_hackathon_submission(&self, id: String) -> Result<HackathonSubmissionsSchema> {
        let table = ResourceEnum::HackathonSubmissions.to_string();

        let existing: Option<HackathonSubmissionsSchema> = self.state.surrealdb_ws.select((table.clone(), id.clone())).await?;
        let mut existing = existing.ok_or_else(|| anyhow!("Submission not found"))?;

        if existing.is_deleted {
            bail!("Submission not found");
        }

        existing.submission_status = super::hackathon_schema::SubmissionStatus::Submitted;
        existing.submitted_at = chrono::Utc::now();
        existing.updated_at = Some(get_iso_date());

        info!(query = %format!("UPDATE {} SET submission_status = 'Submitted' WHERE id = '{}'", table, id), "Executing SurrealDB query");
        let record: Option<HackathonSubmissionsSchema> = self
            .state.surrealdb_ws
            .update((table, id))
            .content(existing.clone())
            .await?;

        match record {
            Some(s) => Ok(s),
            None => bail!("Failed to submit hackathon submission"),
        }
    }

    #[instrument(skip(self, id), err)]
    pub async fn delete_hackathon_submission(&self, id: String) -> Result<String> {
        let table = ResourceEnum::HackathonSubmissions.to_string();

        let updates: HashMap<String, serde_json::Value> = HashMap::from([
            ("is_deleted".to_string(), true.into()),
            ("updated_at".to_string(), get_iso_date().into()),
        ]);

        info!(query = %format!("UPDATE {} SET is_deleted = true WHERE id = '{}'", table, id), "Executing SurrealDB query");
        let record: Option<HackathonSubmissionsSchema> = self
            .state.surrealdb_ws
            .update((table, id))
            .merge(serde_json::to_value(updates)?)
            .await?;

        match record {
            Some(_) => Ok("Submission deleted successfully".to_string()),
            None => bail!("Failed to delete submission"),
        }
    }
}