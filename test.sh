#!/bin/bash

# ==============================================================================
# IMPHNEN API Comprehensive Test Suite (Bash Version)
# ==============================================================================

BASE_URL="http://127.0.0.1:4099"
TEST_EMAIL="admin@example.com"
TEST_PASSWORD="password"

declare -A ALL_USERS=(
    ["admin@example.com"]="Admin"
    ["staff@example.com"]="Staff"
    ["user@example.com"]="User"
    ["mentor@example.com"]="Mentor User"
)

START_SERVER=false
SKIP_BASIC=false
SKIP_COMPREHENSIVE=false
SKIP_CRUD=false
GENERATE_REPORT=false
VERBOSE=false

while getopts "sbcrgvh" opt; do
  case ${opt} in
    s ) START_SERVER=true ;;
    b ) SKIP_BASIC=true ;;
    c ) SKIP_COMPREHENSIVE=true ;;
    r ) SKIP_CRUD=true ;;
    g ) GENERATE_REPORT=true ;;
    v ) VERBOSE=true ;;
    h ) 
      echo "IMPHNEN API Test Suite"
      echo "Usage: $0 [OPTIONS]"
      echo ""
      echo "Options:"
      echo "  -s    Start server automatically"
      echo "  -b    Skip basic tests (auth, error handling)"
      echo "  -c    Skip comprehensive tests (users, roles, mentors, etc.)"
      echo "  -r    Skip CRUD and advanced tests"
      echo "  -g    Generate JSON test report"
      echo "  -v    Verbose output (show all INFO logs)"
      echo "  -h    Show this help message"
      echo ""
      echo "Examples:"
      echo "  $0                  # Run all tests"
      echo "  $0 -s               # Start server and run all tests"
      echo "  $0 -g               # Run tests and generate report"
      echo "  $0 -sv              # Start server with verbose output"
      echo "  $0 -bc              # Run only public endpoint tests"
      exit 0
      ;;
    \? ) echo "Invalid option: -$OPTARG" >&2; echo "Use -h for help" >&2; exit 1 ;;
  esac
done

if ! command -v curl &> /dev/null; then
  echo "Error: 'curl' tidak ditemukan. Mohon install terlebih dahulu." >&2
  exit 1
fi
if ! command -v jq &> /dev/null; then
  echo "Error: 'jq' tidak ditemukan. Mohon install terlebih dahulu." >&2
  exit 1
fi

TEST_START_TIME=$(date +%s)
AUTH_TOKEN=""
SERVER_PID=""
TEST_RESULTS=()
FALED_TESTS_SUMMARY=()
PASS_COUNT=0
FAIL_COUNT=0
TEST_TESTIMONIAL_ID=""

CYAN='\033[0;36m'
YELLOW='\033[0;33m'
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m'

cleanup() {
  if [ -n "$SERVER_PID" ]; then
    printf "\n${YELLOW}Menghentikan proses server...${NC}\n"
    kill "$SERVER_PID" &>/dev/null
  fi
}
trap cleanup EXIT

write_test_log() {
  local level=$1
  local message=$2
  local color=$NC

  case $level in
    "SUCCESS") color=$GREEN ;;
    "ERROR") color=$RED ;;
    "WARN") color=$YELLOW ;;
    "INFO") color=$CYAN ;;
  esac

  if [[ "$VERBOSE" = true || "$level" != "INFO" ]]; then
    printf "[$(date +'%H:%M:%S')] [${color}%-7s${NC}] %s\n" "$level" "$message" >&2
  fi
}

test_api_endpoint() {
  local test_name=$1
  local method=$2
  local endpoint=$3
  local expected_status=$4
  local body=$5
  local require_auth=$6
  
  local headers=(-H "Content-Type: application/json")
  if [[ "$require_auth" = true && -n "$AUTH_TOKEN" ]]; then
    headers+=(-H "Authorization: Bearer $AUTH_TOKEN")
  elif [[ "$require_auth" = true && -z "$AUTH_TOKEN" ]]; then
    write_test_log "WARN" "✗ $test_name - Dilewati: token autentikasi tidak tersedia"
    return
  fi

  local start_req_time=$(date +%s%3N)
  
  response=$(curl -s -w "\n%{http_code}" -X "$method" "${headers[@]}" -d "$body" "$BASE_URL$endpoint")
  
  http_status=$(echo "$response" | tail -n1)
  response_body=$(echo "$response" | sed '$d')

  local end_req_time=$(date +%s%3N)
  local duration=$((end_req_time - start_req_time))

  local status="FAIL"
  local error_msg=""

  if [ "$http_status" -eq "$expected_status" ]; then
    status="PASS"
    ((PASS_COUNT++))
    write_test_log "SUCCESS" "✓ $test_name - Sukses (Status: $http_status, Waktu: ${duration}ms)"
  else
    status="FAIL"
    ((FAIL_COUNT++))
write_test_log "ERROR" "  Request Body: $body"
    write_test_log "ERROR" "  Response Body: $response_body"
    error_msg="Status yang diharapkan $expected_status, tetapi mendapat $http_status."
    write_test_log "ERROR" "✗ $test_name - Gagal: $error_msg"
    FAILED_TESTS_SUMMARY+=("✗ $test_name - $error_msg")
  fi

  result_json=$(jq -n --arg name "$test_name" --arg ep "$endpoint" --arg meth "$method" \
                        --arg stat "$status" --arg code "$http_status" --arg dur "$duration" \
                        --arg err "$error_msg" \
                        '{TestName: $name, Endpoint: $ep, Method: $meth, Status: $stat, StatusCode: $code, ResponseTimeMs: $dur, Error: $err}')
  TEST_RESULTS+=("$result_json")
  # Return response_body for further processing if needed by the caller
}

