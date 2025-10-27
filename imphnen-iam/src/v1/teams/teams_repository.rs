use super::{
	TeamsDetailQueryDto, TeamsListQueryDto, TeamsListItemDto, TeamsSchema,
	TeamMembersSchema, TeamInvitationsSchema, TeamMembersQueryDto, TeamInvitationsQueryDto,
	TeamsSearchQueryDto
};
use imphnen_libs::{
    AppState, MetaRequestDto, ResourceEnum, ResponseListSuccessDto
};
use imphnen_utils::{
	get_id, DetailQueryBuilder, QueryListBuilder, make_thing_from_enum,
	build_multi_thing_condition, execute_safe_update_query,
};
use surrealdb::sql::Thing;
use anyhow::{Result, bail};
use serde_json;
use std::time::Instant;

pub struct TeamsRepository<'a> {
	state: &'a AppState,
}

impl<'a> TeamsRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	pub async fn query_team_list(
		&self,
		meta: MetaRequestDto,
	) -> Result<ResponseListSuccessDto<Vec<TeamsListItemDto>>> {
		let now = Instant::now();
		let result: ResponseListSuccessDto<Vec<TeamsListQueryDto>> =
			QueryListBuilder::new(
				&self.state.surrealdb_ws,
				&ResourceEnum::Teams.to_string(),
				&meta,
			)
			.with_condition("is_deleted = false AND is_active = true")
			.search_field("name")
			.select_fields(vec!["*"])
			.fetch_fields(vec![])
			.build()
			.await?;
		let elapsed = now.elapsed();

		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_team_list' took: {elapsed:.2?}");
		}

		let data = result
			.data
			.into_iter()
			.map(|dto| dto.into_list_item_dto())
			.collect();
		Ok(ResponseListSuccessDto {
			data,
			meta: result.meta,
		})
	}

	pub async fn query_team_by_id(&self, id: &Thing) -> Result<TeamsDetailQueryDto> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		let builder = DetailQueryBuilder::new(ResourceEnum::Teams.to_string())
			.with_id(id.id.to_raw())
			.with_select_fields(vec!["*"]);
		let sql = builder.build();
		let result: Option<TeamsDetailQueryDto> =
			builder.apply_bindings(db.query(sql)).await?.take(0)?;
		let elapsed = now.elapsed();

		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_team_by_id' took: {elapsed:.2?}");
		}

		let Some(team) = result else {
			bail!("Team not found");
		};
		if team.is_deleted {
			bail!("Team not found");
		}
		Ok(team)
	}

	pub async fn query_create_team(&self, data: TeamsSchema) -> Result<String> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		let record: Option<TeamsSchema> = db
			.create(ResourceEnum::Teams.to_string())
			.content(data)
			.await?;
		let elapsed = now.elapsed();

		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_create_team' took: {elapsed:.2?}");
		}

		match record {
			Some(saved) => {
				// Return the created team id as part of the message so callers can parse it in tests
				let id = saved.id.id.to_raw();
				Ok(format!("Success create team {}", id))
			}
			None => bail!("Failed to create team"),
		}
	}

	pub async fn query_update_team(&self, data: TeamsSchema) -> Result<String> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		let record_key = get_id(&data.id)?;
		let existing = self.query_team_by_id(&data.id).await?;
		if existing.is_deleted {
			bail!("Team already deleted");
		}
		let merged = TeamsSchema {
			created_at: existing.created_at,
			..data.clone()
		};
		let record: Option<TeamsSchema> = db.update(record_key).merge(merged).await?;
		let elapsed = now.elapsed();

		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_update_team' took: {elapsed:.2?}");
		}

		match record {
			Some(_) => Ok("Success update team".into()),
			None => bail!("Failed to update team"),
		}
	}

	pub async fn query_delete_team(&self, id: String) -> Result<String> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		let team = self.query_team_by_id(&make_thing_from_enum(ResourceEnum::Teams, &id)).await?;
		if team.is_deleted {
			bail!("Team not found");
		}
		let record_key = get_id(&team.id)?;
		let record: Option<TeamsSchema> = db
			.update(record_key)
			.merge(serde_json::json!({ "is_deleted": true }))
			.await?;
		let elapsed = now.elapsed();

		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_delete_team' took: {elapsed:.2?}");
		}

		match record {
			Some(_) => Ok("Success delete team".into()),
			None => bail!("Failed to delete team"),
		}
	}

	pub async fn query_add_team_member(&self, data: TeamMembersSchema) -> Result<String> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		let record: Option<TeamMembersSchema> = db
			.create(ResourceEnum::TeamMembers.to_string())
			.content(data)
			.await?;
		let elapsed = now.elapsed();

		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_add_team_member' took: {elapsed:.2?}");
		}

		match record {
			Some(saved_member) => {
				println!("Member saved with ID: {:?}", saved_member.id);
				Ok("Success add team member".into())
			},
			None => bail!("Failed to add team member"),
		}
	}

	pub async fn query_team_members(&self, team_id: &Thing) -> Result<Vec<TeamMembersQueryDto>> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		
		let builder = DetailQueryBuilder::new(ResourceEnum::TeamMembers.to_string())
			.with_thing_equals("team_id", team_id)
			.with_condition("is_active = true")
			.with_select_fields(vec!["*"]);
			
		let sql = builder.build();
		let mut result = db.query(sql).await?;
		
		let members: Vec<TeamMembersQueryDto> = match result.take(0) {
			Ok(members) => members,
			Err(e) => {
				println!("Error getting team members: {:?}", e);
				vec![]
			}
		};
		let elapsed = now.elapsed();

		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_team_members' returned {} members", members.len());
			println!("Query 'query_team_members' took: {elapsed:.2?}");
		}

		Ok(members)
	}

	pub async fn query_teams_by_user(&self, user_id: &Thing) -> Result<Vec<TeamsDetailQueryDto>> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		let sql = format!(
			"SELECT team.* FROM {} membership
			 INNER JOIN {} team ON membership.team_id = team.id
			 WHERE membership.user_id = $user_id
			 AND membership.is_active = true
			 AND team.is_deleted = false
			 AND team.is_active = true",
			ResourceEnum::TeamMembers,
			ResourceEnum::Teams
		);
		let mut result = db.query(sql).bind(("user_id", user_id.id.to_raw())).await?;
		let teams: Vec<TeamsDetailQueryDto> = result.take(0)?;
		let elapsed = now.elapsed();

		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_user_teams' took: {elapsed:.2?}");
		}

		Ok(teams)
	}

	pub async fn query_is_team_member(&self, team_id: &Thing, user_id: &Thing) -> Result<bool> {
			let now = Instant::now();
			let db = &self.state.surrealdb_ws;
			
			// Use direct SQL query for more control over the team member check
			let sql = format!(
				"SELECT COUNT() AS count FROM {}
				WHERE team_id = $team_id
				AND user_id = $user_id
				AND is_active = true",
				ResourceEnum::TeamMembers
			);
			
			let mut result = db.query(sql)
				.bind(("team_id", team_id.id.to_raw()))
				.bind(("user_id", user_id.id.to_raw()))
				.await?;
			
			// Use a simpler approach to get the count
			let count = match result.take(0) {
				Ok(Some(surrealdb::sql::Value::Number(num))) => num.to_int(),
				_ => 0,
			};
			
			let elapsed = now.elapsed();
	
			if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
				== "development"
			{
				println!("Query 'query_is_team_member' found {} matching members", count);
				println!("Query 'query_is_team_member' took: {elapsed:.2?}");
			}
	
			Ok(count > 0)
		}

	pub async fn query_create_invitation(&self, data: TeamInvitationsSchema) -> Result<String> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		let record: Option<TeamInvitationsSchema> = db
			.create(ResourceEnum::TeamInvitations.to_string())
			.content(data)
			.await?;
		let elapsed = now.elapsed();

		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_create_invitation' took: {elapsed:.2?}");
		}

		match record {
			Some(_) => Ok("Success create invitation".into()),
			None => bail!("Failed to create invitation"),
		}
	}

	pub async fn query_invitation_by_token(&self, token: &str) -> Result<TeamInvitationsQueryDto> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		let sql = format!(
			"SELECT * FROM {} WHERE invite_code = $invite_code AND status = 'pending' LIMIT 1",
			ResourceEnum::TeamInvitations
		);
		let mut result = db.query(sql).bind(("invite_code", token.to_string())).await?;
		let invitation: Option<TeamInvitationsQueryDto> = result.take(0)?;
		let elapsed = now.elapsed();

		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_invitation_by_token' took: {elapsed:.2?}");
		}

		invitation.ok_or_else(|| anyhow::anyhow!("Invitation not found"))
	}

	pub async fn query_update_invitation(&self, data: TeamInvitationsSchema) -> Result<String> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		let record_key = get_id(&data.id)?;
		let record: Option<TeamInvitationsSchema> = db.update(record_key).merge(data).await?;
		let elapsed = now.elapsed();

		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_update_invitation' took: {elapsed:.2?}");
		}

		match record {
			Some(_) => Ok("Success update invitation".into()),
			None => bail!("Failed to update invitation"),
		}
	}

	pub async fn query_search_teams(
		&self,
		search_params: TeamsSearchQueryDto,
	) -> Result<ResponseListSuccessDto<Vec<TeamsListItemDto>>> {
		let now = Instant::now();
		let page = search_params.page.unwrap_or(1);
		let per_page = search_params.per_page.unwrap_or(10);
		
		let mut conditions = vec!["is_deleted = false".to_string(), "is_active = true".to_string()];
		
		if let Some(open) = search_params.open
			&& open {
			conditions.push("is_open = true".to_string());
		}
		
		if let Some(location) = &search_params.location {
			conditions.push(format!("location CONTAINS '{}'", location));
		}
		
		let mut query_conditions = conditions.join(" AND ");
		
		if let Some(query) = &search_params.query {
			query_conditions = format!("({}) AND (name CONTAINS '{}' OR description CONTAINS '{}')", query_conditions, query, query);
		}
		
		if let Some(skills) = &search_params.skills {
			for skill in skills.iter() {
				query_conditions = format!("{} AND skills_required CONTAINS '{}'", query_conditions, skill);
			}
		}

		let meta = MetaRequestDto {
			page: Some(page.try_into().unwrap()),
			per_page: Some(per_page.try_into().unwrap()),
			search: None, // Don't use built-in search since we're doing custom filtering
			sort_by: Some("created_at".to_string()),
			order: Some("DESC".to_string()),
			filter: None,
			filter_by: None,
		};

		let result: ResponseListSuccessDto<Vec<TeamsListQueryDto>> = QueryListBuilder::new(
			&self.state.surrealdb_ws,
			&ResourceEnum::Teams.to_string(),
			&meta,
		)
		.with_condition(&query_conditions)
		.select_fields(vec!["*"])
		.build()
		.await?;
		let elapsed = now.elapsed();

		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_search_teams' took: {elapsed:.2?}");
		}

		let data = result
			.data
			.into_iter()
			.map(|dto| dto.into_list_item_dto())
			.collect();
		Ok(ResponseListSuccessDto {
			data,
			meta: result.meta,
		})
	}

	pub async fn query_remove_team_member(&self, team_id: &Thing, user_id: &Thing) -> Result<String> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		
		let conditions = build_multi_thing_condition(&[("team_id", team_id), ("user_id", user_id)]);
		let sql = format!(
			"UPDATE {} SET is_active = false WHERE {}",
			ResourceEnum::TeamMembers,
			conditions
		);
		
		execute_safe_update_query(db, sql).await?;
		
		let elapsed = now.elapsed();

		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_remove_team_member' took: {elapsed:.2?}");
		}

				Ok("Success remove team member".into())
	}

	pub async fn query_update_team_member_role(&self, team_id: &Thing, user_id: &Thing, role: &str) -> Result<String> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		
		let conditions = build_multi_thing_condition(&[("team_id", team_id), ("user_id", user_id)]);
		let sql = format!(
			"UPDATE {} SET role = '{}' WHERE {} AND is_active = true",
			ResourceEnum::TeamMembers,
			role,
			conditions
		);
		
		execute_safe_update_query(db, sql).await?;
		
		let elapsed = now.elapsed();

		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_update_team_member_role' took: {elapsed:.2?}");
		}

		Ok("Success update team member role".into())
	}

	pub async fn query_team_invitations(&self, team_id: &Thing) -> Result<Vec<TeamInvitationsQueryDto>> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		let team_id_clone = team_id.clone();
		
		let sql = format!(
			"SELECT * FROM {} WHERE team_id = $team_id AND status = 'pending' ORDER BY invited_at DESC",
			ResourceEnum::TeamInvitations
		);
		
		let mut result = db.query(&sql).bind(("team_id", team_id_clone)).await?;
		let invitations: Vec<TeamInvitationsQueryDto> = result.take(0)?;
		
		let elapsed = now.elapsed();

		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_team_invitations' took: {elapsed:.2?}");
		}

		Ok(invitations)
	}

	pub async fn query_user_invitations(&self, email: &str) -> Result<Vec<TeamInvitationsQueryDto>> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		
		let sql = format!(
			"SELECT * FROM {} WHERE email = '{}' AND status = 'pending' ORDER BY invited_at DESC",
			ResourceEnum::TeamInvitations,
			email
		);
		
		let mut result = db.query(&sql).await?;
		let invitations: Vec<TeamInvitationsQueryDto> = result.take(0)?;
		
		let elapsed = now.elapsed();

		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_user_invitations' took: {elapsed:.2?}");
		}

		Ok(invitations)
	}

	pub async fn query_delete_invitation(&self, token: &str) -> Result<String> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		
		let sql = format!(
			"UPDATE {} SET status = 'cancelled' WHERE invite_code = '{}'",
			ResourceEnum::TeamInvitations,
			token
		);
		
		execute_safe_update_query(db, sql).await?;
		
		let elapsed = now.elapsed();

		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_delete_invitation' took: {elapsed:.2?}");
		}

		Ok("Invitation cancelled successfully".into())
	}
}
