use super::{
	TeamsCreateRequestDto, TeamsUpdateRequestDto, TeamInviteRequestDto,
	TeamAcceptInvitationRequestDto, TeamsDetailItemDto, MemberTeamsDetailItemDto,
	TeamMemberDto, TeamsRepository, TeamsSchema, TeamMembersSchema,
	TeamInvitationsSchema, TeamsSearchQueryDto, PublicTeamsDetailItemDto, AdminTeamsListItemDto
};
use crate::{
	AppState, MetaRequestDto, ResponseListSuccessDto, ResponseSuccessDto,
	common_response, success_list_response, success_response, validate_request,
	UsersRepository
};
use axum::{http::StatusCode, response::Response};
use imphnen_libs::{ResourceEnum, send_email};
use imphnen_utils::{make_thing_from_enum, OtpManager};
use uuid::Uuid;
use std::pin::Pin;
use std::future::Future;
use anyhow::Result;
use tracing::{info, error};
use serde_json::json;
use chrono::Utc;

pub trait TeamsServiceTrait: Send + Sync + 'static {
	fn get_team_list(state: &AppState, meta: MetaRequestDto) -> Pin<Box<dyn Future<Output = Response> + Send>>;
	fn get_team_by_id(state: &AppState, id: String) -> Pin<Box<dyn Future<Output = Response> + Send>>;
	fn get_member_team_list(state: &AppState, meta: MetaRequestDto) -> Pin<Box<dyn Future<Output = Response> + Send>>;
	fn get_member_team_by_id(state: &AppState, id: String) -> Pin<Box<dyn Future<Output = Response> + Send>>;
	fn get_public_team_list(state: &AppState, meta: MetaRequestDto) -> Pin<Box<dyn Future<Output = Response> + Send>>;
	fn get_public_team_by_id(state: &AppState, id: String) -> Pin<Box<dyn Future<Output = Response> + Send>>;
	fn create_team(state: &AppState, claims: imphnen_libs::jsonwebtoken::Claims, new_team: TeamsCreateRequestDto) -> Pin<Box<dyn Future<Output = Response> + Send>>;
	fn update_team(state: &AppState, claims: imphnen_libs::jsonwebtoken::Claims, id: String, team: TeamsUpdateRequestDto) -> Pin<Box<dyn Future<Output = Response> + Send>>;
	fn update_team_admin(state: &AppState, claims: imphnen_libs::jsonwebtoken::Claims, id: String, team: TeamsUpdateRequestDto) -> Pin<Box<dyn Future<Output = Response> + Send>>;
	fn delete_team(state: &AppState, claims: imphnen_libs::jsonwebtoken::Claims, id: String) -> Pin<Box<dyn Future<Output = Response> + Send>>;
	fn delete_team_admin(state: &AppState, claims: imphnen_libs::jsonwebtoken::Claims, id: String) -> Pin<Box<dyn Future<Output = Response> + Send>>;
	fn invite_team_members(state: &AppState, claims: imphnen_libs::jsonwebtoken::Claims, team_id: String, invite: TeamInviteRequestDto) -> Pin<Box<dyn Future<Output = Response> + Send>>;
	fn invite_team_members_admin(state: &AppState, claims: imphnen_libs::jsonwebtoken::Claims, team_id: String, invite: TeamInviteRequestDto) -> Pin<Box<dyn Future<Output = Response> + Send>>;
	fn accept_invitation(state: &AppState, claims: imphnen_libs::jsonwebtoken::Claims, accept: TeamAcceptInvitationRequestDto) -> Pin<Box<dyn Future<Output = Response> + Send>>;
	fn get_team_members(state: &AppState, claims: imphnen_libs::jsonwebtoken::Claims, team_id: String) -> Pin<Box<dyn Future<Output = Response> + Send>>;
	fn leave_team(state: &AppState, claims: imphnen_libs::jsonwebtoken::Claims, team_id: String) -> Pin<Box<dyn Future<Output = Response> + Send>>;
	fn leave_current_team(state: &AppState, claims: imphnen_libs::jsonwebtoken::Claims) -> Pin<Box<dyn Future<Output = Response> + Send>>;
	fn get_my_team(state: &AppState, claims: imphnen_libs::jsonwebtoken::Claims) -> Pin<Box<dyn Future<Output = Response> + Send>>;
	fn get_team_invitations(state: &AppState, claims: imphnen_libs::jsonwebtoken::Claims, team_id: String) -> Pin<Box<dyn Future<Output = Response> + Send>>;
	fn cancel_invitation(state: &AppState, claims: imphnen_libs::jsonwebtoken::Claims, token: String) -> Pin<Box<dyn Future<Output = Response> + Send>>;
	fn get_my_invitations(state: &AppState, claims: imphnen_libs::jsonwebtoken::Claims) -> Pin<Box<dyn Future<Output = Response> + Send>>;
	fn search_teams(state: &AppState, search_params: TeamsSearchQueryDto) -> Pin<Box<dyn Future<Output = Response> + Send>>;
	fn get_admin_team_list(state: &AppState, meta: MetaRequestDto) -> Pin<Box<dyn Future<Output = Response> + Send>>;
	fn get_admin_team_by_id(state: &AppState, id: String) -> Pin<Box<dyn Future<Output = Response> + Send>>;
	fn get_admin_team_members(state: &AppState, team_id: String) -> Pin<Box<dyn Future<Output = Response> + Send>>;
}

#[derive(Clone)]
pub struct TeamsService;

