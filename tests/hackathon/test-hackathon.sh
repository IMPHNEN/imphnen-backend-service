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
      start_time: "'$(date -u +%Y-%m-%dT09:00:00Z)'",
      end_time: "'$(date -u +%Y-%m-%dT11:00:00Z)'",
      location: "Online - Zoom",
      event_type: "workshop",
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
      phase: "registration",
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
        phase: "registration",
        phase_name: "Updated Registration Phase",
        description: "Updated description"
      }')
      test_api_endpoint "PUT Update Timeline" "PUT" "/v1/hackathons/timeline/$created_timeline_id" 200 "$update_timeline_data" true
      
      # Delete timeline
      test_api_endpoint "DELETE Timeline" "DELETE" "/v1/hackathons/timeline/$created_timeline_id" 200 "" true
    fi
    
    # === Hackathon Participants ===
    printf "\n${CYAN}Testing Hackathon Participants...${NC}\n"
    
    # Register participant for hackathon
    local register_participant_data=$(jq -n --arg hackathon_id "$created_hackathon_id" --arg user_id "$AUTH_USER_ID" '{
      hackathon_id: $hackathon_id,
      user_id: $user_id,
      role: "participant"
    }')
    test_api_endpoint "POST Register Participant" "POST" "/v1/hackathons/$created_hackathon_id/participants" 200 "$register_participant_data" true
    
    # List participants
    test_api_endpoint "GET List Participants" "GET" "/v1/hackathons/$created_hackathon_id/participants" 200 "" true
    test_api_endpoint "GET List Participants (Paginated)" "GET" "/v1/hackathons/$created_hackathon_id/participants?page=1&limit=10" 200 "" true
    
    # === Hackathon Submissions ===
    printf "\n${CYAN}Testing Hackathon Submissions...${NC}\n"
    
    # Get or create a team for submissions
    local teams_response=$(curl -s -H "Authorization: Bearer $AUTH_TOKEN" "$BASE_URL/v1/teams?page=1&limit=1")
    local team_id=$(echo "$teams_response" | jq -r '.data[0].id // empty')
    
    if [ -z "$team_id" ]; then
      # Create a team for submission testing
      local create_team_data=$(jq -n '{
        name: "Hackathon Test Team '$(date +%s)'",
        description: "Team for hackathon submission testing",
        max_members: 5
      }')
      local team_response=$(curl -s -X POST -H "Authorization: Bearer $AUTH_TOKEN" \
        -H "Content-Type: application/json" -d "$create_team_data" \
        "$BASE_URL/v1/teams/create")
      team_id=$(echo "$team_response" | jq -r '.data.id // empty')
    fi
    
    if [ -n "$team_id" ]; then
      # Create submission with all required fields
      local create_submission_data=$(jq -n --arg hackathon_id "$created_hackathon_id" --arg team_id "$team_id" '{
        project_name: "Test Submission '$(date +%s)'",
        description: "Automated test submission for hackathon",
        repository_url: "https://github.com/test/repo",
        upload_file_url: "https://storage.example.com/submissions/test-project.zip",
        demo_url: "https://demo.example.com",
        slides_url: "https://slides.example.com/test",
        technologies: ["Rust", "Axum", "SurrealDB"],
        contact_instagram: "@team_instagram",
        contact_twitter: "@team_twitter",
        contact_linkedin: "linkedin.com/in/team"
      }')
      local create_submission_response=$(test_api_endpoint "POST Create Submission" "POST" "/v1/hackathons/$created_hackathon_id/teams/$team_id/submissions" 201 "$create_submission_data" true)
      local submission_id=$(echo "$create_submission_response" | jq -r '.data.id // empty')
      
      if [ -n "$submission_id" ]; then
        # Get submission by ID
        test_api_endpoint "GET Submission By ID" "GET" "/v1/hackathons/submissions/$submission_id" 200 "" true
        
        # List all submissions for hackathon
        test_api_endpoint "GET Hackathon Submissions" "GET" "/v1/hackathons/$created_hackathon_id/submissions" 200 "" true
        test_api_endpoint "GET Hackathon Submissions (Paginated)" "GET" "/v1/hackathons/$created_hackathon_id/submissions?page=1&limit=10" 200 "" true
        
        # Update submission
        local update_submission_data=$(jq -n '{
          project_name: "Updated Test Submission",
          description: "Updated description for testing",
          repository_url: "https://github.com/test/updated-repo",
          upload_file_url: "https://storage.example.com/submissions/updated-project.zip",
          technologies: ["Rust", "Axum", "PostgreSQL"],
          contact_youtube: "youtube.com/@teamchannel",
          contact_facebook: "facebook.com/teampage"
        }')
        test_api_endpoint "PUT Update Submission" "PUT" "/v1/hackathons/submissions/$submission_id" 200 "$update_submission_data" true
        
        # === Test Validation Errors ===
        printf "\n${CYAN}Testing Submission Validation Errors...${NC}\n"
        
        # Create a submission without repo/upload to test validation
        local invalid_submission_no_repo=$(jq -n '{
          project_name: "Invalid Submission No Repo",
          description: "Testing validation - missing both repo and upload",
          technologies: ["Test"],
          contact_instagram: "@testaccount"
        }')
        local invalid_sub_response=$(curl -s -X POST -H "Authorization: Bearer $AUTH_TOKEN" \
          -H "Content-Type: application/json" -d "$invalid_submission_no_repo" \
          "$BASE_URL/v1/hackathons/$created_hackathon_id/teams/$team_id/submissions")
        local invalid_sub_id=$(echo "$invalid_sub_response" | jq -r '.data.id // empty')
        
        if [ -n "$invalid_sub_id" ]; then
          # Try to submit without repo/upload - should fail with 400
          printf "${YELLOW}Testing: Submit without repo/upload (should fail)${NC}\n"
          local submit_response=$(curl -s -w "\n%{http_code}" -X POST \
            -H "Authorization: Bearer $AUTH_TOKEN" \
            "$BASE_URL/v1/hackathons/submissions/$invalid_sub_id/submit")
          local submit_status=$(echo "$submit_response" | tail -n1)
          if [ "$submit_status" == "400" ]; then
            printf "${GREEN}✓ Validation works: Rejected submission without repo/upload${NC}\n"
          else
            printf "${RED}✗ Validation failed: Should reject submission without repo/upload (got $submit_status)${NC}\n"
          fi
          
          # Cleanup invalid submission
          curl -s -X DELETE -H "Authorization: Bearer $AUTH_TOKEN" \
            "$BASE_URL/v1/hackathons/submissions/$invalid_sub_id" > /dev/null
        fi
        
        # Create a submission without contact to test validation
        local invalid_submission_no_contact=$(jq -n '{
          project_name: "Invalid Submission No Contact",
          description: "Testing validation - missing contact",
          repository_url: "https://github.com/test/repo",
          technologies: ["Test"]
        }')
        invalid_sub_response=$(curl -s -X POST -H "Authorization: Bearer $AUTH_TOKEN" \
          -H "Content-Type: application/json" -d "$invalid_submission_no_contact" \
          "$BASE_URL/v1/hackathons/$created_hackathon_id/teams/$team_id/submissions")
        invalid_sub_id=$(echo "$invalid_sub_response" | jq -r '.data.id // empty')
        
        if [ -n "$invalid_sub_id" ]; then
          # Try to submit without contact - should fail with 400
          printf "${YELLOW}Testing: Submit without social media contact (should fail)${NC}\n"
          submit_response=$(curl -s -w "\n%{http_code}" -X POST \
            -H "Authorization: Bearer $AUTH_TOKEN" \
            "$BASE_URL/v1/hackathons/submissions/$invalid_sub_id/submit")
          submit_status=$(echo "$submit_response" | tail -n1)
          if [ "$submit_status" == "400" ]; then
            printf "${GREEN}✓ Validation works: Rejected submission without social media contact${NC}\n"
          else
            printf "${RED}✗ Validation failed: Should reject submission without contact (got $submit_status)${NC}\n"
          fi
          
          # Cleanup invalid submission
          curl -s -X DELETE -H "Authorization: Bearer $AUTH_TOKEN" \
            "$BASE_URL/v1/hackathons/submissions/$invalid_sub_id" > /dev/null
        fi
        
        # === Submit Valid Submission ===
        printf "\n${CYAN}Testing Valid Submission...${NC}\n"
        # Submit final submission (no body required) - should succeed as all validations pass
        test_api_endpoint "POST Submit Final Submission" "POST" "/v1/hackathons/submissions/$submission_id/submit" 200 "" true
        
        # Update submission status (admin only)
        local update_status_data=$(jq -n '{
          status: "under_review",
          feedback: "Great project, under review by our panel"
        }')
        test_api_endpoint "PUT Update Submission Status" "PUT" "/v1/hackathons/submissions/$submission_id/status" 200 "$update_status_data" true
        
        # Get user submissions
        test_api_endpoint "GET User Submissions" "GET" "/v1/users/$AUTH_USER_ID/hackathon-submissions" 200 "" true
        
        # Delete submission
        test_api_endpoint "DELETE Submission" "DELETE" "/v1/hackathons/submissions/$submission_id" 200 "" true
      fi
    fi
    
    # === Hackathon Results ===
    printf "\n${CYAN}Testing Hackathon Results...${NC}\n"
    test_api_endpoint "GET Public Results" "GET" "/v1/hackathons/$created_hackathon_id/results" 200 "" false
    test_api_endpoint "GET Admin Results" "GET" "/v1/hackathons/$created_hackathon_id/admin/results" 200 "" true
    
    # === Search Hackathons ===
    local search_data=$(jq -n '{
      query: "test",
      page: 1,
      limit: 10
    }')
    test_api_endpoint "POST Search Hackathons" "POST" "/v1/hackathons/search" 200 "$search_data" false
    
    # Delete hackathon (cleanup)
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
