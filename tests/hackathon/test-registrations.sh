#!/bin/bash

# ==============================================================================
# Hackathon Registration Tests - Sprint 4
# ==============================================================================

source "$(dirname "$0")/../common/test-common.sh"

test_hackathon_registration_endpoints() {
  printf "\n${CYAN}=== Testing Hackathon Registration Endpoints ===${NC}\n"
  
  # First, create a hackathon to test with
  local create_hackathon_data=$(jq -n --arg user_id "$AUTH_USER_ID" '{
    name: "Registration Test Hackathon '$(date +%s)'",
    description: "Hackathon for testing registration endpoints",
    start_date: "'$(date -u -d '+7 days' +%Y-%m-%dT%H:%M:%SZ)'",
    end_date: "'$(date -u -d '+14 days' +%Y-%m-%dT%H:%M:%SZ)'",
    registration_deadline: "'$(date -u -d '+5 days' +%Y-%m-%dT%H:%M:%SZ)'",
    max_participants: 100,
    theme: "Innovation",
    rules: "Follow the hackathon rules",
    prizes: [
      {position: 1, title: "First Prize", description: "Winner", value: "$5000"}
    ],
    organizers: [$user_id]
  }')
  
  local create_hackathon_response=$(curl -s -X POST -H "Authorization: Bearer $AUTH_TOKEN" \
    -H "Content-Type: application/json" -d "$create_hackathon_data" \
    "$BASE_URL/v1/hackathons")
  local hackathon_id=$(echo "$create_hackathon_response" | jq -r '.data.id // empty')
  
  if [ -z "$hackathon_id" ]; then
    printf "${RED}✗ Failed to create test hackathon${NC}\n"
    return 1
  fi
  
  printf "${GREEN}✓ Created test hackathon: $hackathon_id${NC}\n"
  
  # === 1. Register for Hackathon ===
  printf "\n${CYAN}Testing: POST /v1/hackathons/{id}/registrations/create${NC}\n"
  local register_data=$(jq -n '{
    role: "individual",
    skills: ["Rust", "Web Development", "API Design"],
    experience_level: "intermediate",
    github_username: "testuser123",
    portfolio_url: "https://portfolio.example.com",
    motivation: "I am passionate about building scalable systems",
    dietary_requirements: "Vegetarian",
    tshirt_size: "L",
    emergency_contact_name: "John Doe",
    emergency_contact_phone: "+1234567890",
    emergency_contact_relationship: "Father"
  }')
  
  local register_response=$(test_api_endpoint "POST Register for Hackathon" "POST" "/v1/hackathons/$hackathon_id/registrations/create" 200 "$register_data" true)
  local registration_id=$(echo "$register_response" | jq -r '.data.id // empty')
  
  if [ -z "$registration_id" ]; then
    printf "${RED}✗ Failed to create registration${NC}\n"
  else
    printf "${GREEN}✓ Created registration: $registration_id${NC}\n"
    
    # Test duplicate registration (should fail)
    printf "\n${CYAN}Testing: Duplicate registration (should fail)${NC}\n"
    local dup_response=$(curl -s -w "\n%{http_code}" -X POST \
      -H "Authorization: Bearer $AUTH_TOKEN" \
      -H "Content-Type: application/json" -d "$register_data" \
      "$BASE_URL/v1/hackathons/$hackathon_id/registrations/create")
    local dup_status=$(echo "$dup_response" | tail -n1)
    if [ "$dup_status" == "400" ]; then
      printf "${GREEN}✓ Duplicate registration prevented${NC}\n"
    else
      printf "${RED}✗ Should prevent duplicate registration (got $dup_status)${NC}\n"
    fi
    
    # === 2. Get Hackathon Registrations (Admin View) ===
    printf "\n${CYAN}Testing: GET /v1/hackathons/{id}/registrations${NC}\n"
    test_api_endpoint "GET All Registrations" "GET" "/v1/hackathons/$hackathon_id/registrations" 200 "" true
    test_api_endpoint "GET Registrations (Paginated)" "GET" "/v1/hackathons/$hackathon_id/registrations?page=1&page_size=10" 200 "" true
    test_api_endpoint "GET Registrations (Filter by status)" "GET" "/v1/hackathons/$hackathon_id/registrations?status=pending" 200 "" true
    
    # === 3. Get User's Hackathon Registrations ===
    printf "\n${CYAN}Testing: GET /v1/users/me/hackathons${NC}\n"
    test_api_endpoint "GET My Hackathons" "GET" "/v1/users/me/hackathons" 200 "" true
    
    # === 4. Update Registration Status ===
    printf "\n${CYAN}Testing: PUT /v1/hackathons/{hackathon_id}/registrations/update/{registration_id}/status${NC}\n"
    
    # Approve registration
    local approve_data=$(jq -n '{
      status: "approved",
      reason: "Your application meets all requirements. Welcome!"
    }')
    test_api_endpoint "PUT Approve Registration" "PUT" "/v1/hackathons/$hackathon_id/registrations/update/$registration_id/status" 200 "$approve_data" true
    
    # Test reject status
    local reject_data=$(jq -n '{
      status: "rejected",
      reason: "Unfortunately, we are at capacity."
    }')
    # This will fail since already approved, but test the endpoint
    local reject_response=$(curl -s -w "\n%{http_code}" -X PUT \
      -H "Authorization: Bearer $AUTH_TOKEN" \
      -H "Content-Type: application/json" -d "$reject_data" \
      "$BASE_URL/v1/hackathons/$hackathon_id/registrations/update/$registration_id/status")
    
    # Re-approve for check-in test
    curl -s -X PUT -H "Authorization: Bearer $AUTH_TOKEN" \
      -H "Content-Type: application/json" -d "$approve_data" \
      "$BASE_URL/v1/hackathons/$hackathon_id/registrations/update/$registration_id/status" > /dev/null
    
    # Test waitlist status
    local waitlist_data=$(jq -n '{
      status: "waitlisted",
      reason: "You are on the waitlist and will be notified if a spot opens."
    }')
    curl -s -X PUT -H "Authorization: Bearer $AUTH_TOKEN" \
      -H "Content-Type: application/json" -d "$waitlist_data" \
      "$BASE_URL/v1/hackathons/$hackathon_id/registrations/update/$registration_id/status" > /dev/null
    
    # Re-approve again for check-in
    curl -s -X PUT -H "Authorization: Bearer $AUTH_TOKEN" \
      -H "Content-Type: application/json" -d "$approve_data" \
      "$BASE_URL/v1/hackathons/$hackathon_id/registrations/update/$registration_id/status" > /dev/null
    
    # === 5. Check-in Participant ===
    printf "\n${CYAN}Testing: POST /v1/hackathons/{hackathon_id}/registrations/{registration_id}/check-in${NC}\n"
    test_api_endpoint "POST Check-in Participant" "POST" "/v1/hackathons/$hackathon_id/registrations/$registration_id/check-in" 200 "" true
    
    # Test duplicate check-in (should fail)
    printf "\n${CYAN}Testing: Duplicate check-in (should fail)${NC}\n"
    local dup_checkin_response=$(curl -s -w "\n%{http_code}" -X POST \
      -H "Authorization: Bearer $AUTH_TOKEN" \
      "$BASE_URL/v1/hackathons/$hackathon_id/registrations/$registration_id/check-in")
    local dup_checkin_status=$(echo "$dup_checkin_response" | tail -n1)
    if [ "$dup_checkin_status" == "400" ]; then
      printf "${GREEN}✓ Duplicate check-in prevented${NC}\n"
    else
      printf "${RED}✗ Should prevent duplicate check-in (got $dup_checkin_status)${NC}\n"
    fi
    
    # === 6. Get Registration Statistics ===
    printf "\n${CYAN}Testing: GET /v1/hackathons/{id}/registrations/stats${NC}\n"
    local stats_response=$(test_api_endpoint "GET Registration Stats" "GET" "/v1/hackathons/$hackathon_id/registrations/stats" 200 "" true)
    
    # Verify stats structure
    local total=$(echo "$stats_response" | jq -r '.data.total_registrations // empty')
    local approved=$(echo "$stats_response" | jq -r '.data.approved_count // empty')
    local checked_in=$(echo "$stats_response" | jq -r '.data.checked_in_count // empty')
    
    if [ -n "$total" ] && [ -n "$approved" ] && [ -n "$checked_in" ]; then
      printf "${GREEN}✓ Stats structure valid: total=$total, approved=$approved, checked_in=$checked_in${NC}\n"
    else
      printf "${RED}✗ Stats structure incomplete${NC}\n"
    fi
    
    # === Test with Team Registration ===
    printf "\n${CYAN}Testing: Registration with Team${NC}\n"
    
    # Create a team first
    local create_team_data=$(jq -n '{
      name: "Test Registration Team '$(date +%s)'",
      description: "Team for registration testing",
      max_members: 5
    }')
    local team_response=$(curl -s -X POST -H "Authorization: Bearer $AUTH_TOKEN" \
      -H "Content-Type: application/json" -d "$create_team_data" \
      "$BASE_URL/v1/teams/create")
    local team_id=$(echo "$team_response" | jq -r '.data.id // empty')
    
    if [ -n "$team_id" ]; then
      # Create second hackathon for team test
      local hackathon2_data=$(jq -n --arg user_id "$AUTH_USER_ID" '{
        name: "Team Registration Test '$(date +%s)'",
        description: "Testing team registration",
        start_date: "'$(date -u -d '+7 days' +%Y-%m-%dT%H:%M:%SZ)'",
        end_date: "'$(date -u -d '+14 days' +%Y-%m-%dT%H:%M:%SZ)'",
        registration_deadline: "'$(date -u -d '+5 days' +%Y-%m-%dT%H:%M:%SZ)'",
        max_participants: 50,
        theme: "Teamwork",
        organizers: [$user_id]
      }')
      local hackathon2_response=$(curl -s -X POST -H "Authorization: Bearer $AUTH_TOKEN" \
        -H "Content-Type: application/json" -d "$hackathon2_data" \
        "$BASE_URL/v1/hackathons")
      local hackathon2_id=$(echo "$hackathon2_response" | jq -r '.data.id // empty')
      
      if [ -n "$hackathon2_id" ]; then
        local team_register_data=$(jq -n --arg team_id "$team_id" '{
          role: "participant",
          team_id: $team_id,
          skills: ["Teamwork", "Leadership"],
          experience_level: "advanced",
          motivation: "We work great as a team",
          tshirt_size: "M",
          emergency_contact_name: "Jane Doe",
          emergency_contact_phone: "+0987654321",
          emergency_contact_relationship: "Mother"
        }')
        test_api_endpoint "POST Register with Team" "POST" "/v1/hackathons/$hackathon2_id/registrations/create" 200 "$team_register_data" true
        
        # Cleanup second hackathon
        curl -s -X DELETE -H "Authorization: Bearer $AUTH_TOKEN" \
          "$BASE_URL/v1/hackathons/delete/$hackathon2_id" > /dev/null
      fi
    fi
    
    # === Test Different Participant Roles ===
    printf "\n${CYAN}Testing: Different Participant Roles${NC}\n"
    
    # Create hackathon for role tests
    local hackathon3_data=$(jq -n --arg user_id "$AUTH_USER_ID" '{
      name: "Role Test Hackathon '$(date +%s)'",
      description: "Testing different roles",
      start_date: "'$(date -u -d '+7 days' +%Y-%m-%dT%H:%M:%SZ)'",
      end_date: "'$(date -u -d '+14 days' +%Y-%m-%dT%H:%M:%SZ)'",
      registration_deadline: "'$(date -u -d '+5 days' +%Y-%m-%dT%H:%M:%SZ)'",
      max_participants: 30,
      organizers: [$user_id]
    }')
    local hackathon3_response=$(curl -s -X POST -H "Authorization: Bearer $AUTH_TOKEN" \
      -H "Content-Type: application/json" -d "$hackathon3_data" \
      "$BASE_URL/v1/hackathons")
    local hackathon3_id=$(echo "$hackathon3_response" | jq -r '.data.id // empty')
    
    if [ -n "$hackathon3_id" ]; then
      # Test individual role with advanced experience
      local individual_advanced_register=$(jq -n '{
        role: "individual",
        skills: ["Mentoring", "Technical Guidance"],
        experience_level: "advanced",
        motivation: "I want to challenge myself with advanced projects",
        tshirt_size: "L"
      }')
      test_api_endpoint "POST Register as Advanced Individual" "POST" "/v1/hackathons/$hackathon3_id/registrations/create" 200 "$individual_advanced_register" true
      
      # Cleanup third hackathon
      curl -s -X DELETE -H "Authorization: Bearer $AUTH_TOKEN" \
        "$BASE_URL/v1/hackathons/delete/$hackathon3_id" > /dev/null
    fi
  fi
  
  # Cleanup test hackathon
  if [ -n "$hackathon_id" ]; then
    curl -s -X DELETE -H "Authorization: Bearer $AUTH_TOKEN" \
      "$BASE_URL/v1/hackathons/delete/$hackathon_id" > /dev/null
    printf "\n${GREEN}✓ Cleaned up test hackathon${NC}\n"
  fi
}

# Run if executed directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
  get_auth_token
  test_hackathon_registration_endpoints
  print_test_summary
  [ "$FAIL_COUNT" -eq 0 ] && exit 0 || exit 1
fi