impl TeamsService {
	async fn send_invitation_email(
		team_name: &str,
		inviter_name: &str,
		email: &str,
		token: &str,
		is_existing_user: bool,
	) -> Result<()> {
		let subject = format!("Invitation to join team: {}", team_name);
		
		let (action_text, action_url) = if is_existing_user {
			("Login and Accept Invitation", format!("https://app.example.com/login?redirect=/teams/invite/{}", token))
		} else {
			("Register and Join Team", format!("https://app.example.com/register?team_token={}", token))
		};

		let body = format!(
			"Hello,\n\n\
			You have been invited by {} to join the team '{}'.\n\n\
			{}\n\
			{}\n\n\
			This invitation will expire in 72 hours.\n\n\
			Best regards,\n\
			The Team",
			inviter_name, team_name, action_text, action_url
		);

		send_email(email, &subject, &body)
			.map_err(|e| anyhow::anyhow!("Failed to send invitation email: {}", e))?;

		info!("Invitation email sent to: {}", email);
		Ok(())
	}

	async fn generate_invitation_token() -> String {
		format!("team_{}_{}", Uuid::new_v4(), OtpManager::generate_otp().code)
	}

	async fn get_user_info_with_privacy(
		user_id: &str,
		requester_user_id: &str,
		is_team_member: bool,
		state: &AppState,
	) -> Result<TeamMemberDto> {
		let users_repo = UsersRepository::new(state);
		let user_thing = make_thing_from_enum(ResourceEnum::Users, user_id);
		let user = users_repo.query_user_by_id(&user_thing).await?;

		let show_sensitive_data = is_team_member || user_id == requester_user_id;

		Ok(TeamMemberDto {
			id: String::new(),
			user_id: user.id.id.to_raw(),
			fullname: user.fullname,
			email: if show_sensitive_data { Some(user.email) } else { None },
			avatar: user.avatar,
			role: "member".to_string(),
			skills: user.skills,
			joined_at: user.created_at,
		})
	}
}

impl TeamsServiceTrait for TeamsService {
	fn get_team_list(state: &AppState, meta: MetaRequestDto) -> Pin<Box<dyn Future<Output = Response> + Send>> {
		let state = state.to_owned();
		Box::pin(async move {
			let repo = TeamsRepository::new(&state);
			match repo.query_team_list(meta).await {
				Ok(data) => {
					let response = ResponseListSuccessDto {
						data: data.data,
						meta: data.meta,
					};
					success_list_response(response)
				}
				Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
			}
		})
	}
	
	fn get_team_by_id(state: &AppState, id: String) -> Pin<Box<dyn Future<Output = Response> + Send>> {
		let state = state.to_owned();
		Box::pin(async move {
			if id.trim().is_empty() {
				return common_response(StatusCode::BAD_REQUEST, "Invalid Team ID format");
			}
			let repo = TeamsRepository::new(&state);
			let thing_id = make_thing_from_enum(ResourceEnum::Teams, &id);
			match repo.query_team_by_id(&thing_id).await {
				Ok(team) if !team.is_deleted => {
					let team_thing = make_thing_from_enum(ResourceEnum::Teams, &team.id.id.to_raw());
						let members = repo.query_team_members(&team_thing).await.unwrap_or_default();
					let members_len = members.len();
					
					// For public team details, only show sensitive info if user is authenticated and part of the team
					let team_dto = TeamsDetailItemDto {
						id: team.id.id.to_raw(),
						name: team.name,
						description: team.description,
						leader: TeamMemberDto {
							id: String::new(),
							user_id: team.leader_id.id.to_raw(),
							fullname: String::new(),
							email: None,
							avatar: None,
							role: "leader".to_string(),
							skills: None,
							joined_at: team.created_at.clone(),
						},
						is_open: team.is_open,
						max_members: team.max_members,
						current_member_count: members_len as i32 + 1,
						skills_required: team.skills_required,
						location: team.location,
						avatar: team.avatar,
						website_url: team.website_url,
						github_url: team.github_url,
						members: None,
						is_active: team.is_active,
						created_at: team.created_at,
						updated_at: team.updated_at,
					};
					success_response(ResponseSuccessDto { data: team_dto })
				}
				Ok(_) => common_response(StatusCode::NOT_FOUND, "Team not found"),
				Err(e) => common_response(StatusCode::NOT_FOUND, &e.to_string()),
			}
		})
	}
	
	fn get_member_team_list(state: &AppState, meta: MetaRequestDto) -> Pin<Box<dyn Future<Output = Response> + Send>> {
		let state = state.to_owned();
		Box::pin(async move {
			let repo = TeamsRepository::new(&state);
			match repo.query_team_list(meta).await {
				Ok(data) => {
					let response = ResponseListSuccessDto {
						data: data.data,
						meta: data.meta,
					};
					success_list_response(response)
				}
				Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
			}
		})
	}
	
