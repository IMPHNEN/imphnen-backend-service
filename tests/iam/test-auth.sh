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
  
  # Security: Test SQL injection in login
  local sql_injection_login=$(jq -n '{email: "admin@example.com\" OR \"1\"=\"1", password: "password"}')
  test_api_endpoint "SQL Injection in Login Email (Should Fail)" "POST" "/v1/auth/login" 400 "$sql_injection_login"
  
  local sql_injection_pass=$(jq -n '{email: "admin@example.com", password: "password\" OR \"1\"=\"1"}')
  test_api_endpoint "SQL Injection in Login Password (Should Fail)" "POST" "/v1/auth/login" 401 "$sql_injection_pass"
  
  # Security: Test XSS in login
  local xss_login=$(jq -n '{email: "<script>alert(\"XSS\")</script>", password: "password"}')
  test_api_endpoint "XSS in Login Email (Should Fail)" "POST" "/v1/auth/login" 400 "$xss_login"
  
  # Security: Test empty credentials
  local empty_login=$(jq -n '{email: "", password: ""}')
  test_api_endpoint "Empty Credentials (Should Fail)" "POST" "/v1/auth/login" 400 "$empty_login"
  
  # Security: Test missing fields
  local missing_password=$(jq -n '{email: "admin@example.com"}')
  test_api_endpoint "Missing Password (Should Fail)" "POST" "/v1/auth/login" 422 "$missing_password"
  
  # Mentor login
  local mentor_login=$(jq -n '{email: "mentor@example.com", password: "password"}')
  test_api_endpoint "Mentor Login" "POST" "/v1/auth/login-mentor" 200 "$mentor_login" false
  
  # Security: Test invalid mentor login
  local invalid_mentor=$(jq -n '{email: "nonexistent@example.com", password: "wrongpass"}')
  test_api_endpoint "Invalid Mentor Login (Should Fail)" "POST" "/v1/auth/login-mentor" 401 "$invalid_mentor"
  
  # Forgot password
  local forgot_password_data
  forgot_password_data=$(jq -n --arg email "admin@example.com" '{email: $email}')
  test_api_endpoint "Forgot Password Test" "POST" "/v1/auth/forgot" 200 "$forgot_password_data"
  
  # Security: Test forgot password with invalid email
  local invalid_forgot=$(jq -n '{email: "not_an_email"}')
  test_api_endpoint "Forgot Password with Invalid Email (Should Fail)" "POST" "/v1/auth/forgot" 400 "$invalid_forgot"
  
  # Security: Test forgot password with non-existent email (should not reveal if user exists)
  local nonexistent_forgot=$(jq -n '{email: "nonexistent@example.com"}')
  test_api_endpoint "Forgot Password with Non-existent Email" "POST" "/v1/auth/forgot" 200 "$nonexistent_forgot"
  
  # Invalid new password (invalid token)
  local new_password_data
  new_password_data=$(jq -n --arg token "some_reset_token" --arg pass "newpassword123!A" '{token: $token, password: $pass}')
  test_api_endpoint "New Password Test (Invalid Token)" "POST" "/v1/auth/new-password" 400 "$new_password_data"
  
  # Security: Test weak password in reset
  local weak_reset=$(jq -n --arg token "some_reset_token" '{token: $token, password: "123456"}')
  test_api_endpoint "New Password with Weak Password (Should Fail)" "POST" "/v1/auth/new-password" 400 "$weak_reset"
  
  # Refresh token
  local refresh_token=$(curl -s -X POST -H "Content-Type: application/json" \
    -d "$(jq -n '{email: "admin@example.com", password: "password"}')" \
    "$BASE_URL/v1/auth/login" | jq -r '.data.token.refresh_token // empty')
  
  if [ -n "$refresh_token" ]; then
    local refresh_data
    refresh_data=$(jq -n --arg token "$refresh_token" '{refresh_token: $token}')
    test_api_endpoint "Refresh Token Test" "POST" "/v1/auth/refresh" 200 "$refresh_data"
    
    # Security: Test invalid refresh token
    local invalid_refresh=$(jq -n '{refresh_token: "invalid_token_12345"}')
    test_api_endpoint "Invalid Refresh Token (Should Fail)" "POST" "/v1/auth/refresh" 401 "$invalid_refresh"
    
    # Security: Test expired/malformed refresh token
    local malformed_refresh=$(jq -n '{refresh_token: "Bearer.malformed.token"}')
    test_api_endpoint "Malformed Refresh Token (Should Fail)" "POST" "/v1/auth/refresh" 401 "$malformed_refresh"
  else
    write_test_log "WARN" "✗ Refresh Token Test - Dilewati: Refresh token tidak tersedia dari login"
  fi
  
  # Resend OTP
  local resend_data=$(jq -n '{email: "admin@example.com"}')
  test_api_endpoint "Resend OTP" "POST" "/v1/auth/send-otp" 200 "$resend_data" false
  
  # Security: Test resend OTP with invalid email
  local invalid_otp=$(jq -n '{email: "not_an_email"}')
  test_api_endpoint "Resend OTP with Invalid Email (Should Fail)" "POST" "/v1/auth/send-otp" 400 "$invalid_otp"
  
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
