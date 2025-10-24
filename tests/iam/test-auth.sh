#!/bin/bash

# ==============================================================================
# IAM Tests - Authentication Endpoints
# ==============================================================================

source "$(dirname "$0")/../common/test-common.sh"

test_authentication_endpoints() {
  printf "\n${CYAN}=== Testing Authentication Endpoints ===${NC}\n"
  
  # Valid login
  get_auth_token
  
  # Invalid login
  local invalid_login
  invalid_login=$(jq -n '{email: "invalid@example.com", password: "wrongpassword"}')
  test_api_endpoint "Invalid Login Test" "POST" "/v1/auth/login" 401 "$invalid_login"
  
  # Mentor login
  local mentor_login=$(jq -n '{email: "mentor@example.com", password: "password"}')
  test_api_endpoint "Mentor Login" "POST" "/v1/auth/login-mentor" 200 "$mentor_login" false
  
  # Forgot password
  local forgot_password_data
  forgot_password_data=$(jq -n --arg email "admin@example.com" '{email: $email}')
  test_api_endpoint "Forgot Password Test" "POST" "/v1/auth/forgot" 200 "$forgot_password_data"
  
  # Invalid new password (invalid token)
  local new_password_data
  new_password_data=$(jq -n --arg token "some_reset_token" --arg pass "newpassword123!A" '{token: $token, password: $pass}')
  test_api_endpoint "New Password Test (Invalid Token)" "POST" "/v1/auth/new-password" 400 "$new_password_data"
  
  # Refresh token
  local refresh_token=$(curl -s -X POST -H "Content-Type: application/json" \
    -d "$(jq -n '{email: "admin@example.com", password: "password"}')" \
    "$BASE_URL/v1/auth/login" | jq -r '.data.token.refresh_token // empty')
  
  if [ -n "$refresh_token" ]; then
    local refresh_data
    refresh_data=$(jq -n --arg token "$refresh_token" '{refresh_token: $token}')
    test_api_endpoint "Refresh Token Test" "POST" "/v1/auth/refresh" 200 "$refresh_data"
  else
    write_test_log "WARN" "✗ Refresh Token Test - Dilewati: Refresh token tidak tersedia dari login"
  fi
  
  # Resend OTP
  local resend_data=$(jq -n '{email: "admin@example.com"}')
  test_api_endpoint "Resend OTP" "POST" "/v1/auth/send-otp" 200 "$resend_data" false
  
  # Logout (skip - endpoint may not exist)
  # test_api_endpoint "Logout" "POST" "/v1/auth/logout" 200 "" true
}

# Run if executed directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
  get_auth_token
  test_authentication_endpoints
  print_test_summary
  [ "$FAIL_COUNT" -eq 0 ] && exit 0 || exit 1
fi
