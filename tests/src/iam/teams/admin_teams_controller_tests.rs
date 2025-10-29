use crate::get_app_state;
use axum::{http::HeaderMap, response::Response};
use imphnen_iam::{
    AppState, Claims, PermissionsEnum, ResponseSuccessDto, ResponseListSuccessDto,
    AdminTeamsListItemDto, AdminTeamsDetailItemDto, TeamMemberDto
};
use imphnen_libs::jsonwebtoken::{encode, Header};
use imphnen_utils::make_thing_from_enum;
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

#[tokio::test]
async fn test_admin_team_endpoints_sensitive_data_exposure() {
    let app_state = get_app_state().await;
    let repo = imphnen_iam::TeamsRepository::new(&app_state);
    
    // Create test data
    let team_id = Uuid::new_v4().to_string();
    let leader_id = Uuid::new_v4().to_string();
    let member_id_1 = Uuid::new_v4().to_string();
    let member_id_2 = Uuid::new_v4().to_string();
    
    // Create test team
    let team = imphnen_iam::TeamsSchema {
        id: make_thing_from_enum(imphnen_libs::ResourceEnum::Teams, &team_id),
        name: "Admin Test Team".to_string(),
        description: Some("Test team for admin endpoints".to_string()),
        leader_id: make_thing_from_enum(imphnen_libs::ResourceEnum::Users, &leader_id),
        is_open: true,
        max_members: Some(10),
        skills_required: Some(vec!["Rust".to_string(), "Backend".to_string()]),
        location: Some("Remote".to_string()),
        avatar: Some("https://example.com/avatar.jpg".to_string()),
        website_url: Some("https://example.com".to_string()),
        github_url: Some("https://github.com/example".to_string()),
        is_active: true,
        is_deleted: false,
        created_at: Utc::now().to_rfc3339(),
        updated_at: Utc::now().to_rfc3339(),
    };
    
    let create_result = repo.query_create_team(team.clone()).await;
    assert!(create_result.is_ok(), "Failed to create test team");

    // Create test members
    let member_1 = imphnen_iam::TeamMembersSchema {
        id: make_thing_from_enum(imphnen_libs::ResourceEnum::TeamMembers, &Uuid::new_v4().to_string()),
        team_id: make_thing_from_enum(imphnen_libs::ResourceEnum::Teams, &team_id),
        user_id: make_thing_from_enum(imphnen_libs::ResourceEnum::Users, &member_id_1),
        role: "member".to_string(),
        joined_at: Utc::now().to_rfc3339(),
        is_active: true,
    };
    
    let member_2 = imphnen_iam::TeamMembersSchema {
        id: make_thing_from_enum(imphnen_libs::ResourceEnum::TeamMembers, &Uuid::new_v4().to_string()),
        team_id: make_thing_from_enum(imphnen_libs::ResourceEnum::Teams, &team_id),
        user_id: make_thing_from_enum(imphnen_libs::ResourceEnum::Users, &member_id_2),
        role: "contributor".to_string(),
        joined_at: Utc::now().to_rfc3339(),
        is_active: true,
    };
    
    let add_member_result_1 = repo.query_add_team_member(member_1).await;
    let add_member_result_2 = repo.query_add_team_member(member_2).await;
    
    assert!(add_member_result_1.is_ok(), "Failed to add test member 1");
    assert!(add_member_result_2.is_ok(), "Failed to add test member 2");

    // Create admin user with proper permissions
    let admin_claims = Claims {
        user_id: "admin_user_123".to_string(),
        email: "admin@example.com".to_string(),
        fullname: "Admin User".to_string(),
        avatar: None,
        role: imphnen_iam::RoleSchema {
            id: make_thing_from_enum(imphnen_libs::ResourceEnum::Roles, "admin_role"),
            name: "Admin".to_string(),
            permissions: vec![
                imphnen_iam::PermissionSchema {
                    id: make_thing_from_enum(imphnen_libs::ResourceEnum::Permissions, "read_list_teams"),
                    name: PermissionsEnum::ReadListTeams.to_string(),
                },
                imphnen_iam::PermissionSchema {
                    id: make_thing_from_enum(imphnen_libs::ResourceEnum::Permissions, "read_detail_teams"),
                    name: PermissionsEnum::ReadDetailTeams.to_string(),
                },
            ],
        },
        exp: 1_000_000_000,
        iat: 0,
    };

    let admin_token = encode(&Header::default(), &admin_claims, &app_state.jwt_secret).unwrap();
    
    let mut headers = HeaderMap::new();
    headers.insert("Authorization", format!("Bearer {}", admin_token).parse().unwrap());

    // Test 1: Admin team list endpoint should expose sensitive fields
    let response = imphnen_iam::teams_controller::get_admin_team_list(
        headers.clone(),
        axum::extract::Extension(app_state.clone()),
        axum::extract::Query(imphnen_iam::MetaRequestDto {
            page: Some(1),
            per_page: Some(10),
            search: None,
            sort_by: None,
            order: None,
            filter: None,
            filter_by: None,
        }),
    ).await;

    assert!(response.status().is_success(), "Admin team list should return success");
    
    let v = crate::common::response_helpers::parse_response_value(response, 8192).await;
    let response_json: ResponseListSuccessDto<Vec<AdminTeamsListItemDto>> =
        serde_json::from_value(v).unwrap();
    
    // Verify ALL fields are present and not empty in admin response (AdminTeamsListItemDto)
    assert!(response_json.data.iter().any(|team| {
        !team.id.is_empty() &&               // Required: non-empty id
        !team.name.is_empty() &&             // Required: non-empty name
        team.description.is_some() &&        // Required: description field exists
        team.leader.is_some() &&             // Required: leader field exists
        team.leader.as_ref().map_or(false, |l| !l.id.is_empty()) && // Leader has id
        team.leader.as_ref().map_or(false, |l| !l.user_id.is_empty()) && // Leader has user_id
        team.leader.as_ref().map_or(false, |l| !l.fullname.is_empty()) && // Leader has fullname
        team.leader.as_ref().map_or(false, |l| !l.role.is_empty()) && // Leader has role
        team.is_open != false &&             // Required: is_open field exists
        team.current_member_count >= 0 &&    // Required: current_member_count exists
        team.max_members.is_some() &&        // Required: max_members field exists
        team.skills_required.is_some() &&    // Required: skills_required field exists
        team.location.is_some() &&           // Required: location field exists
        team.avatar.is_some() &&             // Required: avatar field exists
        team.website_url.is_some() &&        // Required: website_url field exists
        team.github_url.is_some() &&         // Required: github_url field exists
        team.is_active != false &&           // Required: is_active field exists
        team.is_deleted != false &&          // Required: is_deleted field exists
        team.created_at.is_some()            // Required: created_at field exists
    }), "Admin team list should expose ALL required fields and sensitive data");

    // Test 2: Admin team detail endpoint should expose sensitive fields and full member info
    let response = imphnen_iam::teams_controller::get_admin_team_by_id(
        headers.clone(),
        axum::extract::Extension(app_state.clone()),
        axum::extract::Path(team_id.clone()),
    ).await;

    assert!(response.status().is_success(), "Admin team detail should return success");
    
    let v = crate::common::response_helpers::parse_response_value(response, 8192).await;
    let response_json: ResponseSuccessDto<AdminTeamsDetailItemDto> =
        serde_json::from_value(v).unwrap();
    let admin_team = response_json.data;
    
    // Verify ALL fields are present and not empty in admin team detail response (AdminTeamsDetailItemDto)
    assert!(!admin_team.id.is_empty(), "Admin team detail should have non-empty id");
    assert!(!admin_team.name.is_empty(), "Admin team detail should have non-empty name");
    assert!(admin_team.description.is_some(), "Admin team detail should have description field");
    assert!(admin_team.leader.is_some(), "Admin team detail should have leader field");
    
    // Validate leader object
    let leader = admin_team.leader.as_ref().unwrap();
    assert!(!leader.id.is_empty(), "Admin team leader should have non-empty id");
    assert!(!leader.user_id.is_empty(), "Admin team leader should have non-empty user_id");
    assert!(!leader.fullname.is_empty(), "Admin team leader should have non-empty fullname");
    assert!(leader.role.is_some(), "Admin team leader should have role field");
    assert!(leader.joined_at.is_some(), "Admin team leader should have joined_at field");
    
    assert!(admin_team.is_open != false, "Admin team detail should show is_open field");
    assert!(admin_team.current_member_count >= 0, "Admin team detail should show current_member_count");
    assert!(admin_team.max_members.is_some(), "Admin team detail should show max_members field");
    assert!(admin_team.skills_required.is_some(), "Admin team detail should show skills_required field");
    assert!(admin_team.location.is_some(), "Admin team detail should show location field");
    assert!(admin_team.avatar.is_some(), "Admin team detail should show avatar field");
    assert!(admin_team.website_url.is_some(), "Admin team detail should show website_url");
    assert!(admin_team.github_url.is_some(), "Admin team detail should show github_url");
    assert!(admin_team.members.len() >= 2, "Admin team detail should show all members");
    assert!(admin_team.is_active != false, "Admin team detail should show is_active field");
    assert!(admin_team.is_deleted != false, "Admin team detail should show is_deleted field");
    assert!(admin_team.created_at.is_some(), "Admin team detail should show created_at field");
    assert!(admin_team.updated_at.is_some(), "Admin team detail should show updated_at field");
    
    // Verify ALL member fields are present and not empty (TeamMemberDto)
    let has_all_member_info = admin_team.members.iter().all(|member| {
        !member.id.is_empty() &&               // Required: non-empty id
        !member.user_id.is_empty() &&          // Required: non-empty user_id
        !member.fullname.is_empty() &&         // Required: non-empty fullname
        member.email.is_some() &&              // Required: email field exists (admin can see emails)
        !member.role.is_empty() &&             // Required: non-empty role
        member.skills.is_some() &&             // Required: skills field exists
        member.joined_at.is_some() &&          // Required: joined_at field exists
        member.avatar.is_some()                // Required: avatar field exists
    });
    
    assert!(has_all_member_info, "Admin team detail should expose all member sensitive information");

    // Test 3: Admin team members endpoint should expose sensitive info
    let response = imphnen_iam::teams_controller::get_admin_team_members(
        headers,
        axum::extract::Extension(app_state),
        axum::extract::Path(team_id),
    ).await;

    assert!(response.status().is_success(), "Admin team members should return success");
    
    let v = crate::common::response_helpers::parse_response_value(response, 8192).await;
    let response_json: ResponseSuccessDto<Vec<TeamMemberDto>> =
        serde_json::from_value(v).unwrap();
    let admin_members = response_json.data;
    
    // Verify ALL member fields are present and not empty (TeamMemberDto)
    let has_all_member_info = admin_members.iter().all(|member| {
        !member.id.is_empty() &&               // Required: non-empty id
        !member.user_id.is_empty() &&          // Required: non-empty user_id
        !member.fullname.is_empty() &&         // Required: non-empty fullname
        member.email.is_some() &&              // Required: email field exists (admin can see emails)
        !member.role.is_empty() &&             // Required: non-empty role
        member.skills.is_some() &&             // Required: skills field exists
        member.joined_at.is_some() &&          // Required: joined_at field exists
        member.avatar.is_some()                // Required: avatar field exists
    });
    
    assert!(has_all_member_info, "Admin team members endpoint should expose all member sensitive information");

    // Clean up
    let _ = repo.query_delete_team(team_id).await;
}

