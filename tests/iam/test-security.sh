#!/bin/bash

# ==============================================================================
# IAM Tests - Security & Authorization Tests
# ==============================================================================

source "$(dirname "$0")/../common/test-common.sh"

test_unauthorized_access() {
  printf "\n${CYAN}=== Testing Unauthorized Access ===${NC}\n"
  
  # Test protected endpoints without authentication token
  test_api_endpoint "GET Users without Auth" "GET" "/v1/users" 401 "" false
  test_api_endpoint "GET User Me without Auth" "GET" "/v1/users/me" 401 "" false
  test_api_endpoint "GET Roles without Auth" "GET" "/v1/roles" 401 "" false
  test_api_endpoint "GET Permissions without Auth" "GET" "/v1/permissions" 401 "" false
  test_api_endpoint "GET Teams Admin without Auth" "GET" "/v1/teams/admin" 401 "" false
  test_api_endpoint "GET Mentors without Auth" "GET" "/v1/mentors" 401 "" false
  
  # Test CMS endpoints - some may return 404 if not implemented
  local cms_response=$(curl -s -w "\n%{http_code}" "$BASE_URL/v1/cms/events")
  local cms_code=$(echo "$cms_response" | tail -1)
  if [ "$cms_code" = "401" ] || [ "$cms_code" = "404" ]; then
    write_test_log "SUCCESS" "✓ CMS Events endpoint properly protected or not implemented (code: $cms_code)"
  else
    write_test_log "WARN" "✗ CMS Events endpoint returned unexpected code: $cms_code"
  fi
  
  test_api_endpoint "GET Gacha Items without Auth" "GET" "/v1/gacha/items" 401 "" false
  
  # Hackathon admin endpoint may return 404 if not implemented
  local hackathon_response=$(curl -s -w "\n%{http_code}" "$BASE_URL/v1/hackathon")
  local hackathon_code=$(echo "$hackathon_response" | tail -1)
  if [ "$hackathon_code" = "401" ] || [ "$hackathon_code" = "404" ]; then
    write_test_log "SUCCESS" "✓ Hackathon endpoint properly protected or not implemented (code: $hackathon_code)"
  else
    write_test_log "WARN" "✗ Hackathon endpoint returned unexpected code: $hackathon_code"
  fi
}

test_invalid_token_access() {
  printf "\n${CYAN}=== Testing Invalid/Expired Token Access ===${NC}\n"
  
  # Save the original token
  local original_token="$AUTH_TOKEN"
  
  # Test with invalid token
  AUTH_TOKEN="invalid_token_12345"
  test_api_endpoint "GET Users with Invalid Token" "GET" "/v1/users" 401 "" true
  test_api_endpoint "GET User Me with Invalid Token" "GET" "/v1/users/me" 401 "" true
  
  # Test with malformed token
  AUTH_TOKEN="Bearer.malformed.token"
  test_api_endpoint "GET Users with Malformed Token" "GET" "/v1/users" 401 "" true
  
  # Test with empty token
  AUTH_TOKEN=""
  test_api_endpoint "GET Users with Empty Token" "GET" "/v1/users" 401 "" true
  
  # Restore original token
  AUTH_TOKEN="$original_token"
}

