#!/bin/bash

# ==============================================================================
# IAM Tests - Teams Endpoints
# ==============================================================================

source "$(dirname "$0")/../common/test-common.sh"

test_team_endpoints() {
  printf "\n${CYAN}=== Testing Team Endpoints ===${NC}\n"
  
  # === Public Team Endpoints (Authenticated) ===
  test_api_endpoint "GET Public Teams List" "GET" "/v1/teams" 200 "" true
  test_api_endpoint "GET Public Teams (Paginated)" "GET" "/v1/teams?page=1&limit=10" 200 "" true
  test_api_endpoint "GET Teams Search" "GET" "/v1/teams/search?query=test" 200 "" true
  
  # === Admin Endpoints ===
  test_api_endpoint "GET Admin Teams" "GET" "/v1/teams/admin" 200 "" true
  test_api_endpoint "GET Admin Teams (Paginated)" "GET" "/v1/teams/admin?page=1&limit=10" 200 "" true
  
  # Test with dynamic team from list
  local teams_response=$(curl -s -H "Authorization: Bearer $AUTH_TOKEN" "$BASE_URL/v1/teams/admin")
  local test_team_id=$(echo "$teams_response" | jq -r '.data[0].id // empty')
  
  if [ -n "$test_team_id" ]; then
    test_api_endpoint "GET Team By ID" "GET" "/v1/teams/admin/$test_team_id" 200 "" true
    test_api_endpoint "GET Team Members" "GET" "/v1/teams/admin/$test_team_id/members" 200 "" true
    test_api_endpoint "GET Team By ID (Public)" "GET" "/v1/teams/$test_team_id" 200 "" true
    test_api_endpoint "GET Team Members (Public)" "GET" "/v1/teams/$test_team_id/members" 200 "" true
  fi
  
  # === Create Team and Test Full Flow ===
  local create_team_data=$(jq -n '{
    name: "Test Team '$(date +%s)'",
    description: "Auto-generated test team for comprehensive testing",
    is_open: true,
    max_members: 5,
    skills_required: ["Rust", "Testing", "API"],
    location: "Remote"
  }')
  local create_team_response=$(test_api_endpoint "POST Create Team" "POST" "/v1/teams/create" 201 "$create_team_data" true)
  local created_team_id=$(echo "$create_team_response" | jq -r '.data.id // empty')
  
  if [ -n "$created_team_id" ]; then
    # Update team
    local update_team_data=$(jq -n '{
      name: "Updated Test Team",
      description: "Updated description for testing",
      is_open: false,
      max_members: 10
    }')
    test_api_endpoint "PUT Update Team" "PUT" "/v1/teams/update/$created_team_id" 200 "$update_team_data" true
    
    # === Team Member Management ===
    # Get a test user ID for member operations
    local users_response=$(curl -s -H "Authorization: Bearer $AUTH_TOKEN" "$BASE_URL/v1/users?page=1&limit=1")
    local test_user_id=$(echo "$users_response" | jq -r '.data[0].id // empty')
    
    if [ -n "$test_user_id" ] && [ "$test_user_id" != "$AUTH_USER_ID" ]; then
      # Add team member
      local add_member_data=$(jq -n --arg user_id "$test_user_id" '{
        user_id: $user_id,
        role: "member"
      }')
      test_api_endpoint "POST Add Team Member" "POST" "/v1/teams/$created_team_id/members" 200 "$add_member_data" true
      
      # Remove team member
      test_api_endpoint "DELETE Remove Team Member" "DELETE" "/v1/teams/$created_team_id/members/$test_user_id" 200 "" true
    fi
    
    # === Team Invitation Flow ===
    local invite_emails_data=$(jq -n '{
      emails: ["test-invite@example.com"],
      message: "Join our test team!"
    }')
    local invite_response=$(test_api_endpoint "POST Invite Team Members" "POST" "/v1/teams/$created_team_id/invite" 200 "$invite_emails_data" true)
    
    # Note: Accept invitation requires valid token from email
    # This would be tested in integration tests with email service
    # test_api_endpoint "POST Accept Invitation" "POST" "/v1/teams/accept/{token}" 200 "" true
    
    # === Leave Team ===
    # Test leave team endpoint (will fail if user is owner, which is expected)
    # test_api_endpoint "POST Leave Team" "POST" "/v1/teams/$created_team_id/leave" 200 "" true
    # test_api_endpoint "POST Leave Current Team" "POST" "/v1/teams/leave-me" 200 "" true
    
    # === Get My Team ===
    test_api_endpoint "GET My Team" "GET" "/v1/teams/me" 200 "" true
    
    # Delete team (cleanup)
    test_api_endpoint "DELETE Team" "DELETE" "/v1/teams/delete/$created_team_id" 200 "" true
  fi
}

# Run if executed directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
  get_auth_token
  test_team_endpoints
  print_test_summary
  [ "$FAIL_COUNT" -eq 0 ] && exit 0 || exit 1
fi