	fn get_member_team_by_id(state: &AppState, id: String) -> Pin<Box<dyn Future<Output = Response> + Send>> {
		let state = state.to_owned();
		Box::pin(async move {
			if id.trim().is_empty() {
				return common_response(StatusCode::BAD_REQUEST, "Invalid Team ID format");
			}
			let repo = TeamsRepository::new(&state);
			let thing_id = make_thing_from_enum(ResourceEnum::Teams, &id);
			match repo.query_team_by_id(&thing_id).await {
				Ok(team) if !team.is_deleted => {
					let team_thing = make_thing_from_enum(ResourceEnum::Teams, &team.id.id.to_raw());
						let members = repo.query_team_members(&team_thing).await.unwrap_or_default();
					let members_len = members.len();
					
					// For member team details, include all information including members list
					let mut member_dtos = Vec::new();
					for member in members {
						match Self::get_user_info_with_privacy(
							&member.user_id.id.to_raw(),
							"system",  // In member context, we show all user info
							true,       // In member context, we show all user info
							&state,
						).await {
							Ok(mut member_dto) => {
								member_dto.role = member.role;
								member_dto.joined_at = member.joined_at;
								member_dtos.push(member_dto);
							}
							Err(_) => continue,
						}
					}
					
					// Add leader with full info
					let leader_dto = match Self::get_user_info_with_privacy(
						&team.leader_id.id.to_raw(),
						"system",
						true,
						&state,
					).await {
						Ok(mut leader_dto) => {
							leader_dto.role = "leader".to_string();
							leader_dto
						}
						Err(_) => TeamMemberDto {
							id: String::new(),
							user_id: team.leader_id.id.to_raw(),
							fullname: String::new(),
							email: None,
							avatar: None,
							role: "leader".to_string(),
							skills: None,
							joined_at: team.created_at.clone(),
						}
					};
					
					let leader_dto_clone = leader_dto.clone();
					member_dtos.insert(0, leader_dto);
					
					let team_dto = MemberTeamsDetailItemDto {
						id: team.id.id.to_raw(),
						name: team.name,
						description: team.description,
						leader: leader_dto_clone,
						is_open: team.is_open,
						max_members: team.max_members,
						current_member_count: members_len as i32 + 1,
						skills_required: team.skills_required,
						location: team.location,
						avatar: team.avatar,
						website_url: team.website_url,
						github_url: team.github_url,
						members: member_dtos,
						is_active: team.is_active,
						created_at: team.created_at,
						updated_at: team.updated_at,
					};
					success_response(ResponseSuccessDto { data: team_dto })
				}
				Ok(_) => common_response(StatusCode::NOT_FOUND, "Team not found"),
				Err(e) => common_response(StatusCode::NOT_FOUND, &e.to_string()),
			}
		})
	}

	fn get_public_team_list(state: &AppState, meta: MetaRequestDto) -> Pin<Box<dyn Future<Output = Response> + Send>> {
		let state = state.to_owned();
		Box::pin(async move {
			let repo = TeamsRepository::new(&state);
			match repo.query_team_list(meta).await {
				Ok(mut data) => {
					// Calculate actual member count for each team
					for team in &mut data.data {
						let team_thing = make_thing_from_enum(ResourceEnum::Teams, &team.id);
						let members = repo.query_team_members(&team_thing).await.unwrap_or_default();
						team.current_member_count = members.len() as i32 + 1; // +1 for leader
					}

					let response = ResponseListSuccessDto {
						data: data.data,
						meta: data.meta,
					};
					success_list_response(response)
				}
				Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
			}
		})
	}

	fn get_public_team_by_id(state: &AppState, id: String) -> Pin<Box<dyn Future<Output = Response> + Send>> {
		let state = state.to_owned();
		Box::pin(async move {
			if id.trim().is_empty() {
				return common_response(StatusCode::BAD_REQUEST, "Invalid Team ID format");
			}
			let repo = TeamsRepository::new(&state);
			let thing_id = make_thing_from_enum(ResourceEnum::Teams, &id);
			match repo.query_team_by_id(&thing_id).await {
				Ok(team) if !team.is_deleted => {
					let team_thing = make_thing_from_enum(ResourceEnum::Teams, &team.id.id.to_raw());
						let members = repo.query_team_members(&team_thing).await.unwrap_or_default();
					let members_len = members.len();
					
					// For public team details, only show sensitive info if user is authenticated and part of the team
					let team_dto = PublicTeamsDetailItemDto {
						id: team.id.id.to_raw(),
						name: team.name,
						description: team.description,
						is_open: team.is_open,
						max_members: team.max_members,
						current_member_count: members_len as i32 + 1,
						skills_required: team.skills_required,
						location: team.location,
						avatar: team.avatar,
						website_url: team.website_url,
						github_url: team.github_url,
						is_active: team.is_active,
						created_at: team.created_at,
						updated_at: team.updated_at,
					};
					success_response(ResponseSuccessDto { data: team_dto })
				}
				Ok(_) => common_response(StatusCode::NOT_FOUND, "Team not found"),
				Err(e) => common_response(StatusCode::NOT_FOUND, &e.to_string()),
			}
		})
	}

	fn create_team(
		state: &AppState,
		claims: imphnen_libs::jsonwebtoken::Claims,
		new_team: TeamsCreateRequestDto,
	) -> Pin<Box<dyn Future<Output = Response> + Send>> {
		let state = state.to_owned();
		Box::pin(async move {
			if let Err((status, message)) = validate_request(&new_team) {
				return common_response(status, &message);
			}

			let repo = TeamsRepository::new(&state);
			let users_repo = UsersRepository::new(&state);
			
			let team_schema = TeamsSchema::create(new_team.clone(), claims.user_id.clone());
			
			match repo.query_create_team(team_schema.clone()).await {
				Ok(_) => {
					let leader_member = TeamMembersSchema::create(
						team_schema.id.id.to_raw(),
						claims.user_id.clone(),
						Some("leader".to_string()),
					);
					
					if let Err(e) = repo.query_add_team_member(leader_member).await {
						error!("Failed to add team leader as member: {}", e);
					}

					let mut successful_invites = Vec::new();
					let mut failed_invites = Vec::new();

					for email in new_team.member_emails {
						let existing_user = users_repo.query_user_by_email(email.clone()).await.ok();
						let is_existing_user = existing_user.is_some();
						
						let token = Self::generate_invitation_token().await;
						let invitation = TeamInvitationsSchema::create(
							team_schema.id.id.to_raw(),
							email.clone(),
							claims.user_id.clone(),
							token.clone(),
						);

						match repo.query_create_invitation(invitation).await {
							Ok(_) => {
								let inviter_user = match users_repo.query_user_by_id(&make_thing_from_enum(ResourceEnum::Users, &claims.user_id)).await {
									Ok(user) => user,
									Err(_) => continue,
								};
								if let Err(e) = Self::send_invitation_email(
									&team_schema.name,
									&inviter_user.fullname,
									&email,
									&token,
									is_existing_user,
								).await {
									error!("Failed to send invitation email to {}: {}", email, e);
									failed_invites.push(email);
								} else {
									successful_invites.push(email);
								}
							}
							Err(e) => {
								error!("Failed to create invitation for {}: {}", email, e);
								failed_invites.push(email);
							}
						}
					}

					let response_data = json!({
						"team_id": team_schema.id.id.to_raw(),
						"message": "Team created successfully",
						"invitations_sent": successful_invites.len(),
						"invitations_failed": failed_invites.len(),
						"failed_emails": failed_invites
					});

					imphnen_utils::success_created_response(ResponseSuccessDto { data: response_data })
				}
				Err(err) => {
					error!("Failed to create team: {}", err);
					common_response(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string())
				}
			}
		})
	}