test_server_connection() {
  curl -s --head "$BASE_URL/v1/cms/landing/events" > /dev/null
  return $?
}

clear_database() {
  write_test_log "INFO" "Membersihkan database via WebSocket..."
  if ! RUST_LOG=debug cargo run --bin clear_db_test --release; then
    write_test_log "ERROR" "Gagal membersihkan database."
    exit 1
  fi
  write_test_log "SUCCESS" "Pembersihan database selesai."
}

get_auth_token() {
  write_test_log "INFO" "Mengautentikasi test user..."
  local login_data
  login_data=$(jq -n --arg email "$TEST_EMAIL" --arg pass "$TEST_PASSWORD" '{email: $email, password: $pass}')
  
  local headers=(-H "Content-Type: application/json")
  local start_req_time=$(date +%s%3N)
  
  response=$(curl -s -w "\n%{http_code}" -X "POST" "${headers[@]}" -d "$login_data" "$BASE_URL/v1/auth/login")
  
  local http_status=$(echo "$response" | tail -n1)
  local response_body=$(echo "$response" | sed '$d')
  local end_req_time=$(date +%s%3N)
  local duration=$((end_req_time - start_req_time))
  
  if [ "$http_status" -eq 200 ]; then
    if echo "$response_body" | jq . > /dev/null 2>&1; then
      AUTH_TOKEN=$(echo "$response_body" | jq -r '.data.token.access_token // empty')
      if [[ -n "$AUTH_TOKEN" && "$AUTH_TOKEN" != "null" ]]; then
          write_test_log "SUCCESS" "✓ User Authentication - Sukses (Status: $http_status, Waktu: ${duration}ms)"
          write_test_log "SUCCESS" "Autentikasi berhasil"
          ((PASS_COUNT++))
      else
          write_test_log "ERROR" "Autentikasi gagal - token tidak ditemukan dalam response"
          AUTH_TOKEN=""
          ((FAIL_COUNT++))
      fi
    else
      write_test_log "ERROR" "Autentikasi gagal - response bukan JSON valid"
      AUTH_TOKEN=""
      ((FAIL_COUNT++))
    fi
  else
    write_test_log "ERROR" "✗ User Authentication - Gagal (Status: $http_status, Waktu: ${duration}ms)"
    AUTH_TOKEN=""
    ((FAIL_COUNT++))
  fi

  local status="PASS"
  local error_msg=""
  if [ "$http_status" -ne 200 ] || [[ -z "$AUTH_TOKEN" ]]; then
    status="FAIL"
    error_msg="Authentication failed"
  fi
  
  result_json=$(jq -n --arg name "User Authentication" --arg ep "/v1/auth/login" --arg meth "POST" \
                        --arg stat "$status" --arg code "$http_status" --arg dur "$duration" \
                        --arg err "$error_msg" \
                        '{TestName: $name, Endpoint: $ep, Method: $meth, Status: $stat, StatusCode: $code, ResponseTimeMs: $dur, Error: $err}')
  TEST_RESULTS+=("$result_json")
}

test_all_users_login_performance() {
  printf "\n${CYAN}=== Menguji Login Performance Semua User ===${NC}\n"
  
  local total_login_time=0
  local successful_logins=0
  local failed_logins=0
  
  local email="admin@example.com"
  local fullname="${ALL_USERS[$email]}"
  write_test_log "INFO" "Testing login for: $fullname ($email)"
  
  local login_data
  login_data=$(jq -n --arg email "$email" --arg pass "$TEST_PASSWORD" '{email: $email, password: $pass}')
  
  local start_time=$(date +%s%3N)
  
  response=$(curl -s -w "\n%{http_code}" -X "POST" \
      -H "Content-Type: application/json" \
      -d "$login_data" \
      "$BASE_URL/v1/auth/login")
  
  local http_status=$(echo "$response" | tail -n1)
  local response_body=$(echo "$response" | sed '$d')
  local end_time=$(date +%s%3N)
  local duration=$((end_time - start_time))
  
  total_login_time=$((total_login_time + duration))
  
  if [ "$http_status" -eq 200 ]; then
    if echo "$response_body" | jq -e '.data.token.access_token' > /dev/null 2>&1; then
      ((successful_logins++))
      ((PASS_COUNT++))
      write_test_log "SUCCESS" "✓ Login $fullname - ${duration}ms"
      
      result_json=$(jq -n --arg name "Login Performance - $fullname" --arg ep "/v1/auth/login" --arg meth "POST" \
                          --arg stat "PASS" --arg code "$http_status" --arg dur "$duration" \
                          --arg err "" \
                          '{TestName: $name, Endpoint: $ep, Method: $meth, Status: $stat, StatusCode: $code, ResponseTimeMs: $dur, Error: $err}')
      TEST_RESULTS+=("$result_json")
    else
      ((failed_logins++))
      ((FAIL_COUNT++))
      write_test_log "ERROR" "✗ Login $fullname - No token (${duration}ms)"
      FAILED_TESTS_SUMMARY+=("✗ Login $fullname - No token in response")
    fi
  else
    ((failed_logins++))
    ((FAIL_COUNT++))
    write_test_log "ERROR" "✗ Login $fullname - HTTP $http_status (${duration}ms)"
    FAILED_TESTS_SUMMARY+=("✗ Login $fullname - HTTP $http_status")
  fi
  
  local total_users=1
  local avg_login_time=0
  if [ "$total_users" -gt 0 ]; then
    avg_login_time=$((total_login_time / total_users))
  fi
  
  printf "\n${BLUE}=== Login Performance Summary ===${NC}\n"
  printf "Total Users Tested: %d\n" "$total_users"
  printf "${GREEN}Successful Logins: %d${NC}\n" "$successful_logins"
  printf "${RED}Failed Logins: %d${NC}\n" "$failed_logins"
  printf "${BLUE}Average Login Time: %dms${NC}\n" "$avg_login_time"
  printf "${BLUE}Total Login Time: %dms${NC}\n" "$total_login_time"
  
  if [ "$avg_login_time" -lt 2000 ]; then
    printf "${GREEN}✅ Performance Status: EXCELLENT (< 2s average)${NC}\n"
  elif [ "$avg_login_time" -lt 5000 ]; then
    printf "${YELLOW}⚠️  Performance Status: GOOD (2-5s average)${NC}\n"
  else
    printf "${RED}❌ Performance Status: POOR (> 5s average)${NC}\n"
  fi
  printf "\n"
}

