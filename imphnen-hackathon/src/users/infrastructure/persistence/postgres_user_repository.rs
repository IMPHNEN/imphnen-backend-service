use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use async_trait::async_trait;
use sqlx::{PgPool, FromRow};
use imphnen_utils::errors::AppError;
use crate::users::domain::entity::{HackathonUserEntity, UpdateUserInput};
use crate::users::domain::repository::HackathonUserRepository;

#[derive(FromRow)]
struct UserRow {
    id: Uuid,
    email: String,
    fullname: String,
    avatar: Option<String>,
    phone_number: Option<String>,
    location: Option<String>,
    bio: Option<String>,
    skills: Option<Vec<String>>,
    is_active: Option<bool>,
    created_at: Option<chrono::DateTime<Utc>>,
    updated_at: Option<chrono::DateTime<Utc>>,
}

impl From<UserRow> for HackathonUserEntity {
    fn from(r: UserRow) -> Self {
        Self {
            id: r.id, email: r.email, fullname: r.fullname, avatar: r.avatar,
            phone_number: r.phone_number, location: r.location, bio: r.bio,
            skills: r.skills, is_active: r.is_active,
            created_at: r.created_at, updated_at: r.updated_at,
        }
    }
}

pub struct PostgresHackathonUserRepository {
    pool: Arc<PgPool>,
}

impl PostgresHackathonUserRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl HackathonUserRepository for PostgresHackathonUserRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<HackathonUserEntity, AppError> {
        sqlx::query_as::<_, UserRow>(
            "SELECT id, email, fullname, avatar, phone_number, location, bio, skills, is_active, created_at, updated_at FROM hackathon_users WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?
        .map(Into::into)
        .ok_or_else(|| AppError::NotFoundError("User not found".to_string()))
    }

    async fn update(&self, id: Uuid, input: UpdateUserInput) -> Result<HackathonUserEntity, AppError> {
        let mut sets = Vec::new();
        let mut idx = 1usize;
        if input.fullname.is_some() { sets.push(format!("fullname = ${}", idx)); idx += 1; }
        if input.phone_number.is_some() { sets.push(format!("phone_number = ${}", idx)); idx += 1; }
        if input.avatar.is_some() { sets.push(format!("avatar = ${}", idx)); idx += 1; }
        if input.location.is_some() { sets.push(format!("location = ${}", idx)); idx += 1; }
        if input.bio.is_some() { sets.push(format!("bio = ${}", idx)); idx += 1; }
        if input.skills.is_some() { sets.push(format!("skills = ${}", idx)); idx += 1; }
        if sets.is_empty() { return self.find_by_id(id).await; }
        sets.push(format!("updated_at = ${}", idx));
        let sql = format!(
            "UPDATE hackathon_users SET {} WHERE id = ${} RETURNING id, email, fullname, avatar, phone_number, location, bio, skills, is_active, created_at, updated_at",
            sets.join(", "), idx + 1
        );
        let mut q = sqlx::query_as::<_, UserRow>(&sql);
        if let Some(v) = input.fullname { q = q.bind(v); }
        if let Some(v) = input.phone_number { q = q.bind(v); }
        if let Some(v) = input.avatar { q = q.bind(v); }
        if let Some(v) = input.location { q = q.bind(v); }
        if let Some(v) = input.bio { q = q.bind(v); }
        if let Some(v) = input.skills { q = q.bind(v); }
        q.bind(Utc::now()).bind(id)
            .fetch_one(self.pool.as_ref())
            .await
            .map(Into::into)
            .map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    async fn get_user_teams(&self, user_id: Uuid) -> Result<Vec<serde_json::Value>, AppError> {
        #[derive(FromRow)]
        struct TeamRow {
            id: Uuid, name: String, description: String, city: String, visibility: String,
            logo: Option<String>, banner: Option<String>, leader_id: Uuid,
            created_at: chrono::DateTime<Utc>, updated_at: chrono::DateTime<Utc>,
        }
        let rows = sqlx::query_as::<_, TeamRow>(
            "SELECT t.id, t.name, t.description, t.city, t.visibility, t.logo, t.banner, t.leader_id, t.created_at, t.updated_at FROM hackathon_teams t JOIN hackathon_team_members tm ON t.id = tm.team_id WHERE tm.user_id = $1 AND tm.status = 'active' ORDER BY t.created_at DESC"
        )
        .bind(user_id)
        .fetch_all(self.pool.as_ref())
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;
        Ok(rows.into_iter().map(|r| serde_json::json!({
            "id": r.id, "name": r.name, "description": r.description, "city": r.city,
            "visibility": r.visibility, "logo": r.logo, "banner": r.banner,
            "leader_id": r.leader_id, "created_at": r.created_at, "updated_at": r.updated_at
        })).collect())
    }
}