	fn update_team(
		state: &AppState,
		claims: imphnen_libs::jsonwebtoken::Claims,
		id: String,
		team: TeamsUpdateRequestDto,
	) -> Pin<Box<dyn Future<Output = Response> + Send>> {
		let state = state.to_owned();
		Box::pin(async move {
			if id.trim().is_empty() {
				return common_response(StatusCode::BAD_REQUEST, "Invalid Team ID format");
			}

			if let Err((status, message)) = validate_request(&team) {
				return common_response(status, &message);
			}

			let repo = TeamsRepository::new(&state);
			let thing_id = make_thing_from_enum(ResourceEnum::Teams, &id);
			
			let current_team = match repo.query_team_by_id(&thing_id).await {
				Ok(team) => team,
				Err(_) => return common_response(StatusCode::NOT_FOUND, "Team not found"),
			};

			// Allow update if requester is leader
			if current_team.leader_id.id.to_raw() != claims.user_id {
				// Not leader; deny here (admin endpoints should use update_team_admin)
				return common_response(StatusCode::FORBIDDEN, "Only team leader can update team");
			}

			let updated_team = TeamsSchema {
				id: current_team.id,
				leader_id: current_team.leader_id,
				is_active: current_team.is_active,
				is_deleted: current_team.is_deleted,
				created_at: current_team.created_at,
				..TeamsSchema::default()
			}.update(team);

			match repo.query_update_team(updated_team).await {
				Ok(msg) => common_response(StatusCode::OK, &msg),
				Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
			}
		})
	}

	fn update_team_admin(
		state: &AppState,
		_claims: imphnen_libs::jsonwebtoken::Claims,
		id: String,
		team: TeamsUpdateRequestDto,
	) -> Pin<Box<dyn Future<Output = Response> + Send>> {
		let state = state.to_owned();
		Box::pin(async move {
			if id.trim().is_empty() {
				return common_response(StatusCode::BAD_REQUEST, "Invalid Team ID format");
			}

			if let Err((status, message)) = validate_request(&team) {
				return common_response(status, &message);
			}

			let repo = TeamsRepository::new(&state);
			let thing_id = make_thing_from_enum(ResourceEnum::Teams, &id);
			let current_team = match repo.query_team_by_id(&thing_id).await {
				Ok(team) => team,
				Err(_) => return common_response(StatusCode::NOT_FOUND, "Team not found"),
			};

			let updated_team = TeamsSchema {
				id: current_team.id,
				leader_id: current_team.leader_id,
				is_active: current_team.is_active,
				is_deleted: current_team.is_deleted,
				created_at: current_team.created_at,
				..TeamsSchema::default()
			}.update(team);

			match repo.query_update_team(updated_team).await {
				Ok(msg) => common_response(StatusCode::OK, &msg),
				Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
			}
		})
	}

	fn delete_team(
		state: &AppState,
		claims: imphnen_libs::jsonwebtoken::Claims,
		id: String,
	) -> Pin<Box<dyn Future<Output = Response> + Send>> {
		let state = state.to_owned();
		Box::pin(async move {
			if id.trim().is_empty() {
				return common_response(StatusCode::BAD_REQUEST, "Invalid Team ID format");
			}

			let repo = TeamsRepository::new(&state);
			let thing_id = make_thing_from_enum(ResourceEnum::Teams, &id);
			
			let team = match repo.query_team_by_id(&thing_id).await {
				Ok(team) => team,
				Err(_) => return common_response(StatusCode::NOT_FOUND, "Team not found"),
			};

			if team.leader_id.id.to_raw() != claims.user_id {
				return common_response(StatusCode::FORBIDDEN, "Only team leader can delete team");
			}

			match repo.query_delete_team(id).await {
				Ok(msg) => common_response(StatusCode::OK, &msg),
				Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
			}
		})
	}

	fn delete_team_admin(
		state: &AppState,
		_claims: imphnen_libs::jsonwebtoken::Claims,
		id: String,
	) -> Pin<Box<dyn Future<Output = Response> + Send>> {
		let state = state.to_owned();
		Box::pin(async move {
			if id.trim().is_empty() {
				return common_response(StatusCode::BAD_REQUEST, "Invalid Team ID format");
			}

			let repo = TeamsRepository::new(&state);
			let thing_id = make_thing_from_enum(ResourceEnum::Teams, &id);
			let _team = match repo.query_team_by_id(&thing_id).await {
				Ok(team) => team,
				Err(_) => return common_response(StatusCode::NOT_FOUND, "Team not found"),
			};

			match repo.query_delete_team(id).await {
				Ok(msg) => common_response(StatusCode::OK, &msg),
				Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
			}
		})
	}

