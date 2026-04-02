use super::postgres_team_repository::{PostgresTeamRepository, TeamRow, UserRow};
use crate::teams::domain::entity::{BrowseTeamsInput, TeamEntity, TeamUserInfo};
use imphnen_utils::errors::AppError;
use sqlx::FromRow;
use uuid::Uuid;

impl PostgresTeamRepository {
	pub(super) async fn browse_query(
		&self,
		input: BrowseTeamsInput,
	) -> Result<(Vec<TeamEntity>, i64), AppError> {
		let offset = (input.page - 1) * input.per_page;
		let mut where_clauses: Vec<String> = vec!["t.visibility = 'public'".to_string()];
		let mut param_count = 1usize;

		if input.city.is_some() {
			where_clauses.push(format!("t.city = ${}", param_count));
			param_count += 1;
		}
		if input.search.is_some() {
			where_clauses.push(format!("t.name ILIKE ${}", param_count));
			param_count += 1;
		}
		if input.min_members.is_some() {
			where_clauses.push(format!("mc.member_count >= ${}", param_count));
			param_count += 1;
		}
		if input.max_members.is_some() {
			where_clauses.push(format!("mc.member_count <= ${}", param_count));
			param_count += 1;
		}
		if let Some(has_sub) = input.has_submission {
			let clause = if has_sub {
				"EXISTS(SELECT 1 FROM hackathon_project_submissions WHERE team_id = t.id)"
					.to_string()
			} else {
				"NOT EXISTS(SELECT 1 FROM hackathon_project_submissions WHERE team_id = t.id)".to_string()
			};
			where_clauses.push(clause);
		}

		let where_sql = where_clauses.join(" AND ");
		let base = format!(
			"FROM hackathon_teams t LEFT JOIN (SELECT team_id, COUNT(*) as member_count FROM hackathon_team_members WHERE status = 'active' GROUP BY team_id) mc ON mc.team_id = t.id WHERE {}",
			where_sql
		);

		let count_sql = format!("SELECT COUNT(*) {}", base);
		let mut count_q = sqlx::query_scalar::<_, i64>(&count_sql);
		if let Some(ref v) = input.city {
			count_q = count_q.bind(v.clone());
		}
		if let Some(ref v) = input.search {
			count_q = count_q.bind(format!("%{}%", v));
		}
		if let Some(v) = input.min_members {
			count_q = count_q.bind(v);
		}
		if let Some(v) = input.max_members {
			count_q = count_q.bind(v);
		}
		let total: i64 = count_q.fetch_one(self.pool.as_ref()).await.unwrap_or(0);

		let select_sql = format!(
			"SELECT t.id, t.name, t.description, t.city, t.visibility, t.logo, t.banner, t.leader_id, t.created_at, t.updated_at {} ORDER BY t.created_at DESC LIMIT ${} OFFSET ${}",
			base,
			param_count,
			param_count + 1
		);
		let mut q = sqlx::query_as::<_, TeamRow>(&select_sql);
		if let Some(v) = input.city {
			q = q.bind(v);
		}
		if let Some(v) = input.search {
			q = q.bind(format!("%{}%", v));
		}
		if let Some(v) = input.min_members {
			q = q.bind(v);
		}
		if let Some(v) = input.max_members {
			q = q.bind(v);
		}
		q = q.bind(input.per_page).bind(offset);
		let rows = q
			.fetch_all(self.pool.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;
		Ok((rows.into_iter().map(Into::into).collect(), total))
	}

	pub(super) async fn update_query(
		&self,
		id: Uuid,
		input: crate::teams::domain::entity::UpdateTeamInput,
	) -> Result<TeamEntity, AppError> {
		use chrono::Utc;
		let mut sets = vec!["updated_at = $1".to_string()];
		let mut idx = 2usize;
		if input.name.is_some() {
			sets.push(format!("name = ${}", idx));
			idx += 1;
		}
		if input.description.is_some() {
			sets.push(format!("description = ${}", idx));
			idx += 1;
		}
		if input.city.is_some() {
			sets.push(format!("city = ${}", idx));
			idx += 1;
		}
		if input.visibility.is_some() {
			sets.push(format!("visibility = ${}", idx));
			idx += 1;
		}
		if input.logo.is_some() {
			sets.push(format!("logo = ${}", idx));
			idx += 1;
		}
		if input.banner.is_some() {
			sets.push(format!("banner = ${}", idx));
			idx += 1;
		}
		let sql = format!(
			"UPDATE hackathon_teams SET {} WHERE id = ${} RETURNING id, name, description, city, visibility, logo, banner, leader_id, created_at, updated_at",
			sets.join(", "),
			idx
		);
		let mut q = sqlx::query_as::<_, TeamRow>(&sql).bind(Utc::now());
		if let Some(v) = input.name {
			q = q.bind(v);
		}
		if let Some(v) = input.description {
			q = q.bind(v);
		}
		if let Some(v) = input.city {
			q = q.bind(v);
		}
		if let Some(v) = input.visibility {
			q = q.bind(v);
		}
		if let Some(v) = input.logo {
			q = q.bind(v);
		}
		if let Some(v) = input.banner {
			q = q.bind(v);
		}
		q.bind(id)
			.fetch_one(self.pool.as_ref())
			.await
			.map(Into::into)
			.map_err(|e| AppError::InternalServerError(e.to_string()))
	}

	pub(super) async fn leaders_batch_query(
		&self,
		leader_ids: Vec<Uuid>,
	) -> Result<Vec<TeamUserInfo>, AppError> {
		if leader_ids.is_empty() {
			return Ok(vec![]);
		}
		let placeholders = (1..=leader_ids.len())
			.map(|i| format!("${}", i))
			.collect::<Vec<_>>()
			.join(", ");
		let sql = format!(
			"SELECT id, email, fullname, avatar, phone_number, location, bio, skills, is_active, created_at, updated_at FROM hackathon_users WHERE id IN ({})",
			placeholders
		);
		let mut q = sqlx::query_as::<_, UserRow>(&sql);
		for id in &leader_ids {
			q = q.bind(id);
		}
		let rows = q
			.fetch_all(self.pool.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;
		Ok(rows.into_iter().map(Into::into).collect())
	}

	pub(super) async fn member_counts_batch_query(
		&self,
		team_ids: Vec<Uuid>,
	) -> Result<Vec<(Uuid, i64)>, AppError> {
		if team_ids.is_empty() {
			return Ok(vec![]);
		}
		let placeholders = (1..=team_ids.len())
			.map(|i| format!("${}", i))
			.collect::<Vec<_>>()
			.join(", ");
		let sql = format!(
			"SELECT team_id, COUNT(*) as count FROM hackathon_team_members WHERE team_id IN ({}) AND status = 'active' GROUP BY team_id",
			placeholders
		);
		#[derive(FromRow)]
		struct CountRow {
			team_id: Uuid,
			count: i64,
		}
		let mut q = sqlx::query_as::<_, CountRow>(&sql);
		for id in &team_ids {
			q = q.bind(id);
		}
		let rows = q
			.fetch_all(self.pool.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;
		Ok(rows.into_iter().map(|r| (r.team_id, r.count)).collect())
	}

	pub(super) async fn submitted_team_ids_query(
		&self,
		team_ids: Vec<Uuid>,
	) -> Result<Vec<Uuid>, AppError> {
		if team_ids.is_empty() {
			return Ok(vec![]);
		}
		let placeholders = (1..=team_ids.len())
			.map(|i| format!("${}", i))
			.collect::<Vec<_>>()
			.join(", ");
		let sql = format!(
			"SELECT DISTINCT team_id FROM hackathon_project_submissions WHERE team_id IN ({})",
			placeholders
		);
		#[derive(FromRow)]
		struct SubRow {
			team_id: Uuid,
		}
		let mut q = sqlx::query_as::<_, SubRow>(&sql);
		for id in &team_ids {
			q = q.bind(id);
		}
		let rows = q
			.fetch_all(self.pool.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;
		Ok(rows.into_iter().map(|r| r.team_id).collect())
	}
}
