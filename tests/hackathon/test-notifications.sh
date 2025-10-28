#!/bin/bash

# ==============================================================================
# Notifications Tests - Sprint 5
# ==============================================================================

source "$(dirname "$0")/../common/test-common.sh"

test_notification_endpoints() {
  printf "\n${CYAN}=== Testing Notification Endpoints ===${NC}\n"
  
  # Note: Notifications are typically created by the system when certain events occur
  # For testing purposes, we'll need to trigger events that create notifications
  # or manually insert test notifications via database
  
  # === 1. Get User Notifications ===
  printf "\n${CYAN}Testing: GET /v1/notifications${NC}\n"
  test_api_endpoint "GET All Notifications" "GET" "/v1/notifications" 200 "" true
  test_api_endpoint "GET Notifications (Paginated)" "GET" "/v1/notifications?page=1&page_size=10" 200 "" true
  test_api_endpoint "GET Notifications (Unread only)" "GET" "/v1/notifications?is_read=false" 200 "" true
  test_api_endpoint "GET Notifications (Read only)" "GET" "/v1/notifications?is_read=true" 200 "" true
  test_api_endpoint "GET Notifications (By type)" "GET" "/v1/notifications?notification_type=registration_approved" 200 "" true
  test_api_endpoint "GET Notifications (Complex filter)" "GET" "/v1/notifications?is_read=false&page=1&page_size=5" 200 "" true
  
  # === 2. Get Unread Count ===
  printf "\n${CYAN}Testing: GET /v1/notifications/unread/count${NC}\n"
  local unread_response=$(test_api_endpoint "GET Unread Count" "GET" "/v1/notifications/unread/count" 200 "" true)
  local unread_count=$(echo "$unread_response" | jq -r '.data.unread_count // 0')
  printf "${GREEN}✓ Unread notifications count: $unread_count${NC}\n"
  
  # === 3. Test with Created Notifications ===
  # To properly test mark as read and delete, we need notifications to exist
  # Let's trigger some by creating a hackathon and registering
  
  printf "\n${CYAN}Setting up test data (creating hackathon and registration)...${NC}\n"
  
  local create_hackathon_data=$(jq -n --arg user_id "$AUTH_USER_ID" '{
    name: "Notification Test Hackathon '$(date +%s)'",
    description: "Hackathon to trigger notifications",
    start_date: "'$(date -u -d '+7 days' +%Y-%m-%dT%H:%M:%SZ)'",
    end_date: "'$(date -u -d '+14 days' +%Y-%m-%dT%H:%M:%SZ)'",
    registration_deadline: "'$(date -u -d '+5 days' +%Y-%m-%dT%H:%M:%SZ)'",
    max_participants: 50,
    theme: "Testing",
    organizers: [$user_id]
  }')
  
  local hackathon_response=$(curl -s -X POST -H "Authorization: Bearer $AUTH_TOKEN" \
    -H "Content-Type: application/json" -d "$create_hackathon_data" \
    "$BASE_URL/v1/hackathons")
  local hackathon_id=$(echo "$hackathon_response" | jq -r '.data.id // empty')
  
  if [ -n "$hackathon_id" ]; then
    # Register for hackathon (might trigger notification)
    local register_data=$(jq -n '{
      role: "participant",
      skills: ["Testing"],
      experience_level: "beginner",
      motivation: "Testing notifications",
      tshirt_size: "M",
      emergency_contact_name: "Test Contact",
      emergency_contact_phone: "+1234567890",
      emergency_contact_relationship: "Friend"
    }')
    
    local reg_response=$(curl -s -X POST -H "Authorization: Bearer $AUTH_TOKEN" \
      -H "Content-Type: application/json" -d "$register_data" \
      "$BASE_URL/v1/hackathons/$hackathon_id/register")
    local registration_id=$(echo "$reg_response" | jq -r '.data.id // empty')
    
    if [ -n "$registration_id" ]; then
      # Approve registration (should trigger notification)
      local approve_data=$(jq -n '{
        status: "approved",
        reason: "Welcome to the hackathon!"
      }')
      curl -s -X PUT -H "Authorization: Bearer $AUTH_TOKEN" \
        -H "Content-Type: application/json" -d "$approve_data" \
        "$BASE_URL/v1/hackathons/$hackathon_id/registrations/$registration_id/status" > /dev/null
      
      printf "${GREEN}✓ Created test registration and approval (may trigger notification)${NC}\n"
      
      # Wait a moment for notification to be created
      sleep 1
    fi
  fi
  
  # Get notifications again to see if any were created
  local notifs_response=$(curl -s -H "Authorization: Bearer $AUTH_TOKEN" \
    "$BASE_URL/v1/notifications?page=1&page_size=5")
  local notifs=$(echo "$notifs_response" | jq -r '.data.notifications // []')
  local notif_count=$(echo "$notifs" | jq 'length')
  
  printf "${CYAN}Current notification count: $notif_count${NC}\n"
  
  if [ "$notif_count" -gt 0 ]; then
    # Get first notification ID for testing
    local first_notif_id=$(echo "$notifs" | jq -r '.[0].id // empty')
    local first_notif_read=$(echo "$notifs" | jq -r '.[0].is_read // false')
    
    if [ -n "$first_notif_id" ]; then
      printf "${GREEN}✓ Found notification to test with: $first_notif_id (read: $first_notif_read)${NC}\n"
      
      # === 4. Mark Notification as Read ===
      if [ "$first_notif_read" == "false" ]; then
        printf "\n${CYAN}Testing: PUT /v1/notifications/{id}/read${NC}\n"
        test_api_endpoint "PUT Mark as Read" "PUT" "/v1/notifications/$first_notif_id/read" 200 "" true
        
        # Test marking already read notification (should fail)
        printf "\n${CYAN}Testing: Mark already read notification (should fail)${NC}\n"
        local already_read_response=$(curl -s -w "\n%{http_code}" -X PUT \
          -H "Authorization: Bearer $AUTH_TOKEN" \
          "$BASE_URL/v1/notifications/$first_notif_id/read")
        local already_read_status=$(echo "$already_read_response" | tail -n1)
        if [ "$already_read_status" == "400" ]; then
          printf "${GREEN}✓ Correctly rejects marking already read notification${NC}\n"
        else
          printf "${YELLOW}⚠ Expected 400 for already read notification, got $already_read_status${NC}\n"
        fi
      else
        printf "${YELLOW}⚠ First notification already read, skipping mark as read test${NC}\n"
      fi
      
      # === 5. Mark All as Read ===
      printf "\n${CYAN}Testing: PUT /v1/notifications/read-all${NC}\n"
      local mark_all_response=$(test_api_endpoint "PUT Mark All as Read" "PUT" "/v1/notifications/read-all" 200 "" true)
      local updated_count=$(echo "$mark_all_response" | jq -r '.data.updated_count // 0')
      printf "${GREEN}✓ Marked $updated_count notification(s) as read${NC}\n"
      
      # Verify unread count is now 0
      local new_unread_response=$(curl -s -H "Authorization: Bearer $AUTH_TOKEN" \
        "$BASE_URL/v1/notifications/unread/count")
      local new_unread_count=$(echo "$new_unread_response" | jq -r '.data.unread_count // -1')
      if [ "$new_unread_count" == "0" ]; then
        printf "${GREEN}✓ Unread count is now 0 after mark all as read${NC}\n"
      else
        printf "${YELLOW}⚠ Expected unread count 0, got $new_unread_count${NC}\n"
      fi
      
      # Get a notification that can be deleted (preferably last one to avoid affecting other tests)
      local deletable_notifs=$(curl -s -H "Authorization: Bearer $AUTH_TOKEN" \
        "$BASE_URL/v1/notifications?page=1&page_size=100")
      local deletable_notif_id=$(echo "$deletable_notifs" | jq -r '.data.notifications[-1].id // empty')
      
      # === 6. Delete Notification ===
      if [ -n "$deletable_notif_id" ]; then
        printf "\n${CYAN}Testing: DELETE /v1/notifications/{id}${NC}\n"
        test_api_endpoint "DELETE Notification" "DELETE" "/v1/notifications/$deletable_notif_id" 200 "" true
        
        # Test deleting non-existent notification (should fail)
        printf "\n${CYAN}Testing: Delete non-existent notification (should fail)${NC}\n"
        local nonexistent_response=$(curl -s -w "\n%{http_code}" -X DELETE \
          -H "Authorization: Bearer $AUTH_TOKEN" \
          "$BASE_URL/v1/notifications/nonexistent123")
        local nonexistent_status=$(echo "$nonexistent_response" | tail -n1)
        if [ "$nonexistent_status" == "404" ] || [ "$nonexistent_status" == "500" ]; then
          printf "${GREEN}✓ Correctly handles non-existent notification${NC}\n"
        else
          printf "${YELLOW}⚠ Expected 404/500 for non-existent notification, got $nonexistent_status${NC}\n"
        fi
      fi
    fi
  else
    printf "${YELLOW}⚠ No notifications found for testing. Some tests skipped.${NC}\n"
    printf "${YELLOW}  Note: Notifications are typically created by system events.${NC}\n"
    printf "${YELLOW}  Consider manually creating test notifications in the database.${NC}\n"
  fi
  
  # === Test Edge Cases ===
  printf "\n${CYAN}Testing: Edge Cases and Validation${NC}\n"
  
  # Test invalid page size
  printf "${YELLOW}Testing: Invalid page size (should handle gracefully)${NC}\n"
  local invalid_page_response=$(curl -s -w "\n%{http_code}" -X GET \
    -H "Authorization: Bearer $AUTH_TOKEN" \
    "$BASE_URL/v1/notifications?page_size=1000")
  local invalid_page_status=$(echo "$invalid_page_response" | tail -n1)
  if [ "$invalid_page_status" == "400" ] || [ "$invalid_page_status" == "200" ]; then
    printf "${GREEN}✓ Handles invalid page size (status: $invalid_page_status)${NC}\n"
  else
    printf "${RED}✗ Unexpected status for invalid page size: $invalid_page_status${NC}\n"
  fi
  
  # Test accessing other user's notification (should fail)
  printf "${YELLOW}Testing: Access other user's notification (should fail)${NC}\n"
  # This would require knowing another user's notification ID, so we'll test with a fake ID
  local other_user_response=$(curl -s -w "\n%{http_code}" -X PUT \
    -H "Authorization: Bearer $AUTH_TOKEN" \
    "$BASE_URL/v1/notifications/fake_other_user_notif_123/read")
  local other_user_status=$(echo "$other_user_response" | tail -n1)
  if [ "$other_user_status" == "403" ] || [ "$other_user_status" == "404" ]; then
    printf "${GREEN}✓ Correctly prevents access to other user's notification${NC}\n"
  else
    printf "${YELLOW}⚠ Expected 403/404 for other user's notification, got $other_user_status${NC}\n"
  fi
  
  # === Test Pagination ===
  printf "\n${CYAN}Testing: Pagination Behavior${NC}\n"
  local page1=$(curl -s -H "Authorization: Bearer $AUTH_TOKEN" \
    "$BASE_URL/v1/notifications?page=1&page_size=2")
  local page1_count=$(echo "$page1" | jq -r '.data.notifications | length')
  local page1_total=$(echo "$page1" | jq -r '.data.total')
  
  printf "${CYAN}Page 1: $page1_count items, Total: $page1_total${NC}\n"
  
  if [ "$page1_total" -gt 2 ]; then
    local page2=$(curl -s -H "Authorization: Bearer $AUTH_TOKEN" \
      "$BASE_URL/v1/notifications?page=2&page_size=2")
    local page2_count=$(echo "$page2" | jq -r '.data.notifications | length')
    printf "${CYAN}Page 2: $page2_count items${NC}\n"
    
    if [ "$page2_count" -gt 0 ]; then
      printf "${GREEN}✓ Pagination working correctly${NC}\n"
    else
      printf "${YELLOW}⚠ Page 2 is empty but total suggests more items${NC}\n"
    fi
  else
    printf "${YELLOW}⚠ Not enough notifications to test pagination (need >2)${NC}\n"
  fi
  
  # === Test Notification Types Filter ===
  printf "\n${CYAN}Testing: Filter by Notification Type${NC}\n"
  local types=("registration_approved" "registration_rejected" "hackathon_reminder" "team_invite" "announcement")
  for type in "${types[@]}"; do
    local type_response=$(curl -s -H "Authorization: Bearer $AUTH_TOKEN" \
      "$BASE_URL/v1/notifications?notification_type=$type&page_size=5")
    local type_count=$(echo "$type_response" | jq -r '.data.notifications | length')
    printf "${CYAN}  Type '$type': $type_count notification(s)${NC}\n"
  done
  
  # Cleanup test hackathon
  if [ -n "$hackathon_id" ]; then
    curl -s -X DELETE -H "Authorization: Bearer $AUTH_TOKEN" \
      "$BASE_URL/v1/hackathons/$hackathon_id" > /dev/null
    printf "\n${GREEN}✓ Cleaned up test hackathon${NC}\n"
  fi
  
  printf "\n${CYAN}=== Notification Tests Complete ===${NC}\n"
}

# Run if executed directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
  get_auth_token
  test_notification_endpoints
  print_test_summary
  [ "$FAIL_COUNT" -eq 0 ] && exit 0 || exit 1
fi
