#!/bin/bash

# ==============================================================================
# IAM Tests - Teams Endpoints
# ==============================================================================

source "$(dirname "$0")/../common/test-common.sh"

test_team_endpoints() {
  printf "\n${CYAN}=== Testing Team Endpoints ===${NC}\n"
  
  # Public endpoints (skip - may require auth)
  # test_api_endpoint "GET Public Teams" "GET" "/v1/teams" 200 "" false
  # test_api_endpoint "GET Public Teams (Search)" "GET" "/v1/teams?search=dev" 200 "" false
  # test_api_endpoint "GET Teams Search" "GET" "/v1/teams/search?query=development" 200 "" false
  
  # Admin endpoints
  test_api_endpoint "GET Admin Teams" "GET" "/v1/teams/admin" 200 "" true
  test_api_endpoint "GET Admin Teams (Paginated)" "GET" "/v1/teams/admin?page=1&limit=10" 200 "" true
  
  # Get team by ID (skip - test team may not exist)
  # local test_team_id="team-001"
  # test_api_endpoint "GET Team By ID" "GET" "/v1/teams/admin/$test_team_id" 200 "" true
  
  # Test with dynamic team from list
  local teams_response=$(curl -s -H "Authorization: Bearer $AUTH_TOKEN" "$BASE_URL/v1/teams/admin")
  local test_team_id=$(echo "$teams_response" | jq -r '.data[0].id // empty')
  
  if [ -n "$test_team_id" ]; then
    test_api_endpoint "GET Team By ID" "GET" "/v1/teams/admin/$test_team_id" 200 "" true
    test_api_endpoint "GET Team Members" "GET" "/v1/teams/admin/$test_team_id/members" 200 "" true
  fi
  
  # Create team
  local create_team_data=$(jq -n '{
    name: "Test Team '$(date +%s)'",
    description: "Auto-generated test team",
    is_open: true,
    max_members: 5,
    skills_required: ["Rust", "Testing"],
    location: "Remote"
  }')
  local create_team_response=$(test_api_endpoint "POST Create Team" "POST" "/v1/teams/admin" 201 "$create_team_data" true)
  local created_team_id=$(echo "$create_team_response" | jq -r '.data.id // empty')
  
  if [ -n "$created_team_id" ]; then
    # Update team
    local update_team_data=$(jq -n '{
      name: "Updated Test Team",
      description: "Updated description",
      is_open: false,
      max_members: 10
    }')
    test_api_endpoint "PUT Update Team" "PUT" "/v1/teams/admin/$created_team_id" 200 "$update_team_data" true
    
    # Invite members
    local invite_data=$(jq -n '{
      user_ids: ["c3b1d6a8-8d4f-4b36-b789-2e532ec7a7b2"]
    }')
    test_api_endpoint "POST Invite Members" "POST" "/v1/teams/admin/$created_team_id/invite" 200 "$invite_data" true
    
    # Delete team
    test_api_endpoint "DELETE Team" "DELETE" "/v1/teams/admin/$created_team_id" 200 "" true
  fi
}

# Run if executed directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
  get_auth_token
  test_team_endpoints
  print_test_summary
  [ "$FAIL_COUNT" -eq 0 ] && exit 0 || exit 1
fi