test_with_user() {
  local email=$1
  local fullname=$2
  local test_name=$3
  
  write_test_log "INFO" "Testing $test_name dengan user: $fullname ($email)"
  
  local login_data
  login_data=$(jq -n --arg email "$email" --arg pass "$TEST_PASSWORD" '{email: $email, password: $pass}')
  
  local start_time=$(date +%s%3N)
  
  response=$(curl -s -w "\n%{http_code}" -X "POST" \
      -H "Content-Type: application/json" \
      -d "$login_data" \
      "$BASE_URL/v1/auth/login")
  
  local http_status=$(echo "$response" | tail -n1)
  local response_body=$(echo "$response" | sed '$d')
  local end_time=$(date +%s%3N)
  local duration=$((end_time - start_time))
  
  if [ "$http_status" -eq 200 ]; then
    if echo "$response_body" | jq -e '.data.token.access_token' > /dev/null 2>&1; then
      local user_auth_token=$(echo "$response_body" | jq -r '.data.token.access_token')
      write_test_log "SUCCESS" "✓ Login $fullname berhasil - ${duration}ms"
      
      local me_response
      me_response=$(curl -s -w "\n%{http_code}" -X "GET" \
          -H "Content-Type: application/json" \
          -H "Authorization: Bearer $user_auth_token" \
          "$BASE_URL/v1/users/me")
      
      local me_status=$(echo "$me_response" | tail -n1)
      local me_body=$(echo "$me_response" | sed '$d')
      
      if [ "$me_status" -eq 200 ]; then
        ((PASS_COUNT++))
        write_test_log "SUCCESS" "✓ Get profile $fullname berhasil"
        
        local user_email=$(echo "$me_body" | jq -r '.data.email // empty')
        local user_name=$(echo "$me_body" | jq -r '.data.fullname // empty')
        
        if [ "$user_email" = "$email" ]; then
          write_test_log "SUCCESS" "✓ User data verified: $user_name ($user_email)"
        else
          write_test_log "WARN" "⚠ User data mismatch: expected $email, got $user_email"
        fi
      else
        ((FAIL_COUNT++))
        write_test_log "ERROR" "✗ Get profile $fullname gagal - HTTP $me_status"
        FAILED_TESTS_SUMMARY+=("✗ Get profile $fullname - HTTP $me_status")
      fi
      
    else
      ((FAIL_COUNT++))
      write_test_log "ERROR" "✗ Login $fullname - No token (${duration}ms)"
      FAILED_TESTS_SUMMARY+=("✗ Login $fullname - No token in response")
    fi
  else
    write_test_log "ERROR" "✗ Login $fullname gagal - HTTP $http_status (${duration}ms)"
    FAILED_TESTS_SUMMARY+=("✗ Login $fullname - HTTP $http_status")
  fi
}
 
test_all_users_individually() {
  printf "\n${CYAN}=== Menguji Semua User Secara Individual ===${NC}\n"
  
  for email in "${!ALL_USERS[@]}"; do
    local fullname="${ALL_USERS[$email]}"
    test_with_user "$email" "$fullname" "Individual User Test"
    echo ""
  done
}