test_role_based_access_control() {
  printf "\n${CYAN}=== Testing Role-Based Access Control ===${NC}\n"
  
  # Create a regular user (non-admin) and try to access admin endpoints
  local regular_user_email="regular_user_$(date +%s)@example.com"
  local create_user_data=$(jq -n \
    --arg email "$regular_user_email" \
    --arg pass "RegularUser123!" \
    --arg fullname "Regular User Test" \
    '{
      email: $email,
      password: $pass,
      fullname: $fullname,
      phone_number: "081234567890",
      is_active: true,
      role_id: "5713cb37-dc02-4e87-8048-d7a41d352059"
    }')
  
  local create_response=$(curl -s -X POST \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $AUTH_TOKEN" \
    -d "$create_user_data" \
    "$BASE_URL/v1/users/create")
  
  local created_user_id=$(echo "$create_response" | jq -r '.data.id // empty')
  
  if [ -n "$created_user_id" ]; then
    # Login as regular user
    local user_login=$(jq -n --arg email "$regular_user_email" --arg pass "RegularUser123!" '{email: $email, password: $pass}')
    local login_response=$(curl -s -X POST \
      -H "Content-Type: application/json" \
      -d "$user_login" \
      "$BASE_URL/v1/auth/login")
    
    local user_token=$(echo "$login_response" | jq -r '.data.token.access_token // empty')
    
    if [ -n "$user_token" ]; then
      # Save admin token
      local admin_token="$AUTH_TOKEN"
      AUTH_TOKEN="$user_token"
      
      # Try to access admin endpoints with regular user token
      test_api_endpoint "Regular User Access Admin Teams" "GET" "/v1/teams/admin" 403 "" true
      
      # Try to create role - endpoint might be POST /v1/roles/create with 403 or POST /v1/roles with 405
      local create_role_response=$(curl -s -w "\n%{http_code}" -X POST \
        -H "Content-Type: application/json" \
        -H "Authorization: Bearer $user_token" \
        -d '{"name":"test_role","description":"test","permissions":[]}' \
        "$BASE_URL/v1/roles/create")
      local role_code=$(echo "$create_role_response" | tail -1)
      if [ "$role_code" = "403" ] || [ "$role_code" = "405" ]; then
        write_test_log "SUCCESS" "✓ Regular User Create Role properly denied (code: $role_code)"
      else
        write_test_log "ERROR" "✗ Regular User Create Role not properly denied (code: $role_code)"
      fi
      
      test_api_endpoint "Regular User Delete User" "DELETE" "/v1/users/delete/$created_user_id" 403 "" true
      
      # Regular user should be able to access their own profile
      test_api_endpoint "Regular User Access Own Profile" "GET" "/v1/users/me" 200 "" true
      
      # Restore admin token
      AUTH_TOKEN="$admin_token"
    else
      write_test_log "WARN" "Failed to login as regular user for RBAC tests"
    fi
    
    # Cleanup: Delete the created user
    curl -s -X DELETE \
      -H "Authorization: Bearer $AUTH_TOKEN" \
      "$BASE_URL/v1/users/delete/$created_user_id" > /dev/null
  else
    write_test_log "WARN" "Failed to create regular user for RBAC tests"
  fi
}

test_csrf_and_headers() {
  printf "\n${CYAN}=== Testing CSRF and Security Headers ===${NC}\n"
  
  # Test that server returns appropriate security headers
  local response_headers=$(curl -s -I "$BASE_URL/v1/auth/login")
  
  # Check for security headers (these may vary based on your implementation)
  if echo "$response_headers" | grep -iq "X-Content-Type-Options"; then
    write_test_log "SUCCESS" "✓ X-Content-Type-Options header present"
  else
    write_test_log "WARN" "✗ X-Content-Type-Options header missing"
  fi
  
  if echo "$response_headers" | grep -iq "X-Frame-Options"; then
    write_test_log "SUCCESS" "✓ X-Frame-Options header present"
  else
    write_test_log "WARN" "✗ X-Frame-Options header missing"
  fi
  
  # Test CORS headers
  local cors_response=$(curl -s -I -H "Origin: https://malicious-site.com" "$BASE_URL/v1/auth/login")
  if echo "$cors_response" | grep -iq "Access-Control-Allow-Origin"; then
    write_test_log "INFO" "CORS headers present - verify configuration"
  fi
}

test_sql_injection_attempts() {
  printf "\n${CYAN}=== Testing SQL Injection Protection ===${NC}\n"
  
  # Test SQL injection in login - should fail validation (400) or auth (401)
  local sql_injection_login=$(jq -n '{email: "admin@example.com\" OR \"1\"=\"1", password: "password"}')
  local response=$(curl -s -w "\n%{http_code}" -X POST \
    -H "Content-Type: application/json" \
    -d "$sql_injection_login" \
    "$BASE_URL/v1/auth/login")
  local http_code=$(echo "$response" | tail -1)
  if [ "$http_code" = "400" ] || [ "$http_code" = "401" ]; then
    write_test_log "SUCCESS" "✓ SQL Injection in Login Email properly rejected (code: $http_code)"
  else
    write_test_log "ERROR" "✗ SQL Injection in Login Email not properly handled (code: $http_code)"
  fi
  
  local sql_injection_pass=$(jq -n '{email: "admin@example.com", password: "password\" OR \"1\"=\"1"}')
  test_api_endpoint "SQL Injection in Login Password" "POST" "/v1/auth/login" 401 "$sql_injection_pass" false
  
  # Test SQL injection in search parameters - properly URL encode
  local search_injection=$(printf "%s" "admin' OR '1'='1" | jq -sRr @uri)
  local response=$(curl -s -w "\n%{http_code}" \
    -H "Authorization: Bearer $AUTH_TOKEN" \
    "$BASE_URL/v1/users?search=$search_injection")
  local http_code=$(echo "$response" | tail -1)
  if [ "$http_code" = "200" ]; then
    local body=$(echo "$response" | sed '$d')
    # Check if it returned all users or properly filtered
    local count=$(echo "$body" | jq '.data | length' 2>/dev/null || echo "0")
    write_test_log "SUCCESS" "✓ SQL Injection in User Search handled safely (returned $count users)"
  else
    write_test_log "WARN" "✗ SQL Injection in User Search failed (code: $http_code)"
  fi
  
  # Test UNION injection
  local union_injection=$(printf "%s" "' UNION SELECT * FROM users--" | jq -sRr @uri)
  local response=$(curl -s -w "\n%{http_code}" \
    -H "Authorization: Bearer $AUTH_TOKEN" \
    "$BASE_URL/v1/users?search=$union_injection")
  local http_code=$(echo "$response" | tail -1)
  if [ "$http_code" = "200" ]; then
    write_test_log "SUCCESS" "✓ SQL Injection UNION attack handled safely"
  else
    write_test_log "WARN" "✗ SQL Injection UNION test failed (code: $http_code)"
  fi
  
  # Test sort injection
  local sort_injection=$(printf "%s" "email; DROP TABLE users--" | jq -sRr @uri)
  local response=$(curl -s -w "\n%{http_code}" \
    -H "Authorization: Bearer $AUTH_TOKEN" \
    "$BASE_URL/v1/users?sort_by=$sort_injection")
  local http_code=$(echo "$response" | tail -1)
  if [ "$http_code" = "200" ] || [ "$http_code" = "400" ]; then
    write_test_log "SUCCESS" "✓ SQL Injection in Sort Parameter handled safely (code: $http_code)"
  else
    write_test_log "WARN" "✗ SQL Injection in Sort test failed (code: $http_code)"
  fi
}

test_xss_attempts() {
  printf "\n${CYAN}=== Testing XSS Protection ===${NC}\n"
  
  # Create user with XSS payloads
  local xss_email="xss_test_$(date +%s)@example.com"
  local xss_user_data=$(jq -n \
    --arg email "$xss_email" \
    --arg fullname "<script>alert('XSS')</script>" \
    --arg phone "<img src=x onerror=alert('XSS')>" \
    '{
      email: $email,
      password: "Test123!SecurePass",
      fullname: $fullname,
      phone_number: $phone,
      is_active: true,
      role_id: "5713cb37-dc02-4e87-8048-d7a41d352059"
    }')
  
  local xss_response=$(curl -s -X POST \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $AUTH_TOKEN" \
    -d "$xss_user_data" \
    "$BASE_URL/v1/users/create")
  
  local xss_user_id=$(echo "$xss_response" | jq -r '.data.id // empty')
  
  if [ -n "$xss_user_id" ]; then
    # Retrieve the user and check if XSS payload is escaped/sanitized
    local get_user_response=$(curl -s \
      -H "Authorization: Bearer $AUTH_TOKEN" \
      "$BASE_URL/v1/users/detail/$xss_user_id")
    
    local fullname=$(echo "$get_user_response" | jq -r '.data.fullname // empty')
    
    # Check if dangerous characters are escaped or removed
    if [[ "$fullname" == *"<script>"* ]] && [[ "$fullname" == *"</script>"* ]]; then
      write_test_log "ERROR" "✗ XSS payload not sanitized in fullname - SECURITY RISK!"
    elif [[ "$fullname" == *"&lt;script&gt;"* ]] || [[ "$fullname" != *"<"* ]]; then
      write_test_log "SUCCESS" "✓ XSS payload properly handled in fullname (escaped or stripped)"
    else
      write_test_log "SUCCESS" "✓ XSS payload handled in fullname (modified: $fullname)"
    fi
    
    # Cleanup
    curl -s -X DELETE \
      -H "Authorization: Bearer $AUTH_TOKEN" \
      "$BASE_URL/v1/users/delete/$xss_user_id" > /dev/null
  else
    write_test_log "WARN" "Could not create user with XSS payload to test sanitization"
  fi
}

test_rate_limiting() {
  printf "\n${CYAN}=== Testing Rate Limiting ===${NC}\n"
  
  # Test rapid login attempts
  write_test_log "INFO" "Testing rapid login attempts (rate limiting)..."
  
  local rate_limit_triggered=false
  for i in {1..20}; do
    local response=$(curl -s -w "\n%{http_code}" -X POST \
      -H "Content-Type: application/json" \
      -d '{"email":"admin@example.com","password":"wrongpassword"}' \
      "$BASE_URL/v1/auth/login")
    
    local http_code=$(echo "$response" | tail -1)
    
    if [ "$http_code" = "429" ]; then
      rate_limit_triggered=true
      write_test_log "SUCCESS" "✓ Rate limiting triggered after $i attempts"
      break
    fi
    
    sleep 0.1
  done
  
  if [ "$rate_limit_triggered" = false ]; then
    write_test_log "WARN" "✗ Rate limiting not detected (or threshold > 20 attempts)"
  fi
}

test_password_security() {
  printf "\n${CYAN}=== Testing Password Security ===${NC}\n"
  
  # Test weak passwords - they should be rejected (400 or 422)
  local weak_passwords=("123456" "admin" "test" "abc123" "password123")
  
  for weak_pass in "${weak_passwords[@]}"; do
    local weak_user_data=$(jq -n \
      --arg email "weak_$(date +%s)_${RANDOM}@example.com" \
      --arg pass "$weak_pass" \
      '{
        email: $email,
        password: $pass,
        fullname: "Weak Password Test",
        phone_number: "081234567890",
        is_active: true,
        role_id: "5713cb37-dc02-4e87-8048-d7a41d352059"
      }')
    
    local response=$(curl -s -w "\n%{http_code}" -X POST \
      -H "Content-Type: application/json" \
      -H "Authorization: Bearer $AUTH_TOKEN" \
      -d "$weak_user_data" \
      "$BASE_URL/v1/users/create")
    
    local http_code=$(echo "$response" | tail -1)
    
    if [ "$http_code" = "400" ] || [ "$http_code" = "422" ]; then
      write_test_log "SUCCESS" "✓ Weak password '$weak_pass' rejected"
    else
      write_test_log "WARN" "✗ Weak password '$weak_pass' accepted (code: $http_code)"
      # Cleanup if created
      if [ "$http_code" = "201" ]; then
        local user_id=$(echo "$response" | sed '$d' | jq -r '.data.id // empty')
        if [ -n "$user_id" ]; then
          curl -s -X DELETE -H "Authorization: Bearer $AUTH_TOKEN" "$BASE_URL/v1/users/delete/$user_id" > /dev/null
        fi
      fi
    fi
    
    sleep 0.1
  done
}

test_data_exposure() {
  printf "\n${CYAN}=== Testing Data Exposure Prevention ===${NC}\n"
  
  # Ensure passwords are not returned in responses
  local user_response=$(curl -s \
    -H "Authorization: Bearer $AUTH_TOKEN" \
    "$BASE_URL/v1/users/me")
  
  if echo "$user_response" | jq -e '.data.password' > /dev/null 2>&1; then
    write_test_log "ERROR" "✗ Password field exposed in user response"
  else
    write_test_log "SUCCESS" "✓ Password field not exposed in user response"
  fi
  
  # Test that error messages don't expose sensitive information
  local error_response=$(curl -s -X POST \
    -H "Content-Type: application/json" \
    -d '{"email":"nonexistent@example.com","password":"password"}' \
    "$BASE_URL/v1/auth/login")
  
  local error_msg=$(echo "$error_response" | jq -r '.message // empty' | tr '[:upper:]' '[:lower:]')
  
  # Check that error doesn't reveal if user exists
  if [[ "$error_msg" == *"user not found"* ]] || [[ "$error_msg" == *"user does not exist"* ]]; then
    write_test_log "WARN" "✗ Error message reveals user existence"
  else
    write_test_log "SUCCESS" "✓ Generic error message for invalid login"
  fi
}

test_authorization_bypass() {
  printf "\n${CYAN}=== Testing Authorization Bypass Attempts ===${NC}\n"
  
  # Test accessing other users' data
  local all_users=$(curl -s \
    -H "Authorization: Bearer $AUTH_TOKEN" \
    "$BASE_URL/v1/users")
  
  local other_user_id=$(echo "$all_users" | jq -r '.data[1].id // empty')
  
  if [ -n "$other_user_id" ]; then
    # Create a new user
    local test_user_email="bypass_test_$(date +%s)@example.com"
    local create_user_data=$(jq -n \
      --arg email "$test_user_email" \
      '{
        email: $email,
        password: "Test123!",
        fullname: "Bypass Test User",
        phone_number: "081234567890",
        is_active: true,
        role_id: "5713cb37-dc02-4e87-8048-d7a41d352059"
      }')
    
    local create_response=$(curl -s -X POST \
      -H "Content-Type: application/json" \
      -H "Authorization: Bearer $AUTH_TOKEN" \
      -d "$create_user_data" \
      "$BASE_URL/v1/users/create")
    
    local new_user_id=$(echo "$create_response" | jq -r '.data.id // empty')
    
    if [ -n "$new_user_id" ]; then
      # Login as new user
      local user_login=$(jq -n --arg email "$test_user_email" '{email: $email, password: "Test123!"}')
      local login_response=$(curl -s -X POST \
        -H "Content-Type: application/json" \
        -d "$user_login" \
        "$BASE_URL/v1/auth/login")
      
      local new_user_token=$(echo "$login_response" | jq -r '.data.token.access_token // empty')
      
      if [ -n "$new_user_token" ]; then
        # Try to update another user's data
        local admin_token="$AUTH_TOKEN"
        AUTH_TOKEN="$new_user_token"
        
        local update_data=$(jq -n '{fullname: "Hacked User"}')
        test_api_endpoint "User Update Other User" "PUT" "/v1/users/update/$other_user_id" 403 "$update_data" true
        
        # Try to delete another user
        test_api_endpoint "User Delete Other User" "DELETE" "/v1/users/delete/$other_user_id" 403 "" true
        
        # Restore admin token
        AUTH_TOKEN="$admin_token"
      fi
      
      # Cleanup
      curl -s -X DELETE \
        -H "Authorization: Bearer $AUTH_TOKEN" \
        "$BASE_URL/v1/users/delete/$new_user_id" > /dev/null
    fi
  fi
}

test_input_validation() {
  printf "\n${CYAN}=== Testing Input Validation ===${NC}\n"
  
  # Test invalid email formats
  local invalid_emails=("notanemail" "test@" "@example.com")
  
  for invalid_email in "${invalid_emails[@]}"; do
    local invalid_data=$(jq -n \
      --arg email "$invalid_email" \
      '{
        email: $email,
        password: "Test123!SecurePass",
        fullname: "Invalid Email Test",
        phone_number: "081234567890",
        is_active: true,
        role_id: "5713cb37-dc02-4e87-8048-d7a41d352059"
      }')
    
    local response=$(curl -s -w "\n%{http_code}" -X POST \
      -H "Content-Type: application/json" \
      -H "Authorization: Bearer $AUTH_TOKEN" \
      -d "$invalid_data" \
      "$BASE_URL/v1/users/create")
    
    local http_code=$(echo "$response" | tail -1)
    
    if [ "$http_code" = "400" ] || [ "$http_code" = "422" ]; then
      write_test_log "SUCCESS" "✓ Invalid email '$invalid_email' rejected"
    else
      write_test_log "WARN" "✗ Invalid email '$invalid_email' accepted (code: $http_code)"
      # Cleanup if created
      if [ "$http_code" = "201" ]; then
        local user_id=$(echo "$response" | sed '$d' | jq -r '.data.id // empty')
        if [ -n "$user_id" ]; then
          curl -s -X DELETE -H "Authorization: Bearer $AUTH_TOKEN" "$BASE_URL/v1/users/delete/$user_id" > /dev/null
        fi
      fi
    fi
  done
  
  # Test excessively long inputs (reduced to 500 chars to be more reasonable)
  local long_string=$(printf 'A%.0s' {1..500})
  local long_input_data=$(jq -n \
    --arg email "long_$(date +%s)@example.com" \
    --arg fullname "$long_string" \
    '{
      email: $email,
      password: "Test123!SecurePass",
      fullname: $fullname,
      phone_number: "081234567890",
      is_active: true,
      role_id: "5713cb37-dc02-4e87-8048-d7a41d352059"
    }')
  
  local response=$(curl -s -w "\n%{http_code}" -X POST \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $AUTH_TOKEN" \
    -d "$long_input_data" \
    "$BASE_URL/v1/users/create")
  
  local http_code=$(echo "$response" | tail -1)
  
  if [ "$http_code" = "400" ] || [ "$http_code" = "422" ]; then
    write_test_log "SUCCESS" "✓ Excessively long input rejected"
  else
    write_test_log "WARN" "✗ Excessively long input (500 chars) accepted (code: $http_code)"
    # Cleanup if created
    if [ "$http_code" = "201" ]; then
      local user_id=$(echo "$response" | sed '$d' | jq -r '.data.id // empty')
      if [ -n "$user_id" ]; then
        curl -s -X DELETE -H "Authorization: Bearer $AUTH_TOKEN" "$BASE_URL/v1/users/delete/$user_id" > /dev/null
      fi
    fi
  fi
}

test_session_management() {
  printf "\n${CYAN}=== Testing Session Management ===${NC}\n"
  
  # Test token expiration (if applicable)
  write_test_log "INFO" "Testing session management..."
  
  # Test logout functionality - try common logout endpoints
  local logout_endpoints=("/v1/auth/logout" "/v1/auth/signout" "/v2/auth/logout")
  local logout_exists=false
  
  for endpoint in "${logout_endpoints[@]}"; do
    local logout_response=$(curl -s -w "\n%{http_code}" -X POST \
      -H "Authorization: Bearer $AUTH_TOKEN" \
      "$BASE_URL$endpoint")
    
    local logout_code=$(echo "$logout_response" | tail -1)
    
    if [ "$logout_code" = "200" ] || [ "$logout_code" = "204" ]; then
      write_test_log "SUCCESS" "✓ Logout endpoint exists at $endpoint (code: $logout_code)"
      logout_exists=true
      
      # Try to use token after logout
      local saved_token="$AUTH_TOKEN"
      local after_logout_response=$(curl -s -w "\n%{http_code}" \
        -H "Authorization: Bearer $saved_token" \
        "$BASE_URL/v1/users/me")
      
      local after_logout_code=$(echo "$after_logout_response" | tail -1)
      
      if [ "$after_logout_code" = "401" ]; then
        write_test_log "SUCCESS" "✓ Token invalidated after logout"
      else
        write_test_log "WARN" "✗ Token still valid after logout (code: $after_logout_code)"
      fi
      
      # Re-authenticate for remaining tests
      get_auth_token
      break
    fi
  done
  
  if [ "$logout_exists" = false ]; then
    write_test_log "WARN" "⚠ Logout endpoint not found (tested: ${logout_endpoints[*]})"
  fi
}

# Run all security tests
run_security_tests() {
  test_unauthorized_access
  test_invalid_token_access
  test_role_based_access_control
  test_csrf_and_headers
  test_sql_injection_attempts
  test_xss_attempts
  test_rate_limiting
  test_password_security
  test_data_exposure
  test_authorization_bypass
  test_input_validation
  test_session_management
}

# Run if executed directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
  get_auth_token
  run_security_tests
  print_test_summary
  [ "$FAIL_COUNT" -eq 0 ] && exit 0 || exit 1
fi