	fn invite_team_members(
		state: &AppState,
		claims: imphnen_libs::jsonwebtoken::Claims,
		team_id: String,
		invite: TeamInviteRequestDto,
	) -> Pin<Box<dyn Future<Output = Response> + Send>> {
		let state = state.to_owned();
		Box::pin(async move {
			if team_id.trim().is_empty() {
				return common_response(StatusCode::BAD_REQUEST, "Invalid Team ID format");
			}

			if let Err((status, message)) = validate_request(&invite) {
				return common_response(status, &message);
			}

			let repo = TeamsRepository::new(&state);
			let users_repo = UsersRepository::new(&state);
			let thing_id = make_thing_from_enum(ResourceEnum::Teams, &team_id);
			
			let team = match repo.query_team_by_id(&thing_id).await {
				Ok(team) => team,
				Err(_) => return common_response(StatusCode::NOT_FOUND, "Team not found"),
			};

			let user_thing = make_thing_from_enum(ResourceEnum::Users, &claims.user_id);
			let is_member = repo.query_is_team_member(&thing_id, &user_thing).await.unwrap_or(false);
			let is_leader = team.leader_id.id.to_raw() == claims.user_id;

			if !is_member && !is_leader {
				return common_response(StatusCode::FORBIDDEN, "Only team members can invite others");
			}

			let mut successful_invites = Vec::new();
			let mut failed_invites = Vec::new();

			for email in invite.member_emails {
				let existing_user = users_repo.query_user_by_email(email.clone()).await.ok();
				let is_existing_user = existing_user.is_some();
				
				let token = Self::generate_invitation_token().await;
				let invitation = TeamInvitationsSchema::create(
					team_id.clone(),
					email.clone(),
					claims.user_id.clone(),
					token.clone(),
				);

				match repo.query_create_invitation(invitation).await {
					Ok(_) => {
						let inviter_user = match users_repo.query_user_by_id(&make_thing_from_enum(ResourceEnum::Users, &claims.user_id)).await {
							Ok(user) => user,
							Err(_) => return common_response(StatusCode::INTERNAL_SERVER_ERROR, "Failed to get inviter user information"),
						};
						if let Err(e) = Self::send_invitation_email(
							&team.name,
							&inviter_user.fullname,
							&email,
							&token,
							is_existing_user,
						).await {
							error!("Failed to send invitation email to {}: {}", email, e);
							failed_invites.push(email);
						} else {
							successful_invites.push(email);
						}
					}
					Err(e) => {
						error!("Failed to create invitation for {}: {}", email, e);
						failed_invites.push(email);
					}
				}
			}

			let response_data = json!({
				"invitations_sent": successful_invites.len(),
				"invitations_failed": failed_invites.len(),
				"failed_emails": failed_invites
			});

			success_response(ResponseSuccessDto { data: response_data })
		})
	}

	fn invite_team_members_admin(
		state: &AppState,
		claims: imphnen_libs::jsonwebtoken::Claims,
		team_id: String,
		invite: TeamInviteRequestDto,
	) -> Pin<Box<dyn Future<Output = Response> + Send>> {
		let state = state.to_owned();
		Box::pin(async move {
			if team_id.trim().is_empty() {
				return common_response(StatusCode::BAD_REQUEST, "Invalid Team ID format");
			}

			if let Err((status, message)) = validate_request(&invite) {
				return common_response(status, &message);
			}

			let repo = TeamsRepository::new(&state);
			let users_repo = UsersRepository::new(&state);
			let thing_id = make_thing_from_enum(ResourceEnum::Teams, &team_id);
			let team = match repo.query_team_by_id(&thing_id).await {
				Ok(team) => team,
				Err(_) => return common_response(StatusCode::NOT_FOUND, "Team not found"),
			};

			let mut successful_invites = Vec::new();
			let mut failed_invites = Vec::new();

			for email in invite.member_emails {
				let existing_user = users_repo.query_user_by_email(email.clone()).await.ok();
				let is_existing_user = existing_user.is_some();
				let token = Self::generate_invitation_token().await;
				let invitation = TeamInvitationsSchema::create(
					team_id.clone(),
					email.clone(),
					claims.user_id.clone(),
					token.clone(),
				);

				match repo.query_create_invitation(invitation).await {
					Ok(_) => {
						let inviter_user = match users_repo.query_user_by_id(&make_thing_from_enum(ResourceEnum::Users, &claims.user_id)).await {
							Ok(user) => user,
							Err(_) => return common_response(StatusCode::INTERNAL_SERVER_ERROR, "Failed to get inviter user information"),
						};
						if let Err(e) = Self::send_invitation_email(
							&team.name,
							&inviter_user.fullname,
							&email,
							&token,
							is_existing_user,
						).await {
							error!("Failed to send invitation email to {}: {}", email, e);
							failed_invites.push(email);
						} else {
							successful_invites.push(email);
						}
					}
					Err(e) => {
						error!("Failed to create invitation for {}: {}", email, e);
						failed_invites.push(email);
					}
				}
			}

			let response_data = json!({
				"invitations_sent": successful_invites.len(),
				"invitations_failed": failed_invites.len(),
				"failed_emails": failed_invites
			});

			success_response(ResponseSuccessDto { data: response_data })
		})
	}