#[tokio::test]
async fn test_admin_team_endpoints_permission_guard() {
    let app_state = get_app_state().await;
    
    // Create test team first
    let team_id = Uuid::new_v4().to_string();
    let leader_id = Uuid::new_v4().to_string();
    
    let team = imphnen_iam::TeamsSchema {
        id: make_thing_from_enum(imphnen_libs::ResourceEnum::Teams, &team_id),
        name: "Permission Test Team".to_string(),
        description: Some("Test team for permission checks".to_string()),
        leader_id: make_thing_from_enum(imphnen_libs::ResourceEnum::Users, &leader_id),
        is_open: true,
        max_members: Some(10),
        skills_required: Some(vec!["Rust".to_string()]),
        location: Some("Remote".to_string()),
        avatar: None,
        website_url: None,
        github_url: None,
        is_active: true,
        is_deleted: false,
        created_at: Utc::now().to_rfc3339(),
        updated_at: Utc::now().to_rfc3339(),
    };
    
    let repo = imphnen_iam::TeamsRepository::new(&app_state);
    let create_result = repo.query_create_team(team).await;
    assert!(create_result.is_ok(), "Failed to create test team");

    // Create regular user without admin permissions
    let regular_claims = Claims {
        user_id: "regular_user_123".to_string(),
        email: "user@example.com".to_string(),
        fullname: "Regular User".to_string(),
        avatar: None,
        role: imphnen_iam::RoleSchema {
            id: make_thing_from_enum(imphnen_libs::ResourceEnum::Roles, "user_role"),
            name: "User".to_string(),
            permissions: vec![], // No admin permissions
        },
        exp: 1_000_000_000,
        iat: 0,
    };

    let regular_token = encode(&Header::default(), &regular_claims, &app_state.jwt_secret).unwrap();
    
    let mut headers = HeaderMap::new();
    headers.insert("Authorization", format!("Bearer {}", regular_token).parse().unwrap());

    // Test that regular user gets forbidden for admin endpoints
    let response = imphnen_iam::teams_controller::get_admin_team_list(
        headers,
        axum::extract::Extension(app_state),
        axum::extract::Query(imphnen_iam::MetaRequestDto {
            page: Some(1),
            per_page: Some(10),
            search: None,
            sort_by: None,
            order: None,
            filter: None,
            filter_by: None,
        }),
    ).await;

    assert_eq!(response.status().as_u16(), 403, "Regular user should get forbidden for admin endpoints");
    // Also assert response body contains a permission/forbidden message
    let v = crate::common::response_helpers::parse_response_value(response, 1024).await;
    let msg = v.get("message").and_then(|m| m.as_str()).unwrap_or("");
    let msg_l = msg.to_lowercase();
    assert!(msg_l.contains("forbidden") || msg_l.contains("permission") || msg_l.contains("not authorized") || msg_l.contains("unauthorized"),
        "permission guard response should include a forbidden/permission message");

    // Clean up
    let _ = repo.query_delete_team(team_id).await;
}

