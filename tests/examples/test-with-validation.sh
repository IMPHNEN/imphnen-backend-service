#!/bin/bash

# ==============================================================================
# Example: API Tests with Response Content Validation
# ==============================================================================

source "$(dirname "$0")/../common/test-common.sh"

# ==============================================================================
# Validation Functions Examples
# ==============================================================================

# Example 1: Validate login response structure
validate_login_response() {
  local response=$1
  
  # Check required structure
  assert_response_structure "$response" "data" "version" || return 1
  
  # Check token exists
  assert_json_field "$response" ".data.token.access_token" || return 1
  
  # Check user data exists
  assert_json_field "$response" ".data.user.id" || return 1
  assert_json_field "$response" ".data.user.email" || return 1
  
  # Check version format
  assert_json_contains "$response" '.version | test("^[0-9]+\\.[0-9]+\\.[0-9]+$")' "version has semver format" || return 1
  
  return 0
}

# Example 2: Validate user list response
validate_user_list_response() {
  local response=$1
  
  # Check response structure
  assert_response_structure "$response" "data" "meta" "version" || return 1
  
  # Check data is array
  assert_json_contains "$response" '.data | type == "array"' "data is array" || return 1
  
  # Check meta has pagination
  assert_json_field "$response" ".meta.page" || return 1
  assert_json_field "$response" ".meta.per_page" || return 1
  
  return 0
}

# Example 3: Validate user creation response
validate_user_created() {
  local response=$1
  
  # Check success message
  assert_json_field "$response" ".message" || return 1
  
  # Optionally check specific message text
  local message
  message=$(echo "$response" | jq -r '.message')
  if [[ ! "$message" =~ "Success" ]]; then
    echo "ERROR: Message should contain 'Success', got: $message"
    return 1
  fi
  
  return 0
}

# Example 4: Validate specific field values
validate_mentor_login() {
  local response=$1
  
  # Check user role is Mentor
  assert_json_field "$response" ".data.user.role.name" "Mentor" || return 1
  
  # Check user has permissions
  assert_json_contains "$response" '.data.user.role.permissions | length > 0' "mentor has permissions" || return 1
  
  return 0
}

# Example 5: Validate error response
validate_error_response() {
  local response=$1
  
  # Check error field exists
  assert_json_field "$response" ".error" || return 1
  
  # Check version exists even in error
  assert_json_field "$response" ".version" || return 1
  
  return 0
}

# Example 6: Validate pagination metadata
validate_pagination() {
  local response=$1
  local expected_page=$2
  local expected_per_page=$3
  
  # Check pagination values
  assert_json_field "$response" ".meta.page" "$expected_page" || return 1
  assert_json_field "$response" ".meta.per_page" "$expected_per_page" || return 1
  
  return 0
}

# ==============================================================================
# Test Cases with Validation
# ==============================================================================

test_auth_with_validation() {
  printf "\n${CYAN}=== Testing Authentication with Response Validation ===${NC}\n"
  
  # Test 1: Valid login with full response validation
  local login_data
  login_data=$(jq -n '{email: "admin@example.com", password: "password"}')
  test_api_endpoint \
    "Login with Response Validation" \
    "POST" \
    "/v1/auth/login" \
    200 \
    "$login_data" \
    false \
    validate_login_response
  
  # Test 2: Invalid login with error validation
  local invalid_login
  invalid_login=$(jq -n '{email: "invalid@example.com", password: "wrong"}')
  test_api_endpoint \
    "Invalid Login with Error Validation" \
    "POST" \
    "/v1/auth/login" \
    401 \
    "$invalid_login" \
    false \
    validate_error_response
  
  # Test 3: Mentor login with role validation
  local mentor_login
  mentor_login=$(jq -n '{email: "mentor@example.com", password: "password"}')
  test_api_endpoint \
    "Mentor Login with Role Validation" \
    "POST" \
    "/v1/auth/login-mentor" \
    200 \
    "$mentor_login" \
    false \
    validate_mentor_login
}

test_users_with_validation() {
  printf "\n${CYAN}=== Testing Users with Response Validation ===${NC}\n"
  
  get_auth_token
  
  # Test 1: Get users list with structure validation
  test_api_endpoint \
    "GET Users with List Validation" \
    "GET" \
    "/v1/users" \
    200 \
    "" \
    true \
    validate_user_list_response
  
  # Test 2: Get users with pagination validation
  validate_users_page1() {
    validate_pagination "$1" "1" "10"
  }
  test_api_endpoint \
    "GET Users Page 1 with Pagination Validation" \
    "GET" \
    "/v1/users?page=1&limit=10" \
    200 \
    "" \
    true \
    validate_users_page1
  
  # Test 3: Create user with success message validation
  local new_user
  new_user=$(jq -n '{
    fullname: "Test User with Validation",
    email: "testvalidation@example.com",
    password: "Password@123",
    role_id: "5713cb37-dc02-4e87-8048-d7a41d352059"
  }')
  test_api_endpoint \
    "Create User with Success Validation" \
    "POST" \
    "/v1/users/create" \
    201 \
    "$new_user" \
    true \
    validate_user_created
}

# ==============================================================================
# Inline Validation Examples
# ==============================================================================

test_inline_validation() {
  printf "\n${CYAN}=== Testing with Inline Validation Functions ===${NC}\n"
  
  get_auth_token
  
  # Inline validation function for role detail
  validate_role_detail() {
    local response=$1
    assert_response_structure "$response" "data" "version" || return 1
    assert_json_field "$response" ".data.id" || return 1
    assert_json_field "$response" ".data.name" || return 1
    assert_json_contains "$response" '.data.permissions | type == "array"' "has permissions array" || return 1
    return 0
  }
  
  test_api_endpoint \
    "GET Role Detail with Inline Validation" \
    "GET" \
    "/v1/roles/detail/5713cb37-dc02-4e87-8048-d7a41d352059" \
    200 \
    "" \
    true \
    validate_role_detail
  
  # Another inline example
  validate_gacha_items() {
    local response=$1
    assert_json_contains "$response" '.data | type == "array"' "data is array" || return 1
    assert_json_contains "$response" '.data | length > 0' "data has items" || return 1
    return 0
  }
  
  test_api_endpoint \
    "GET Gacha Items with Data Validation" \
    "GET" \
    "/v1/gacha/items?page=1&per_page=10" \
    200 \
    "" \
    true \
    validate_gacha_items
}

# ==============================================================================
# Run Tests
# ==============================================================================

main() {
  printf "${CYAN}"
  printf "╔═══════════════════════════════════════════════════════════════════════╗\n"
  printf "║         API Tests with Response Content Validation Examples          ║\n"
  printf "╚═══════════════════════════════════════════════════════════════════════╝\n"
  printf "${NC}\n"
  
  test_auth_with_validation
  test_users_with_validation
  test_inline_validation
  
  print_test_summary
  
  [ "$FAIL_COUNT" -eq 0 ] && exit 0 || exit 1
}

# Run if executed directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
  main
fi