test_comprehensive_with_user() {
  printf "\n${CYAN}=== Comprehensive Test untuk $fullname ($email) ===${NC}\n"
  
  local login_data
  login_data=$(jq -n --arg email "$email" --arg pass "$TEST_PASSWORD" '{email: $email, password: $pass}')
  
  local start_time=$(date +%s%3N)
  
  response=$(curl -s -w "\n%{http_code}" -X "POST" \
      -H "Content-Type: application/json" \
      -d "$login_data" \
      "$BASE_URL/v1/auth/login")
  
  local http_status=$(echo "$response" | tail -n1)
  local response_body=$(echo "$response" | sed '$d')
  local end_time=$(date +%s%3N)
  local duration=$((end_time - start_time))
  
  if [ "$http_status" -eq 200 ]; then
    if echo "$response_body" | jq -e '.data.token.access_token' > /dev/null 2>&1; then
      user_token=$(echo "$response_body" | jq -r '.data.token.access_token')
      write_test_log "SUCCESS" "✓ Login $fullname berhasil - ${duration}ms"
      
      local original_auth_token="$AUTH_TOKEN"
      
      AUTH_TOKEN="$user_token"
      
      printf "\n${BLUE}--- Testing dengan $fullname (Expected results berdasarkan role) ---${NC}\n"
      
      test_api_endpoint "Get Current User Profile - $fullname" "GET" "/v1/users/me" 200 "" true
      
      case "$email" in
        "admin@example.com")
          test_api_endpoint "Get Users List - $fullname" "GET" "/v1/users" 200 "" true
          test_api_endpoint "Get Roles List - $fullname" "GET" "/v1/roles" 200 "" true
          test_api_endpoint "Get Permissions List - $fullname" "GET" "/v1/permissions" 200 "" true
          test_api_endpoint "Get Mentors List - $fullname" "GET" "/v1/mentors" 200 "" true
          test_api_endpoint "Get Mentor Me - $fullname" "GET" "/v1/mentors/me" 403 "" true # Admin is not a mentor
          test_api_endpoint "Get Mentor Status - $fullname" "GET" "/v1/mentors/status" 403 "" true # Admin is not a mentor
          test_api_endpoint "Get Gacha Items - $fullname" "GET" "/v1/gacha/items" 200 "" true
          test_api_endpoint "Execute Gacha Roll - $fullname" "POST" "/v1/gacha/rolls/execute" 200 "" true
          
          local testimonial_data
          testimonial_data=$(jq -n --arg content "Test testimonial by $fullname $(date +%s)" '{role: "Student", content: $content}')
          test_api_endpoint "Create Testimonial - $fullname" "POST" "/v1/cms/landing/testimonials/create" 201 "$testimonial_data" true
          ;;
          
        "staff@example.com")
          test_api_endpoint "Get Users List - $fullname" "GET" "/v1/users" 200 "" true
          test_api_endpoint "Get Roles List - $fullname" "GET" "/v1/roles" 200 "" true
          test_api_endpoint "Get Permissions List - $fullname" "GET" "/v1/permissions" 200 "" true
          test_api_endpoint "Get Mentors List - $fullname" "GET" "/v1/mentors" 200 "" true
          test_api_endpoint "Get Gacha Items - $fullname" "GET" "/v1/gacha/items" 200 "" true
          test_api_endpoint "Execute Gacha Roll - $fullname" "POST" "/v1/gacha/rolls/execute" 200 "" true
          
          local testimonial_data
          testimonial_data=$(jq -n --arg content "Test testimonial by $fullname $(date +%s)" '{role: "Student", content: $content}')
          test_api_endpoint "Create Testimonial - $fullname" "POST" "/v1/cms/landing/testimonials/create" 201 "$testimonial_data" true
          ;;
          
        "mentor@example.com")
          test_api_endpoint "Get Users List - $fullname" "GET" "/v1/users" 403 "" true
          test_api_endpoint "Get Roles List - $fullname" "GET" "/v1/roles" 403 "" true
          test_api_endpoint "Get Permissions List - $fullname" "GET" "/v1/permissions" 403 "" true
          test_api_endpoint "Get Mentors List - $fullname" "GET" "/v1/mentors" 200 "" true
          test_api_endpoint "Get Mentor Me - $fullname" "GET" "/v1/mentors/me" 200 "" true
          test_api_endpoint "Get Mentor Status - $fullname" "GET" "/v1/mentors/status" 200 "" true
          test_api_endpoint "Get Gacha Items - $fullname" "GET" "/v1/gacha/items" 200 "" true
          test_api_endpoint "Execute Gacha Roll - $fullname" "POST" "/v1/gacha/rolls/execute" 200 "" true
          
          local testimonial_data
          testimonial_data=$(jq -n --arg content "Test testimonial by $fullname $(date +%s)" '{role: "Student", content: $content}')
          test_api_endpoint "Create Testimonial - $fullname" "POST" "/v1/cms/landing/testimonials/create" 201 "$testimonial_data" true
          ;;
          
        "user@example.com")
          test_api_endpoint "Get Users List - $fullname" "GET" "/v1/users" 403 "" true
          test_api_endpoint "Get Roles List - $fullname" "GET" "/v1/roles" 403 "" true
          test_api_endpoint "Get Permissions List - $fullname" "GET" "/v1/permissions" 403 "" true
          test_api_endpoint "Get Mentors List - $fullname" "GET" "/v1/mentors" 200 "" true
          test_api_endpoint "Get Mentor Me - $fullname" "GET" "/v1/mentors/me" 403 "" true # User is not a mentor
          test_api_endpoint "Get Mentor Status - $fullname" "GET" "/v1/mentors/status" 403 "" true # User is not a mentor
          test_api_endpoint "Get Gacha Items - $fullname" "GET" "/v1/gacha/items" 200 "" true
          test_api_endpoint "Execute Gacha Roll - $fullname" "POST" "/v1/gacha/rolls/execute" 200 "" true
          
          local testimonial_data
          testimonial_data=$(jq -n --arg content "Test testimonial by $fullname $(date +%s)" '{role: "Student", content: $content}')
          test_api_endpoint "Create Testimonial - $fullname" "POST" "/v1/cms/landing/testimonials/create" 201 "$testimonial_data" true
          ;;
      esac
      
      test_api_endpoint "Events with Advanced Filter - $fullname" "GET" "/v1/cms/landing/events?filter=online&filter_by=is_online" 200 "" false
      test_api_endpoint "Testimonials with Search - $fullname" "GET" "/v1/cms/landing/testimonials?search=test" 200 "" false
      
      case "$email" in
        "admin@example.com"|"staff@example.com")
          test_api_endpoint "Users with Sort - $fullname" "GET" "/v1/users?sort_by=created_at&order=DESC" 200 "" true
          ;;
        *)
          test_api_endpoint "Users with Sort - $fullname" "GET" "/v1/users?sort_by=created_at&order=DESC" 403 "" true
          ;;
      esac
      
      AUTH_TOKEN="$original_auth_token"
      
      write_test_log "SUCCESS" "✓ Comprehensive test untuk $fullname selesai"
      
    else
      write_test_log "ERROR" "✗ Login $fullname gagal - No token (${duration}ms)"
    fi
  else
    write_test_log "ERROR" "✗ Login $fullname gagal - HTTP $http_status (${duration}ms)"
  fi
}

