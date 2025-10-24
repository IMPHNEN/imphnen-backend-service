#!/bin/bash

# ==============================================================================
# CMS Tests - Events and Testimonials Endpoints
# ==============================================================================

source "$(dirname "$0")/../common/test-common.sh"

test_events_endpoints() {
  printf "\n${CYAN}=== Testing Events Endpoints ===${NC}\n"
  
  # Public endpoints
  test_api_endpoint "GET Events List" "GET" "/v1/cms/landing/events" 200 "" false
  test_api_endpoint "GET Events (Paginated)" "GET" "/v1/cms/landing/events?page=1&limit=10" 200 "" false
  test_api_endpoint "GET Events (Search)" "GET" "/v1/cms/landing/events?search=test" 200 "" false
  test_api_endpoint "GET Events (Filter Online)" "GET" "/v1/cms/landing/events?filter=online" 200 "" false
  
  # Get event by ID - use correct endpoint /detail/{id}
  local events_response=$(curl -s "$BASE_URL/v1/cms/landing/events")
  local test_event_id=$(echo "$events_response" | jq -r '.data[0].id // empty')
  
  if [ -n "$test_event_id" ]; then
    test_api_endpoint "GET Event By ID" "GET" "/v1/cms/landing/events/detail/$test_event_id" 200 "" false
  fi
  
  # Create event (protected) - use correct field name
  local create_event_data=$(jq -n '{
    name: "Test Event '$(date +%s)'",
    description: "Auto-generated test event",
    start_date: "'$(date -u +%Y-%m-%dT%H:%M:%SZ)'",
    end_date: "'$(date -u -d '+2 hours' +%Y-%m-%dT%H:%M:%SZ)'",
    detail_link: "https://example.com/event",
    price: 0,
    is_online: true
  }')
  local create_event_response=$(test_api_endpoint "POST Create Event" "POST" "/v1/cms/landing/events/create" 201 "$create_event_data" true)
  local created_event_id=$(echo "$create_event_response" | jq -r '.data.id // empty')
  
  if [ -n "$created_event_id" ]; then
    # Update event - use correct endpoint /update/{id} with PATCH
    local update_event_data=$(jq -n '{
      name: "Updated Test Event",
      description: "Updated description",
      is_online: false
    }')
    test_api_endpoint "PATCH Update Event" "PATCH" "/v1/cms/landing/events/update/$created_event_id" 200 "$update_event_data" true
    
    # Delete event - use correct endpoint /delete/{id}
    test_api_endpoint "DELETE Event" "DELETE" "/v1/cms/landing/events/delete/$created_event_id" 200 "" true
  fi
}

test_testimonials_endpoints() {
  printf "\n${CYAN}=== Testing Testimonials Endpoints ===${NC}\n"
  
  # Public endpoints
  test_api_endpoint "GET Testimonials List" "GET" "/v1/cms/landing/testimonials" 200 "" false
  test_api_endpoint "GET Testimonials (Paginated)" "GET" "/v1/cms/landing/testimonials?page=1&limit=10" 200 "" false
  test_api_endpoint "GET Testimonials (Search)" "GET" "/v1/cms/landing/testimonials?search=test" 200 "" false
  
  # Get testimonial by ID - use correct endpoint /detail/{id}
  local testimonials_response=$(curl -s "$BASE_URL/v1/cms/landing/testimonials")
  local test_testimonial_id=$(echo "$testimonials_response" | jq -r '.data[0].id // empty')
  
  if [ -n "$test_testimonial_id" ]; then
    test_api_endpoint "GET Testimonial By ID" "GET" "/v1/cms/landing/testimonials/detail/$test_testimonial_id" 200 "" false
  fi
  
  # Create testimonial (protected)
  local create_testimonial_data=$(jq -n '{
    role: "Student",
    content: "This is a test testimonial created at '$(date +%s)'"
  }')
  local create_testimonial_response=$(test_api_endpoint "POST Create Testimonial" "POST" "/v1/cms/landing/testimonials/create" 201 "$create_testimonial_data" true)
  local created_testimonial_id=$(echo "$create_testimonial_response" | jq -r '.data.id // empty')
  
  if [ -n "$created_testimonial_id" ]; then
    # Update testimonial - use correct endpoint /update/{id} with PATCH
    local update_testimonial_data=$(jq -n '{
      role: "Alumni",
      content: "Updated testimonial content"
    }')
    test_api_endpoint "PATCH Update Testimonial" "PATCH" "/v1/cms/landing/testimonials/update/$created_testimonial_id" 200 "$update_testimonial_data" true
    
    # Delete testimonial - use correct endpoint /delete/{id}
    test_api_endpoint "DELETE Testimonial" "DELETE" "/v1/cms/landing/testimonials/delete/$created_testimonial_id" 200 "" true
  fi
}

# Run if executed directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
  get_auth_token
  test_events_endpoints
  test_testimonials_endpoints
  print_test_summary
  [ "$FAIL_COUNT" -eq 0 ] && exit 0 || exit 1
fi