#[tokio::test]
async fn test_admin_team_dto_conversion_edge_cases() {
    // Test DTO conversion with empty member list
    let team_query_dto = imphnen_iam::TeamsDetailQueryDto {
        id: make_thing_from_enum(imphnen_libs::ResourceEnum::Teams, &Uuid::new_v4().to_string()),
        name: "Test Team".to_string(),
        description: Some("Test description".to_string()),
        leader_id: make_thing_from_enum(imphnen_libs::ResourceEnum::Users, &Uuid::new_v4().to_string()),
        is_open: true,
        max_members: Some(10),
        skills_required: Some(vec!["Rust".to_string()]),
        location: Some("Remote".to_string()),
        avatar: Some("https://example.com/avatar.jpg".to_string()),
        website_url: Some("https://example.com".to_string()),
        github_url: Some("https://github.com/example".to_string()),
        is_active: true,
        is_deleted: false,
        created_at: Utc::now().to_rfc3339(),
        updated_at: Utc::now().to_rfc3339(),
    };
    
    let admin_dto = team_query_dto.to_admin_detail_dto(vec![]); // Empty member list
    
    // Should handle empty member list gracefully
    assert_eq!(admin_dto.members.len(), 0, "Should handle empty member list");
    assert_eq!(admin_dto.current_member_count, 1, "Current member count should be 1 (leader only)");
    assert_eq!(admin_dto.is_deleted, false, "Should preserve is_deleted field");
    assert_eq!(admin_dto.is_active, true, "Should preserve is_active field");
    assert!(admin_dto.website_url.is_some(), "Should preserve website_url");
    assert!(admin_dto.github_url.is_some(), "Should preserve github_url");

    // Test DTO conversion with deleted team
    let deleted_team_query_dto = imphnen_iam::TeamsDetailQueryDto {
        id: make_thing_from_enum(imphnen_libs::ResourceEnum::Teams, &Uuid::new_v4().to_string()),
        name: "Deleted Team".to_string(),
        description: Some("This team is deleted".to_string()),
        leader_id: make_thing_from_enum(imphnen_libs::ResourceEnum::Users, &Uuid::new_v4().to_string()),
        is_open: true,
        max_members: Some(10),
        skills_required: Some(vec!["Rust".to_string()]),
        location: Some("Remote".to_string()),
        avatar: Some("https://example.com/avatar.jpg".to_string()),
        website_url: Some("https://example.com".to_string()),
        github_url: Some("https://github.com/example".to_string()),
        is_active: false,
        is_deleted: true, // Mark as deleted
        created_at: Utc::now().to_rfc3339(),
        updated_at: Utc::now().to_rfc3339(),
    };
    
    let deleted_admin_dto = deleted_team_query_dto.to_admin_detail_dto(vec![]);
    
    assert_eq!(deleted_admin_dto.is_deleted, true, "Should preserve is_deleted field for deleted teams");
    assert_eq!(deleted_admin_dto.is_active, false, "Should preserve is_active field for deleted teams");
}