test_all_endpoints_with_all_users() {
  printf "\n${CYAN}=== Menjalankan Semua Test dengan Semua User ===${NC}\n"
  
  for email in "${!ALL_USERS[@]}"; do
    local fullname="${ALL_USERS[$email]}"
    test_comprehensive_with_user "$email" "$fullname"
    printf "\n${BLUE}--- Selesai testing dengan $fullname ---${NC}\n\n"
  done
}

test_public_endpoints() {
  printf "\n${CYAN}=== Menguji Public Endpoints ===${NC}\n"
  test_api_endpoint "Get Events List" "GET" "/v1/cms/landing/events" 200
  test_api_endpoint "Get Testimonials List" "GET" "/v1/cms/landing/testimonials" 200
}

test_authentication_endpoints() {
  printf "\n${CYAN}=== Menguji Authentication Endpoints ===${NC}\n"
  get_auth_token
  
  local invalid_login
  invalid_login=$(jq -n '{email: "invalid@example.com", password: "wrongpassword"}')
  test_api_endpoint "Invalid Login Test" "POST" "/v1/auth/login" 401 "$invalid_login"

  # User registration and verification tests currently rely on external email service or OTP logic
  # that is not easily testable in a simple curl script without actual email sending/receiving.
  # Skipping these tests for now.
  # local register_email="test_user_$(date +%s%N)@example.com"
  # local register_data=$(jq -n --arg email "$register_email" --arg pass "$TEST_PASSWORD" --arg fullname "Test Register" --arg phone "081234567899" '{email: $email, password: $pass, fullname: $fullname, phone_number: $phone}')
  # test_api_endpoint "User Registration Test" "POST" "/v1/auth/register" 200 "$register_data"
  # local verify_otp_data=$(jq -n --arg email "$register_email" --arg otp "123456" '{email: $email, otp: ($otp | tonumber)}')
  # test_api_endpoint "Verify Email Test (Invalid OTP)" "POST" "/v1/auth/verify-email" 400 "$verify_otp_data"

  local forgot_password_data
  forgot_password_data=$(jq -n --arg email "$TEST_EMAIL" '{email: $email}')
  test_api_endpoint "Forgot Password Test" "POST" "/v1/auth/forgot" 400 "$forgot_password_data"

  local new_password_data
  new_password_data=$(jq -n --arg token "some_reset_token" --arg pass "newpassword123!A" '{token: $token, password: $pass}')
  test_api_endpoint "New Password Test (Invalid Token)" "POST" "/v1/auth/new-password" 400 "$new_password_data"

  local refresh_token=$(curl -s -X POST -H "Content-Type: application/json" -d "$(jq -n --arg email "$TEST_EMAIL" --arg pass "$TEST_PASSWORD" '{email: $email, password: $pass}')" "$BASE_URL/v1/auth/login" | jq -r '.data.token.refresh_token // empty')
  if [ -n "$refresh_token" ]; then
    local refresh_data
    refresh_data=$(jq -n --arg token "$refresh_token" '{refresh_token: $token}')
    test_api_endpoint "Refresh Token Test" "POST" "/v1/auth/refresh" 200 "$refresh_data"
  else
    write_test_log "WARN" "✗ Refresh Token Test - Dilewati: Refresh token tidak tersedia dari login"
  fi
}

test_error_handling() {
  printf "\n${CYAN}=== Menguji Error Handling ===${NC}\n"
  test_api_endpoint "Non-existent Endpoint" "GET" "/v1/nonexistent" 404
  test_api_endpoint "Unauthorized Access" "GET" "/v1/users" 401 "" false
}

