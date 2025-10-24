#!/bin/bash

# ==============================================================================
# Hackathon Tests - Comprehensive Endpoints
# ==============================================================================

source "$(dirname "$0")/../common/test-common.sh"

test_hackathon_endpoints() {
  printf "\n${CYAN}=== Testing Hackathon Endpoints ===${NC}\n"
  
  # Get hackathons
  test_api_endpoint "GET Hackathons" "GET" "/v1/hackathons" 200 "" false
  test_api_endpoint "GET Hackathons (Paginated)" "GET" "/v1/hackathons?page=1&limit=10" 200 "" false
  
  # Create hackathon - add organizers field (required)
  local create_hackathon_data=$(jq -n --arg user_id "$AUTH_USER_ID" '{
    name: "Test Hackathon '$(date +%s)'",
    description: "Auto-generated test hackathon",
    start_date: "'$(date -u +%Y-%m-%dT%H:%M:%SZ)'",
    end_date: "'$(date -u -d '+7 days' +%Y-%m-%dT%H:%M:%SZ)'",
    registration_deadline: "'$(date -u -d '+1 day' +%Y-%m-%dT%H:%M:%SZ)'",
    max_participants: 100,
    theme: "Technology",
    rules: "Follow the rules",
    prizes: [
      {position: 1, title: "Grand Prize", description: "First place", value: "$1000"},
      {position: 2, title: "Runner Up", description: "Second place", value: "$500"}
    ],
    organizers: [$user_id]
  }')
  local create_hackathon_response=$(test_api_endpoint "POST Create Hackathon" "POST" "/v1/hackathons" 201 "$create_hackathon_data" true)
  local created_hackathon_id=$(echo "$create_hackathon_response" | jq -r '.data.id // empty')
  
  if [ -n "$created_hackathon_id" ]; then
    # Get hackathon by ID
    test_api_endpoint "GET Hackathon By ID" "GET" "/v1/hackathons/$created_hackathon_id" 200 "" false
    
    # Update hackathon
    local update_hackathon_data=$(jq -n '{
      title: "Updated Test Hackathon",
      description: "Updated description",
      max_teams: 150
    }')
    test_api_endpoint "PUT Update Hackathon" "PUT" "/v1/hackathons/$created_hackathon_id" 200 "$update_hackathon_data" true
    
    # === Hackathon Events ===
    local create_event_data=$(jq -n --arg hackathon_id "$created_hackathon_id" '{
      hackathon_id: $hackathon_id,
      title: "Kickoff Meeting",
      description: "Opening ceremony and team formation",
      event_date: "'$(date -u +%Y-%m-%dT%H:%M:%SZ)'",
      location: "Online - Zoom",
      is_mandatory: true
    }')
    local create_event_response=$(test_api_endpoint "POST Create Hackathon Event" "POST" "/v1/hackathons/$created_hackathon_id/events" 201 "$create_event_data" true)
    local created_event_id=$(echo "$create_event_response" | jq -r '.data.id // empty')
    
    if [ -n "$created_event_id" ]; then
      # Update event
      local update_event_data=$(jq -n '{
        title: "Updated Kickoff Meeting",
        description: "Updated description",
        is_mandatory: false
      }')
      test_api_endpoint "PUT Update Hackathon Event" "PUT" "/v1/hackathons/events/$created_event_id" 200 "$update_event_data" true
      
      # Delete event
      test_api_endpoint "DELETE Hackathon Event" "DELETE" "/v1/hackathons/events/$created_event_id" 200 "" true
    fi
    
    # === Hackathon Timeline ===
    local create_timeline_data=$(jq -n --arg hackathon_id "$created_hackathon_id" '{
      hackathon_id: $hackathon_id,
      phase_name: "Registration Phase",
      description: "Team registration and formation",
      start_date: "'$(date -u +%Y-%m-%dT%H:%M:%SZ)'",
      end_date: "'$(date -u -d '+2 days' +%Y-%m-%dT%H:%M:%SZ)'",
      allowed_operations: ["REGISTER", "FORM_TEAM"]
    }')
    local create_timeline_response=$(test_api_endpoint "POST Create Timeline" "POST" "/v1/hackathons/$created_hackathon_id/timeline" 201 "$create_timeline_data" true)
    local created_timeline_id=$(echo "$create_timeline_response" | jq -r '.data.id // empty')
    
    if [ -n "$created_timeline_id" ]; then
      # Update timeline
      local update_timeline_data=$(jq -n '{
        phase_name: "Updated Registration Phase",
        description: "Updated description"
      }')
      test_api_endpoint "PUT Update Timeline" "PUT" "/v1/hackathons/timeline/$created_timeline_id" 200 "$update_timeline_data" true
      
      # Delete timeline
      test_api_endpoint "DELETE Timeline" "DELETE" "/v1/hackathons/timeline/$created_timeline_id" 200 "" true
    fi
    
    # === Hackathon Submissions ===
    # Note: Submissions require team participation
    # test_api_endpoint "GET Hackathon Submissions" "GET" "/v1/hackathons/$created_hackathon_id/submissions" 200 "" true
    # test_api_endpoint "GET My Submissions" "GET" "/v1/hackathons/submissions/me" 200 "" true
    
    # === Hackathon Results ===
    # test_api_endpoint "GET Admin Results" "GET" "/v1/hackathons/$created_hackathon_id/results" 200 "" true
    # test_api_endpoint "GET Public Results" "GET" "/v1/hackathons/$created_hackathon_id/results/public" 200 "" false
    
    # Delete hackathon
    test_api_endpoint "DELETE Hackathon" "DELETE" "/v1/hackathons/$created_hackathon_id" 200 "" true
  fi
}

# Run if executed directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
  get_auth_token
  test_hackathon_endpoints
  print_test_summary
  [ "$FAIL_COUNT" -eq 0 ] && exit 0 || exit 1
fi
