#!/bin/bash

# ==============================================================================
# IAM Tests - User Management Endpoints
# ==============================================================================

source "$(dirname "$0")/../common/test-common.sh"

test_user_management_endpoints() {
  printf "\n${CYAN}=== Testing User Management Endpoints ===${NC}\n"
  
  # Get users list
  test_api_endpoint "GET Users List" "GET" "/v1/users" 200 "" true
  test_api_endpoint "GET Users (Paginated)" "GET" "/v1/users?page=1&limit=10" 200 "" true
  test_api_endpoint "GET Users (Search)" "GET" "/v1/users?search=admin" 200 "" true
  test_api_endpoint "GET Users (Sorted)" "GET" "/v1/users?sort_by=created_at&order=DESC" 200 "" true
  
  # Get user me
  test_api_endpoint "GET User Me" "GET" "/v1/users/me" 200 "" true
  
  # Update user me - use correct endpoint /update/me
  local update_me_data=$(jq -n '{
    fullname: "Updated Admin User",
    phone_number: "081234567890",
    gender: "male",
    birthdate: "1990-01-01"
  }')
  test_api_endpoint "PUT User Me" "PUT" "/v1/users/update/me" 200 "$update_me_data" true
  
  # Get user by ID
  local test_user_id="c3b1d6a8-8d4f-4b36-b789-2e532ec7a7b2"
  test_api_endpoint "GET User By ID" "GET" "/v1/users/detail/$test_user_id" 200 "" true
  
  # Create new user
  local new_user_email="test_user_$(date +%s)@example.com"
  local create_user_data=$(jq -n \
    --arg email "$new_user_email" \
    --arg pass "TestPassword123!" \
    --arg fullname "Test User $(date +%s)" \
    --arg phone "089876543211" \
    '{
      email: $email,
      password: $pass,
      fullname: $fullname,
      phone_number: $phone,
      is_active: true,
      role_id: "5713cb37-dc02-4e87-8048-d7a41d352059"
    }')
  
  local create_response=$(test_api_endpoint "POST Create User" "POST" "/v1/users/create" 201 "$create_user_data" true)
  local created_user_id=$(echo "$create_response" | jq -r '.data.id // empty')
  
  if [ -n "$created_user_id" ]; then
    # Update user
    local update_user_data=$(jq -n \
      --arg email "updated_$new_user_email" \
      --arg fullname "Updated Test User" \
      '{
        email: $email,
        fullname: $fullname,
        phone_number: "089876543212",
        is_active: true,
        gender: "Female",
        birthdate: "1995-05-15",
        role_id: "5713cb37-dc02-4e87-8048-d7a41d352059"
      }')
    test_api_endpoint "PUT Update User" "PUT" "/v1/users/update/$created_user_id" 200 "$update_user_data" true
    
    # Deactivate user - endpoint uses PUT, not PATCH
    local deactivate_data=$(jq -n '{is_active: false}')
    test_api_endpoint "PUT Deactivate User" "PUT" "/v1/users/activate/$created_user_id" 200 "$deactivate_data" true
    
    # Reactivate user - endpoint uses PUT, not PATCH
    local reactivate_data=$(jq -n '{is_active: true}')
    test_api_endpoint "PUT Reactivate User" "PUT" "/v1/users/activate/$created_user_id" 200 "$reactivate_data" true
    
    # Delete user
    test_api_endpoint "DELETE User" "DELETE" "/v1/users/delete/$created_user_id" 200 "" true
  else
    write_test_log "WARN" "Skipping user update/delete tests - failed to create user"
  fi
}

# Run if executed directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
  get_auth_token
  test_user_management_endpoints
  print_test_summary
  [ "$FAIL_COUNT" -eq 0 ] && exit 0 || exit 1
fi