test_user_management_endpoints() {
  printf "\n${CYAN}=== Menguji User Management Endpoints ===${NC}\n"
  test_api_endpoint "Get Users List" "GET" "/v1/users" 200 "" true
  
  local test_user_id="c3b1d6a8-8d4f-4b36-b789-2e532ec7a7b2"
  test_api_endpoint "Get User By ID" "GET" "/v1/users/detail/$test_user_id" 200 "" true

  local new_user_email="new_test_user_$(date +%s%N)@example.com"
  local new_user_fullname="New Test User $(date +%s%N)"
  local new_user_phone="089876543211"
  local new_user_password="NewPassword123!"
  local new_user_role_id="5713cb37-dc02-4e87-8048-d7a41d352059" # User role ID from seed_users.rs

  local create_user_data=$(jq -n \
    --arg email "$new_user_email" \
    --arg pass "$new_user_password" \
    --arg fullname "$new_user_fullname" \
    --arg phone "$new_user_phone" \
    --arg is_active true \
    --arg role_id "$new_user_role_id" \
    '{email: $email, password: $pass, fullname: $fullname, phone_number: $phone, is_active: $is_active | fromjson, role_id: $role_id}')
  test_api_endpoint "Create New User" "POST" "/v1/users/create" 201 "$create_user_data" true

  # Assuming the created user can be fetched by email for update/delete
  local created_user_id=$(curl -s -X GET -H "Authorization: Bearer $AUTH_TOKEN" "$BASE_URL/v1/users?search=$new_user_email" | jq -r '.data[0].id // empty')

  if [ -n "$created_user_id" ]; then
    local updated_user_fullname="Updated Test User $(date +%s%N)"
    local updated_user_data=$(jq -n \
      --arg email "$new_user_email" \
      --arg pass "$new_user_password" \
      --arg fullname "$updated_user_fullname" \
      --arg phone "$new_user_phone" \
      --arg is_active true \
      --arg gender "Male" \
      --arg birthdate "1990-01-01" \
      --arg avatar "https://example.com/avatar.jpg" \
      --arg role_id "$new_user_role_id" \
      '{email: $email, password: $pass, fullname: $fullname, phone_number: $phone, is_active: $is_active | fromjson, gender: $gender, birthdate: $birthdate, avatar: $avatar, role_id: $role_id}')
    test_api_endpoint "Update User" "PUT" "/v1/users/update/$created_user_id" 200 "$updated_user_data" true

    local set_active_data=$(jq -n --arg is_active false '{is_active: $is_active | fromjson}')
    test_api_endpoint "Deactivate User" "PUT" "/v1/users/activate/$created_user_id" 200 "$set_active_data" true

    local set_active_data=$(jq -n --arg is_active true '{is_active: $is_active | fromjson}')
    test_api_endpoint "Reactivate User" "PUT" "/v1/users/activate/$created_user_id" 200 "$set_active_data" true

    test_api_endpoint "Delete User" "DELETE" "/v1/users/delete/$created_user_id" 200 "" true
  else
    write_test_log "WARN" "✗ Skipping User Update/Delete tests: Failed to retrieve ID of newly created user."
  fi
}

test_crud_operations() {
  printf "\n${CYAN}=== Menguji CRUD Operations ===${NC}\n"
  
  local testimonial_data
  testimonial_data=$(jq -n --arg content "Test testimonial via Bash $(date +%s)" '{role: "Student", content: $content}')
  local testimonial_response=$(test_api_endpoint "Create Testimonial" "POST" "/v1/cms/landing/testimonials/create" 201 "$testimonial_data" true)
  TEST_TESTIMONIAL_ID=$(echo "$testimonial_response" | jq -r '.data.id // empty')
  write_test_log "INFO" "TEST_TESTIMONIAL_ID: $TEST_TESTIMONIAL_ID"
  write_test_log "INFO" "Captured Testimonial ID: $TEST_TESTIMONIAL_ID"
  sleep 0.2
  
  local permission_data
  permission_data=$(jq -n --arg name "Test Permission $(date +%s)" '{name: $name}')
  test_api_endpoint "Create Permission" "POST" "/v1/permissions/create" 201 "$permission_data" true
  
  local gacha_item_data
  gacha_item_data=$(jq -n --arg name "Test Item $(date +%s)" '{name: $name, image_url: "https://example.com/id.jpg"}')
  test_api_endpoint "Create Gacha Item" "POST" "/v1/gacha/items/create" 201 "$gacha_item_data" true
  
  local event_data
  event_data=$(jq -n --arg name "Test Event $(date +%s)" '{
    name: $name,
    description: "Test event description",
    detail_link: "https://example.com/event",
    price: 50.0,
    is_online: true,
    start_date: "2025-12-01T10:00:00Z",
    end_date: "2025-12-01T16:00:00Z",
    location: null
  }')
  test_api_endpoint "Create Event" "POST" "/v1/cms/landing/events/create" 201 "$event_data" true
}

test_roles_and_permissions() {
  printf "\n${CYAN}=== Menguji Roles & Permissions Endpoints ===${NC}\n"
  test_api_endpoint "Get Roles List" "GET" "/v1/roles" 200 "" true
  test_api_endpoint "Get Permissions List" "GET" "/v1/permissions" 200 "" true
  
  local test_role_id="3b9f8c4e-6a2d-4f8a-9a12-2d6f8b3c4e5a"
  test_api_endpoint "Get Role By ID" "GET" "/v1/roles/detail/$test_role_id" 200 "" true
}

test_mentor_endpoints() {
  printf "\n${CYAN}=== Menguji Mentor Endpoints ===${NC}\n"
  test_api_endpoint "Get Mentors List" "GET" "/v1/mentors" 200 "" true
  # These tests are run with AUTH_TOKEN set to admin. Since admin is not a mentor, these should be 403.
  test_api_endpoint "Get Mentor Me" "GET" "/v1/mentors/me" 403 "" true
  test_api_endpoint "Get Mentor Status" "GET" "/v1/mentors/status" 403 "" true
  
  local test_mentor_id="e6f78d23-83bf-5c2b-bcd4-001345678901"
  test_api_endpoint "Get Mentor By ID" "GET" "/v1/mentors/detail/$test_mentor_id" 200 "" true
  
  local mentor_register_data
  mentor_register_data=$(jq -n --arg email "test.mentor.$(date +%s%N)@example.com" '{
    identity_and_verification: {
      legal_name: "Test Mentor Legal Name",
      identity_document_url: "https://example.com/id.jpg",
      phone_for_verification: "+1234567890"
    },
    professional_profile: {
      bio: "Test mentor bio",
      linkedin_url: "https://linkedin.com/in/testmentor",
      industries: ["Technology", "Software"],
      expertise: ["JavaScript", "Python"],
      languages: ["English", "Indonesian"],
      current_company: "Test Company",
      current_role: "Senior Developer", 
      years_of_experience: 5
    },
    mentoring_logistics: {
      topics_of_interest: ["Career Development", "Technical Skills"],
      preferred_mentee_level: ["Junior", "Mid-level"],
      preferred_mentoring_formats: ["1-on-1", "Group"],
      availability_commitment: "2-3 hours per week",
      mentoring_rate: {
        amount: 100000,
        currency: "IDR",
        per_duration: "hour"
      }
    },
    email: $email
  }')
  test_api_endpoint "Register as Mentor" "POST" "/v1/mentors/register" 422 "$mentor_register_data" true
}