	fn accept_invitation(
		state: &AppState,
		claims: imphnen_libs::jsonwebtoken::Claims,
		accept: TeamAcceptInvitationRequestDto,
	) -> Pin<Box<dyn Future<Output = Response> + Send>> {
		let state = state.to_owned();
		Box::pin(async move {
			let repo = TeamsRepository::new(&state);
			let users_repo = UsersRepository::new(&state);

			let invitation = match repo.query_invitation_by_token(&accept.token).await {
				Ok(inv) => inv,
				Err(_) => return common_response(StatusCode::NOT_FOUND, "Invalid or expired invitation"),
			};

			if invitation.status != "pending" {
				return common_response(StatusCode::BAD_REQUEST, "Invitation already processed");
			}

			if Utc::now().timestamp() > invitation.expires_at.parse::<i64>().unwrap_or(0) {
				return common_response(StatusCode::BAD_REQUEST, "Invitation has expired");
			}

			let user = match users_repo.query_user_by_id(&make_thing_from_enum(ResourceEnum::Users, &claims.user_id)).await {
				Ok(user) => user,
				Err(_) => return common_response(StatusCode::NOT_FOUND, "User not found"),
			};

			if user.email != invitation.email {
				return common_response(StatusCode::FORBIDDEN, "Invitation email does not match user email");
			}

			let user_thing = make_thing_from_enum(ResourceEnum::Users, &claims.user_id);
			let team_thing = make_thing_from_enum(ResourceEnum::Teams, &invitation.team_id.id.to_raw());
			let is_already_member = repo.query_is_team_member(&team_thing, &user_thing).await.unwrap_or(false);

			if is_already_member {
				return common_response(StatusCode::BAD_REQUEST, "User is already a team member");
			}

			let member_schema = TeamMembersSchema::create(
				invitation.team_id.id.to_raw(),
				claims.user_id,
				None,
			);

			match repo.query_add_team_member(member_schema).await {
				Ok(_) => {
					let updated_invitation = TeamInvitationsSchema {
						id: invitation.id,
						team_id: invitation.team_id,
						email: invitation.email,
						inviter_id: invitation.inviter_id,
						invite_code: invitation.invite_code,
						expires_at: chrono::DateTime::parse_from_rfc3339(&invitation.expires_at)
							.unwrap_or_default()
							.with_timezone(&Utc),
						status: "accepted".to_string(),
						invited_at: invitation.invited_at,
						accepted_at: Some(chrono::Utc::now().to_rfc3339()),
					};

					if let Err(e) = repo.query_update_invitation(updated_invitation).await {
						error!("Failed to update invitation status: {}", e);
					}

					common_response(StatusCode::OK, "Successfully joined the team")
				}
				Err(e) => {
					error!("Failed to add team member: {}", e);
					common_response(StatusCode::INTERNAL_SERVER_ERROR, "Failed to join team")
				}
			}
		})
	}

	fn get_team_members(
		state: &AppState,
		claims: imphnen_libs::jsonwebtoken::Claims,
		team_id: String,
	) -> Pin<Box<dyn Future<Output = Response> + Send>> {
		let state = state.to_owned();
		Box::pin(async move {
			if team_id.trim().is_empty() {
				return common_response(StatusCode::BAD_REQUEST, "Invalid Team ID format");
			}

			let repo = TeamsRepository::new(&state);
			let thing_id = make_thing_from_enum(ResourceEnum::Teams, &team_id);
			let user_thing = make_thing_from_enum(ResourceEnum::Users, &claims.user_id);
			
			let team = match repo.query_team_by_id(&thing_id).await {
				Ok(team) => team,
				Err(_) => return common_response(StatusCode::NOT_FOUND, "Team not found"),
			};

			let is_member = repo.query_is_team_member(&thing_id, &user_thing).await.unwrap_or(false);

			let members = match repo.query_team_members(&thing_id).await {
				Ok(members) => members,
				Err(e) => return common_response(StatusCode::BAD_REQUEST, &e.to_string()),
			};

			let mut member_dtos = Vec::new();
			for member in members {
				match Self::get_user_info_with_privacy(
					&member.user_id.id.to_raw(),
					&claims.user_id,
					is_member,
					&state,
				).await {
					Ok(mut member_dto) => {
						member_dto.role = member.role;
						member_dto.joined_at = member.joined_at;
						member_dtos.push(member_dto);
					}
					Err(_) => continue,
				}
			}

			if let Ok(mut leader_dto) = Self::get_user_info_with_privacy(
				&team.leader_id.id.to_raw(),
				&claims.user_id,
				is_member,
				&state,
			).await {
				leader_dto.role = "leader".to_string();
				member_dtos.insert(0, leader_dto);
			}

			success_response(ResponseSuccessDto { data: member_dtos })
		})
	}

	fn leave_team(
		state: &AppState,
		claims: imphnen_libs::jsonwebtoken::Claims,
		team_id: String,
	) -> Pin<Box<dyn Future<Output = Response> + Send>> {
		let state = state.to_owned();
		Box::pin(async move {
			if team_id.trim().is_empty() {
				return common_response(StatusCode::BAD_REQUEST, "Invalid Team ID format");
			}

			let repo = TeamsRepository::new(&state);
			let thing_id = make_thing_from_enum(ResourceEnum::Teams, &team_id);
			let user_thing = make_thing_from_enum(ResourceEnum::Users, &claims.user_id);
			
			let team = match repo.query_team_by_id(&thing_id).await {
				Ok(team) => team,
				Err(_) => return common_response(StatusCode::NOT_FOUND, "Team not found"),
			};

			if team.leader_id.id.to_raw() == claims.user_id {
				return common_response(StatusCode::BAD_REQUEST, "Team leader cannot leave the team");
			}

			let is_member = repo.query_is_team_member(&thing_id, &user_thing).await.unwrap_or(false);
			if !is_member {
				return common_response(StatusCode::BAD_REQUEST, "User is not a team member");
			}

			match repo.query_remove_team_member(&thing_id, &user_thing).await {
				Ok(msg) => common_response(StatusCode::OK, &msg),
				Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
			}
		})
	}

	fn search_teams(
		state: &AppState,
		search_params: TeamsSearchQueryDto,
	) -> Pin<Box<dyn Future<Output = Response> + Send>> {
		let state = state.to_owned();
		Box::pin(async move {
			let repo = TeamsRepository::new(&state);
			match repo.query_search_teams(search_params).await {
				Ok(data) => {
					let response = ResponseListSuccessDto {
						data: data.data,
						meta: data.meta,
					};
					success_list_response(response)
				}
				Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
			}
		})
	}

