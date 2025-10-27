#!/bin/bash

# ==============================================================================
# IAM Tests - User Management Endpoints
# ==============================================================================

source "$(dirname "$0")/../common/test-common.sh"

test_user_management_endpoints() {
  printf "\n${CYAN}=== Testing User Management Endpoints ===${NC}\n"
  
  # Security: Test that endpoints require authentication
  local saved_token="$AUTH_TOKEN"
  AUTH_TOKEN=""
  test_api_endpoint "GET Users without Auth (Should Fail)" "GET" "/v1/users" 401 "" false
  AUTH_TOKEN="$saved_token"
  
  # Get users list
  test_api_endpoint "GET Users List" "GET" "/v1/users" 200 "" true
  test_api_endpoint "GET Users (Paginated)" "GET" "/v1/users?page=1&limit=10" 200 "" true
  test_api_endpoint "GET Users (Search)" "GET" "/v1/users?search=admin" 200 "" true
  test_api_endpoint "GET Users (Sorted)" "GET" "/v1/users?sort_by=created_at&order=DESC" 200 "" true
  
  # Security: Test SQL injection in search
  test_api_endpoint "GET Users with SQL Injection (Should Be Safe)" "GET" "/v1/users?search=' OR '1'='1" 200 "" true
  
  # Get user me
  test_api_endpoint "GET User Me" "GET" "/v1/users/me" 200 "" true
  
  # Security: Test access without token
  AUTH_TOKEN=""
  test_api_endpoint "GET User Me without Auth (Should Fail)" "GET" "/v1/users/me" 401 "" false
  AUTH_TOKEN="$saved_token"
  
  # Update user me - use correct endpoint /update/me
  local update_me_data=$(jq -n '{
    fullname: "Updated Admin User",
    phone_number: "081234567890",
    gender: "male",
    birthdate: "1990-01-01"
  }')
  test_api_endpoint "PUT User Me" "PUT" "/v1/users/update/me" 200 "$update_me_data" true
  
  # Security: Test XSS in user update
  local xss_update_data=$(jq -n '{
    fullname: "<script>alert(\"XSS\")</script>",
    phone_number: "081234567890"
  }')
  test_api_endpoint "PUT User Me with XSS (Should Be Sanitized)" "PUT" "/v1/users/update/me" 200 "$xss_update_data" true
  
  # Get user by ID
  local test_user_id="c3b1d6a8-8d4f-4b36-b789-2e532ec7a7b2"
  test_api_endpoint "GET User By ID" "GET" "/v1/users/detail/$test_user_id" 200 "" true
  
  # Security: Test access to non-existent user
  test_api_endpoint "GET Non-existent User (Should Fail)" "GET" "/v1/users/detail/00000000-0000-0000-0000-000000000000" 404 "" true
  
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
    # Security: Test duplicate email
    test_api_endpoint "POST Create Duplicate User (Should Fail)" "POST" "/v1/users/create" 400 "$create_user_data" true
    
    # Security: Test invalid email format
    local invalid_email_data=$(jq -n '{
      email: "not_an_email",
      password: "TestPassword123!",
      fullname: "Invalid Email User",
      phone_number: "089876543211",
      is_active: true,
      role_id: "5713cb37-dc02-4e87-8048-d7a41d352059"
    }')
    test_api_endpoint "POST Create User with Invalid Email (Should Fail)" "POST" "/v1/users/create" 400 "$invalid_email_data" true
    
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
    
    # Security: Test unauthorized update
    AUTH_TOKEN=""
    test_api_endpoint "PUT Update User without Auth (Should Fail)" "PUT" "/v1/users/update/$created_user_id" 401 "$update_user_data" false
    AUTH_TOKEN="$saved_token"
    
    # Deactivate user - endpoint uses PUT, not PATCH
    local deactivate_data=$(jq -n '{is_active: false}')
    test_api_endpoint "PUT Deactivate User" "PUT" "/v1/users/activate/$created_user_id" 200 "$deactivate_data" true
    
    # Reactivate user - endpoint uses PUT, not PATCH
    local reactivate_data=$(jq -n '{is_active: true}')
    test_api_endpoint "PUT Reactivate User" "PUT" "/v1/users/activate/$created_user_id" 200 "$reactivate_data" true
    
    # Delete user
    test_api_endpoint "DELETE User" "DELETE" "/v1/users/delete/$created_user_id" 200 "" true
    
    # Security: Test double delete
    test_api_endpoint "DELETE Already Deleted User (Should Fail)" "DELETE" "/v1/users/delete/$created_user_id" 404 "" true
    
    # Security: Test unauthorized delete
    AUTH_TOKEN=""
    test_api_endpoint "DELETE User without Auth (Should Fail)" "DELETE" "/v1/users/delete/$created_user_id" 401 "" false
    AUTH_TOKEN="$saved_token"
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