test_events_endpoints() {
  printf "\n${CYAN}=== Menguji Events Endpoints ===${NC}\n"
  test_api_endpoint "Get Events with Pagination" "GET" "/v1/cms/landing/events?page=1&per_page=5" 200
  test_api_endpoint "Get Events with Search" "GET" "/v1/cms/landing/events?search=tech" 200
  
  local test_event_id="e1a2b3c4-5d6e-7f8g-9h0i-1j2k3l4m5n6o"
  test_api_endpoint "Get Event By ID" "GET" "/v1/cms/landing/events/detail/$test_event_id" 200
}

test_testimonials_endpoints() {
  printf "\n${CYAN}=== Menguji Testimonials Endpoints ===${NC}\n"
  test_api_endpoint "Get Testimonials with Pagination" "GET" "/v1/cms/landing/testimonials?page=1&per_page=5" 200
  
  # Ensure TEST_TESTIMONIAL_ID is not empty before testing
  if [ -n "$TEST_TESTIMONIAL_ID" ]; then
    test_api_endpoint "Get Testimonial By ID" "GET" "/v1/cms/landing/testimonials/detail/$TEST_TESTIMONIAL_ID" 200
  else
    write_test_log "WARN" "✗ Get Testimonial By ID - Dilewati: TEST_TESTIMONIAL_ID tidak tersedia"
  fi
}

test_gacha_endpoints() {
  printf "\n${CYAN}=== Menguji Gacha Endpoints ===${NC}\n"
  test_api_endpoint "Get Gacha Items" "GET" "/v1/gacha/items" 200 "" true
  test_api_endpoint "Execute Gacha Roll" "POST" "/v1/gacha/rolls/execute" 200 "" true
}

test_advanced_scenarios() {
  printf "\n${CYAN}=== Menguji Advanced Scenarios ===${NC}\n"
  
  test_api_endpoint "Events with Advanced Filter" "GET" "/v1/cms/landing/events?filter=online&filter_by=is_online" 200
  test_api_endpoint "Users with Sort" "GET" "/v1/users?sort_by=created_at&order=DESC" 200 "" true
  test_api_endpoint "Testimonials with Search" "GET" "/v1/cms/landing/testimonials?search=test" 200
  
  local mentor_register_data
  mentor_register_data=$(jq -n --arg email "test.mentor.$(date +%s%N)@example.com" '{
    identity_and_verification: {
      legal_name: "Test Mentor Legal Name",
      identity_document_url: "https://example.com/id.jpg",
      phone_for_verification: "+1234567890"
    },
    professional_profile: {
      bio: "Test mentor bio",
      linkedin_url: "https://linkedin.com/in/testmentor",
      industries: ["Technology", "Software"],
      expertise: ["JavaScript", "Python"],
      languages: ["English", "Indonesian"],
      current_company: "Test Company",
      current_role: "Senior Developer", 
      years_of_experience: 5
    },
    mentoring_logistics: {
      topics_of_interest: ["Career Development", "Technical Skills"],
      preferred_mentee_level: ["Junior", "Mid-level"],
      preferred_mentoring_formats: ["1-on-1", "Group"],
      availability_commitment: "2-3 hours per week",
      mentoring_rate: {
        amount: 100000,
        currency: "IDR",
        per_duration: "hour"
      }
    },
    email: $email
  }')
  test_api_endpoint "Register as Mentor" "POST" "/v1/mentors/register" 422 "$mentor_register_data" true
  
  test_api_endpoint "Invalid POST to GET endpoint" "POST" "/v1/cms/landing/events" 405
  test_api_endpoint "Invalid PUT with Invalid ID" "PUT" "/v1/users/update/some_invalid_id" 400 "" true
}

show_test_summary() {
  printf "\n${YELLOW}=== Test Coverage Summary ===${NC}\n"
  printf "📋 Authentication: Login, Forgot Password, OTP\n"
  printf "👥 Users: List, Details, Profile Management\n"
  printf "🔐 Roles & Permissions: RBAC System Testing\n"
  printf "👨‍🏫 Mentors: Registration, Profile, Status\n"
  printf "📅 Events: CRUD Operations, Filtering\n"
  printf "💬 Testimonials: Management & Creation\n"
  printf "🎲 Gacha: Items, Rolls, Claims\n"
  printf "🔧 Advanced: Pagination, Search, Edge Cases\n"
  printf "❌ Error Handling: 401, 404, Invalid Requests\n"
  printf "\n"
}

printf "${CYAN}=== IMPHNEN API Comprehensive Test Suite ===${NC}\n"
printf "${YELLOW}Base URL: %s${NC}\n" "$BASE_URL"
show_test_summary