	fn get_admin_team_list(state: &AppState, meta: MetaRequestDto) -> Pin<Box<dyn Future<Output = Response> + Send>> {
			let state = state.to_owned();
			Box::pin(async move {
				let repo = TeamsRepository::new(&state);
				match repo.query_team_list(meta).await {
					Ok(data) => {
						let response = ResponseListSuccessDto {
													data: data.data.into_iter().map(|team| team.into_list_item_dto().into_admin_list_dto()).collect::<Vec<AdminTeamsListItemDto>>(),
													meta: data.meta,
												};
						success_list_response(response)
					}
					Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
				}
			})
		}

	fn get_admin_team_by_id(state: &AppState, id: String) -> Pin<Box<dyn Future<Output = Response> + Send>> {
		let state = state.to_owned();
		Box::pin(async move {
			if id.trim().is_empty() {
				return common_response(StatusCode::BAD_REQUEST, "Invalid Team ID format");
			}
			let repo = TeamsRepository::new(&state);
			let thing_id = make_thing_from_enum(ResourceEnum::Teams, &id);
			match repo.query_team_by_id(&thing_id).await {
				Ok(team) if !team.is_deleted => {
					let team_thing = make_thing_from_enum(ResourceEnum::Teams, &team.id.id.to_raw());
						let members = repo.query_team_members(&team_thing).await.unwrap_or_default();
					let mut member_dtos = Vec::new();
					
					for member in members {
						match Self::get_user_info_with_privacy(
							&member.user_id.id.to_raw(),
							"system",  // Admin context - show all sensitive data
							true,       // Admin context - always show sensitive data
							&state,
						).await {
							Ok(mut member_dto) => {
								member_dto.role = member.role;
								member_dto.joined_at = member.joined_at;
								member_dtos.push(member_dto);
							}
							Err(_) => continue,
						}
					}

					// Add leader with full sensitive info
					if let Ok(mut leader_dto) = Self::get_user_info_with_privacy(
						&team.leader_id.id.to_raw(),
						"system",
						true,
						&state,
					).await {
						leader_dto.role = "leader".to_string();
						member_dtos.insert(0, leader_dto);
					}
					
					let team_dto = team.into_admin_detail_dto(member_dtos);
					success_response(ResponseSuccessDto { data: team_dto })
				}
				Ok(_) => common_response(StatusCode::NOT_FOUND, "Team not found"),
				Err(e) => common_response(StatusCode::NOT_FOUND, &e.to_string()),
			}
		})
	}

	fn get_admin_team_members(state: &AppState, team_id: String) -> Pin<Box<dyn Future<Output = Response> + Send>> {
		let state = state.to_owned();
		Box::pin(async move {
			if team_id.trim().is_empty() {
				return common_response(StatusCode::BAD_REQUEST, "Invalid Team ID format");
			}
			let repo = TeamsRepository::new(&state);
			let thing_id = make_thing_from_enum(ResourceEnum::Teams, &team_id);
			
			let members = match repo.query_team_members(&thing_id).await {
				Ok(members) => members,
				Err(e) => return common_response(StatusCode::BAD_REQUEST, &e.to_string()),
			};

			let mut member_dtos = Vec::new();
			for member in members {
				match Self::get_user_info_with_privacy(
					&member.user_id.id.to_raw(),
					"system",  // Admin context - show all sensitive data
					true,       // Admin context - always show sensitive data
					&state,
				).await {
					Ok(mut member_dto) => {
						member_dto.role = member.role;
						member_dto.joined_at = member.joined_at;
						member_dtos.push(member_dto);
					}
					Err(_) => continue,
				}
			}

			success_response(ResponseSuccessDto { data: member_dtos })
		})
	}
	
	fn leave_current_team(state: &AppState, claims: imphnen_libs::jsonwebtoken::Claims) -> Pin<Box<dyn Future<Output = Response> + Send>> {
		let state = state.to_owned();
		Box::pin(async move {
			let repo = TeamsRepository::new(&state);
			let user_thing = make_thing_from_enum(ResourceEnum::Users, &claims.user_id);
			
			// Find the teams that the user is a member of
			let teams = match repo.query_teams_by_user(&user_thing).await {
				Ok(teams) => teams,
				Err(e) => {
					error!("Failed to query user teams: {}", e);
					return common_response(StatusCode::INTERNAL_SERVER_ERROR, "Failed to retrieve user teams")
				},
			};

			if teams.is_empty() {
				return common_response(StatusCode::BAD_REQUEST, "User is not a member of any team");
			}

			// For now, assume user is in only one team (common case)
			// In a future enhancement, we could ask the user to specify which team to leave
			let team = &teams[0];
			let team_id = team.id.id.to_raw();
			let team_thing = make_thing_from_enum(ResourceEnum::Teams, &team_id);

			// Check if user is the leader
			if team.leader_id.id.to_raw() == claims.user_id {
				return common_response(StatusCode::FORBIDDEN, "Team leader cannot leave the team");
			}

			match repo.query_remove_team_member(&team_thing, &user_thing).await {
				Ok(_) => common_response(StatusCode::OK, &format!("Successfully left team: {}", team.name)),
				Err(e) => {
					error!("Failed to remove team member: {}", e);
					common_response(StatusCode::INTERNAL_SERVER_ERROR, "Failed to leave team")
				},
			}
		})
	}

