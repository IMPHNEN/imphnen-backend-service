#!/bin/bash

# ==============================================================================
# Dimentorin Tests - Mentors Endpoints
# ==============================================================================

source "$(dirname "$0")/../common/test-common.sh"

test_mentor_endpoints() {
  printf "\n${CYAN}=== Testing Mentor Endpoints ===${NC}\n"
  
  # Get mentors list
  test_api_endpoint "GET Mentors List" "GET" "/v1/mentors" 200 "" true
  test_api_endpoint "GET Mentors (Paginated)" "GET" "/v1/mentors?page=1&limit=10" 200 "" true
  test_api_endpoint "GET Mentors (Search)" "GET" "/v1/mentors?search=mentor" 200 "" true
  
  # Get mentor by ID - use correct endpoint /detail/{id}
  local mentors_response=$(curl -s -H "Authorization: Bearer $AUTH_TOKEN" "$BASE_URL/v1/mentors")
  local test_mentor_id=$(echo "$mentors_response" | jq -r '.data[0].id // empty')
  
  if [ -n "$test_mentor_id" ]; then
    test_api_endpoint "GET Mentor By ID" "GET" "/v1/mentors/detail/$test_mentor_id" 200 "" true
    
    # Verify mentor (admin only) - use correct endpoint /verify/{id}
    local verify_data=$(jq -n '{status: "verified"}')
    test_api_endpoint "PUT Verify Mentor" "PUT" "/v1/mentors/verify/$test_mentor_id" 200 "$verify_data" true
    
    # Update mentor (admin) - use correct endpoint /update/{id}
    local update_mentor_data=$(jq -n '{
      expertise: ["Rust", "Backend", "DevOps"],
      bio: "This is an updated mentor bio with sufficient length to meet the 50 character minimum requirement for validation"
    }')
    test_api_endpoint "PUT Update Mentor" "PUT" "/v1/mentors/update/$test_mentor_id" 200 "$update_mentor_data" true
  fi
  
  # Note: Mentor Me and Mentor Status endpoints require mentor-specific token
  # test_api_endpoint "GET Mentor Me" "GET" "/v1/mentors/me" 200 "" true
  # test_api_endpoint "GET Mentor Status" "GET" "/v1/mentors/me/status" 200 "" true
  
  # Delete mentor (admin)
  # test_api_endpoint "DELETE Mentor" "DELETE" "/v1/mentors/delete/$test_mentor_id" 200 "" true
}

# Run if executed directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
  get_auth_token
  test_mentor_endpoints
  print_test_summary
  [ "$FAIL_COUNT" -eq 0 ] && exit 0 || exit 1
fi