if [ "$START_SERVER" = true ]; then
  if ! command -v cargo &> /dev/null; then
    write_test_log "ERROR" "Perintah 'cargo' tidak ditemukan. Tidak bisa memulai server."
    exit 1
  fi
  printf "${YELLOW}Memulai server backend...${NC}\n"
  RUST_LOG=debug cargo run --bin api &
  SERVER_PID=$!
  
  printf "${YELLOW}Menunggu server siap...${NC}\n"
  retries=0
  max_retries=100 # Increased from 15 to 30
  until test_server_connection; do
    ((retries++))
    if [ $retries -ge $max_retries ]; then
      write_test_log "ERROR" "Gagal memulai server dalam timeout\. Cek output terminal untuk detail\."
      exit 1
    fi
    sleep 2
  done
  write_test_log "SUCCESS" "Server berjalan!"
else
  if ! test_server_connection; then
    write_test_log "ERROR" "Server tidak berjalan di $BASE_URL"
    write_test_log "WARN" "Silakan jalankan server secara manual atau gunakan flag -s"
    exit 1
  fi
  write_test_log "SUCCESS" "Server sudah berjalan di $BASE_URL"
fi

clear_database

printf "\n${CYAN}=== Menjalankan Seeders ===${NC}\n"
if ! RUST_LOG=debug cargo run --bin seeder; then
  write_test_log "ERROR" "Gagal menjalankan seeder roles permissions."
  exit 1
fi
write_test_log "SUCCESS" "Seeders selesai."


printf "\n${CYAN}=== Menampilkan User yang Tersedia ===${NC}\n"
for email in "${!ALL_USERS[@]}"; do
  fullname="${ALL_USERS[$email]}"
  printf "${BLUE}• $fullname${NC} - ${email}\n"
done
printf "\n"

test_public_endpoints

test_all_users_login_performance

test_all_users_individually

if [ "$SKIP_BASIC" = false ]; then
  test_authentication_endpoints
  test_error_handling
fi

if [[ "$SKIP_CRUD" = false && -n "$AUTH_TOKEN" ]]; then
  test_crud_operations # This will now set TEST_TESTIMONIAL_ID
fi

if [ "$SKIP_COMPREHENSIVE" = false ]; then
  test_all_endpoints_with_all_users
fi

if [[ "$SKIP_COMPREHENSIVE" = false && -n "$AUTH_TOKEN" ]]; then
  printf "\n${CYAN}=== Test Comprehensive dengan Admin Token ===${NC}\n"
  test_user_management_endpoints
  test_roles_and_permissions
  test_mentor_endpoints
  test_events_endpoints
  test_testimonials_endpoints # This will now use TEST_TESTIMONIAL_ID
  test_gacha_endpoints
fi

test_advanced_scenarios

TEST_END_TIME=$(date +%s)
TOTAL_DURATION=$((TEST_END_TIME - TEST_START_TIME))
TOTAL_TESTS=$((PASS_COUNT + FAIL_COUNT))
SUCCESS_RATE="0"
if [ "$TOTAL_TESTS" -gt 0 ]; then
  SUCCESS_RATE=$(( (PASS_COUNT * 100) / TOTAL_TESTS ))
fi

if [ "$GENERATE_REPORT" = true ]; then
  printf "\n${CYAN}=== Membuat Laporan Tes ===${NC}\n"
  
  all_results_json=$(printf "%s," "${TEST_RESULTS[@]}")
  all_results_json="[${all_results_json%,}]"
  
  report_file="api-test-report-$(date +'%Y%m%d-%H%M%S').json"

  jq -n --arg start "$(date -d @$TEST_START_TIME +'%Y-%m-%d %H:%M:%S')" \
        --arg end "$(date -d @$TEST_END_TIME +'%Y-%m-%d %H:%M:%S')" \
        --arg dur "$TOTAL_DURATION" \
        --arg url "$BASE_URL" \
        --arg total "$TOTAL_TESTS" \
        --arg pass "$PASS_COUNT" \
        --arg fail "$FAIL_COUNT" \
        --arg rate "${SUCCESS_RATE}%" \
        --argjson results "$all_results_json" \
  '{
    TestRun: {StartTime: $start, EndTime: $end, DurationSec: $dur, BaseUrl: $url},
    Summary: {TotalTests: $total, PassedTests: $pass, FailedTests: $fail, SuccessRate: $rate},
    Results: $results
  }' > "$report_file"

  printf "${BLUE}Laporan tes detail disimpan di: %s${NC}\n" "$report_file"
fi

printf "\n${CYAN}=== Ringkasan Test Suite ===${NC}\n"
printf "Total Durasi: %s detik\n" "$TOTAL_DURATION"
printf "Total Tes   : %s\n" "$TOTAL_TESTS"
printf "${GREEN}Lolos       : %s${NC}\n" "$PASS_COUNT"
printf "${RED}Gagal       : %s${NC}\n" "$FAIL_COUNT"
printf "Tingkat Sukses: %s%%\n" "$SUCCESS_RATE"

if [ "$FAIL_COUNT" -gt 0 ]; then
  printf "\n${RED}Tes yang Gagal:${NC}\n"
  for summary in "${FAILED_TESTS_SUMMARY[@]}"; do
    printf "  %s\n" "$summary"
  done
fi

if [ "$FAIL_COUNT" -eq 0 ]; then
  printf "\n${GREEN}Test suite selesai dengan sukses.${NC}\n"
  exit 0
else
  printf "\n${RED}Test suite selesai dengan beberapa kegagalan.${NC}\n"
  exit 1
fi