	fn get_my_team(state: &AppState, claims: imphnen_libs::jsonwebtoken::Claims) -> Pin<Box<dyn Future<Output = Response> + Send>> {
		let state = state.to_owned();
		Box::pin(async move {
			let repo = TeamsRepository::new(&state);
			let user_thing = make_thing_from_enum(ResourceEnum::Users, &claims.user_id);
			
			// Find the teams that the user is a member of
			let teams = match repo.query_teams_by_user(&user_thing).await {
				Ok(teams) => teams,
				Err(e) => {
					error!("Failed to query user teams: {}", e);
					return common_response(StatusCode::INTERNAL_SERVER_ERROR, "Failed to retrieve user teams")
				},
			};

			if teams.is_empty() {
				return common_response(StatusCode::NOT_FOUND, "User is not a member of any team");
			}

			// Return the first team (most common case - user is in one team)
			let team = &teams[0];
			let team_id = team.id.id.to_raw();
			
			// Get full team details
			Self::get_public_team_by_id(&state, team_id).await
		})
	}

	fn get_team_invitations(state: &AppState, claims: imphnen_libs::jsonwebtoken::Claims, team_id: String) -> Pin<Box<dyn Future<Output = Response> + Send>> {
		let state = state.to_owned();
		Box::pin(async move {
			let repo = TeamsRepository::new(&state);
			let team_thing = make_thing_from_enum(ResourceEnum::Teams, &team_id);
			
			// Check if team exists and user is leader
			let team = match repo.query_team_by_id(&team_thing).await {
				Ok(t) => t,
				Err(_) => return common_response(StatusCode::NOT_FOUND, "Team not found"),
			};

			if team.leader_id.id.to_raw() != claims.user_id {
				return common_response(StatusCode::FORBIDDEN, "Only team leader can view invitations");
			}

			// Get invitations
			match repo.query_team_invitations(&team_thing).await {
				Ok(invitations) => {
					use crate::{v1::teams::TeamInvitationListDto, UsersRepository};
					let users_repo = UsersRepository::new(&state);
					
					let mut invitation_list = Vec::new();
					for inv in invitations {
						// Get inviter name
						let inviter_name = match users_repo.query_user_by_id(&inv.inviter_id).await {
							Ok(user) => user.fullname,
							Err(_) => "Unknown".to_string(),
						};

						invitation_list.push(TeamInvitationListDto {
							id: inv.id.id.to_raw(),
							team_id: inv.team_id.id.to_raw(),
							team_name: team.name.clone(),
							email: inv.email,
							inviter_id: inv.inviter_id.id.to_raw(),
							inviter_name,
							status: inv.status,
							invite_code: inv.invite_code,
							expires_at: inv.expires_at,
							invited_at: inv.invited_at,
						});
					}

					success_response(ResponseSuccessDto { data: invitation_list })
				},
				Err(e) => {
					error!("Failed to get team invitations: {}", e);
					common_response(StatusCode::INTERNAL_SERVER_ERROR, "Failed to retrieve invitations")
				}
			}
		})
	}

	fn cancel_invitation(state: &AppState, claims: imphnen_libs::jsonwebtoken::Claims, token: String) -> Pin<Box<dyn Future<Output = Response> + Send>> {
		let state = state.to_owned();
		Box::pin(async move {
			let repo = TeamsRepository::new(&state);
			
			// Get invitation to check ownership
			let invitation = match repo.query_invitation_by_token(&token).await {
				Ok(inv) => inv,
				Err(_) => return common_response(StatusCode::NOT_FOUND, "Invitation not found"),
			};

			// Check if user is the team leader
			let team_thing = invitation.team_id.clone();
			let team = match repo.query_team_by_id(&team_thing).await {
				Ok(t) => t,
				Err(_) => return common_response(StatusCode::NOT_FOUND, "Team not found"),
			};

			if team.leader_id.id.to_raw() != claims.user_id {
				return common_response(StatusCode::FORBIDDEN, "Only team leader can cancel invitations");
			}

			match repo.query_delete_invitation(&token).await {
				Ok(msg) => success_response(ResponseSuccessDto { data: msg }),
				Err(e) => {
					error!("Failed to cancel invitation: {}", e);
					common_response(StatusCode::INTERNAL_SERVER_ERROR, "Failed to cancel invitation")
				}
			}
		})
	}

	fn get_my_invitations(state: &AppState, claims: imphnen_libs::jsonwebtoken::Claims) -> Pin<Box<dyn Future<Output = Response> + Send>> {
		let state = state.to_owned();
		Box::pin(async move {
			use crate::{v1::teams::MyInvitationDto, UsersRepository};
			let users_repo = UsersRepository::new(&state);
			
			// Get user email
			let user_thing = make_thing_from_enum(ResourceEnum::Users, &claims.user_id);
			let user = match users_repo.query_user_by_id(&user_thing).await {
				Ok(u) => u,
				Err(_) => return common_response(StatusCode::NOT_FOUND, "User not found"),
			};

			let repo = TeamsRepository::new(&state);
			match repo.query_user_invitations(&user.email).await {
				Ok(invitations) => {
					let mut my_invitations = Vec::new();
					for inv in invitations {
						// Get team details
						let team = match repo.query_team_by_id(&inv.team_id).await {
							Ok(t) => t,
							Err(_) => continue,
						};

						// Get inviter name
						let inviter_name = match users_repo.query_user_by_id(&inv.inviter_id).await {
							Ok(u) => u.fullname,
							Err(_) => "Unknown".to_string(),
						};

						my_invitations.push(MyInvitationDto {
							id: inv.id.id.to_raw(),
							team_id: inv.team_id.id.to_raw(),
							team_name: team.name,
							team_description: team.description,
							team_avatar: team.avatar,
							inviter_name,
							invite_code: inv.invite_code,
							status: inv.status,
							expires_at: inv.expires_at,
							invited_at: inv.invited_at,
						});
					}

					success_response(ResponseSuccessDto { data: my_invitations })
				},
				Err(e) => {
					error!("Failed to get user invitations: {}", e);
					common_response(StatusCode::INTERNAL_SERVER_ERROR, "Failed to retrieve invitations")
				}
			}
		})
	}
}
