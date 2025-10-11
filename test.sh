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
    ["testuser1@example.com"]="Test User 1"
    ["testuser2@example.com"]="Test User 2"
    ["testuser3@example.com"]="Test User 3"
    ["mentor@example.com"]="Mentor User"
)

START_SERVER=false
SKIP_BASIC=false
SKIP_COMPREHENSIVE=false
SKIP_CRUD=false
GENERATE_REPORT=false
VERBOSE=false
SKIP_CLEAR=false
SKIP_SEED=false

while getopts "sbcrgvhkd" opt; do
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
      echo "  -d    Skip database clear"
      echo "  -k    Skip database seeding"
      echo ""
      echo "Examples:"
      echo "  $0                  # Run all tests"
      echo "  $0 -s               # Start server and run all tests"
      echo "  $0 -g               # Run tests and generate report"
      echo "  $0 -sv              # Start server with verbose output"
      echo "  $0 -bc              # Run only public endpoint tests"
      echo "  $0 -d               # Skip database clear"
      echo "  $0 -k               # Skip database seeding"
      echo "  $0 -dk              # Skip database clear and seeding"
      exit 0
      ;;
    d ) SKIP_CLEAR=true ;;
    k ) SKIP_SEED=true ;;
    \? ) echo "Invalid option: -$OPTARG" >&2; echo "Use -h for help" >&2; exit 1 ;;
  esac
done
# Shift past the options
shift "$((OPTIND-1))"

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
FAILED_TESTS_SUMMARY=()
PASS_COUNT=0
FAIL_COUNT=0
TEST_TESTIMONIAL_ID=""
TEST_EVENT_ID=""

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
  
  # Use a more compatible approach for Windows/Git Bash
  local temp_file=$(mktemp)
  local status_file=$(mktemp)
  
  # Make single request and capture both body and status using response headers
  curl -s -X "$method" "${headers[@]}" -d "$body" "$BASE_URL$endpoint" \
    -D "$status_file" -o "$temp_file"
  
  response_body=$(cat "$temp_file")
  
  # Extract HTTP status code from headers file
  http_status=$(head -n 1 "$status_file" | cut -d' ' -f2)
  
  rm -f "$temp_file" "$status_file"

  local end_req_time=$(date +%s%3N)
  local duration=$((end_req_time - start_req_time))

  local status="FAIL"
  local error_msg=""

  # Check if http_status is numeric
  if [[ "$http_status" =~ ^[0-9]+$ ]] && [ "$http_status" -eq "$expected_status" ]; then
    status="PASS"
    ((PASS_COUNT++))
    write_test_log "SUCCESS" "✓ $test_name - Sukses (Status: $http_status, Waktu: ${duration}ms)"
  else
    status="FAIL"
    ((FAIL_COUNT++))
    write_test_log "ERROR" "  Request Body: $body"
    write_test_log "ERROR" "  Response Body: $response_body"
    if [[ ! "$http_status" =~ ^[0-9]+$ ]]; then
      error_msg="Failed to get valid HTTP status code (got: $http_status)"
    else
      error_msg="Status yang diharapkan $expected_status, tetapi mendapat $http_status."
    fi
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
  if [ "$SKIP_CLEAR" = true ]; then
    write_test_log "INFO" "Melewatkan pembersihan database."
    return
  fi
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
  
  # Use a more compatible approach for Windows/Git Bash
  local temp_file=$(mktemp)
  local status_file=$(mktemp)
  
  # Make single request and capture both body and status using response headers
  curl -s -X "POST" "${headers[@]}" -d "$login_data" "$BASE_URL/v1/auth/login" \
    -D "$status_file" -o "$temp_file"
  
  local response_body=$(cat "$temp_file")
  
  # Extract HTTP status code from headers file
  local http_status=$(head -n 1 "$status_file" | cut -d' ' -f2)
  
  rm -f "$temp_file" "$status_file"
  local end_req_time=$(date +%s%3N)
  local duration=$((end_req_time - start_req_time))
  
  if [[ "$http_status" =~ ^[0-9]+$ ]] && [ "$http_status" -eq 200 ]; then
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
  if [[ ! "$http_status" =~ ^[0-9]+$ ]] || [ "$http_status" -ne 200 ] || [[ -z "$AUTH_TOKEN" ]]; then
    status="FAIL"
    if [[ ! "$http_status" =~ ^[0-9]+$ ]]; then
      error_msg="Failed to get valid HTTP status code (got: $http_status)"
    else
      error_msg="Authentication failed"
    fi
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
  
  # Use a more compatible approach for Windows/Git Bash
  local temp_file=$(mktemp)
  local status_file=$(mktemp)
  
  # Make single request and capture both body and status using response headers
  curl -s -X "POST" -H "Content-Type: application/json" -d "$login_data" "$BASE_URL/v1/auth/login" \
    -D "$status_file" -o "$temp_file"
  
  local response_body=$(cat "$temp_file")
  
  # Extract HTTP status code from headers file
  local http_status=$(head -n 1 "$status_file" | cut -d' ' -f2)
  
  rm -f "$temp_file" "$status_file"
  local end_time=$(date +%s%3N)
  local duration=$((end_time - start_time))
  
  total_login_time=$((total_login_time + duration))
  
  if [[ "$http_status" =~ ^[0-9]+$ ]] && [ "$http_status" -eq 200 ]; then
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
  
  # Use a more compatible approach for Windows/Git Bash
  local temp_file=$(mktemp)
  local status_file=$(mktemp)
  
  # Make single request and capture both body and status using response headers
  curl -s -X "POST" -H "Content-Type: application/json" -d "$login_data" "$BASE_URL/v1/auth/login" \
    -D "$status_file" -o "$temp_file"
  
  local response_body=$(cat "$temp_file")
  
  # Extract HTTP status code from headers file
  local http_status=$(head -n 1 "$status_file" | cut -d' ' -f2)
  
  rm -f "$temp_file" "$status_file"
  local end_time=$(date +%s%3N)
  local duration=$((end_time - start_time))
  
  if [[ "$http_status" =~ ^[0-9]+$ ]] && [ "$http_status" -eq 200 ]; then
    if echo "$response_body" | jq -e '.data.token.access_token' > /dev/null 2>&1; then
      local user_auth_token=$(echo "$response_body" | jq -r '.data.token.access_token')
      write_test_log "SUCCESS" "✓ Login $fullname berhasil - ${duration}ms"
      
      local me_response
      # Use a more compatible approach for Windows/Git Bash
      local temp_file=$(mktemp)
      local status_file=$(mktemp)
      
      # Make single request and capture both body and status using response headers
      curl -s -X "GET" -H "Content-Type: application/json" -H "Authorization: Bearer $user_auth_token" "$BASE_URL/v1/users/me" \
        -D "$status_file" -o "$temp_file"
      
      local me_body=$(cat "$temp_file")
      
      # Extract HTTP status code from headers file
      local me_status=$(head -n 1 "$status_file" | cut -d' ' -f2)
      
      rm -f "$temp_file" "$status_file"
      
      if [[ "$me_status" =~ ^[0-9]+$ ]] && [ "$me_status" -eq 200 ]; then
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
  
  # Use a more compatible approach for Windows/Git Bash
  local temp_file=$(mktemp)
  local status_file=$(mktemp)
  
  # Make single request and capture both body and status using response headers
  curl -s -X "POST" -H "Content-Type: application/json" -d "$login_data" "$BASE_URL/v1/auth/login" \
    -D "$status_file" -o "$temp_file"
  
  local response_body=$(cat "$temp_file")
  
  # Extract HTTP status code from headers file
  local http_status=$(head -n 1 "$status_file" | cut -d' ' -f2)
  
  rm -f "$temp_file" "$status_file"
  local end_time=$(date +%s%3N)
  local duration=$((end_time - start_time))
  
  if [[ "$http_status" =~ ^[0-9]+$ ]] && [ "$http_status" -eq 200 ]; then
    if echo "$response_body" | jq -e '.data.token.access_token' > /dev/null 2>&1; then
      user_token=$(echo "$response_body" | jq -r '.data.token.access_token')
      write_test_log "SUCCESS" "✓ Login $fullname berhasil - ${duration}ms"
      
      local original_auth_token="$AUTH_TOKEN"
      
      AUTH_TOKEN="$user_token"
      # Get user ID and add credits for gacha testing
      local temp_file=$(mktemp)
      local status_file=$(mktemp)
      
      # Get user profile to extract user_id
      curl -s -X "GET" -H "Content-Type: application/json" -H "Authorization: Bearer $user_token" "$BASE_URL/v1/users/me" \
        -D "$status_file" -o "$temp_file"
      
      local user_profile_body=$(cat "$temp_file")
      local user_profile_status=$(head -n 1 "$status_file" | cut -d' ' -f2)
      
      rm -f "$temp_file" "$status_file"
      
      if [[ "$user_profile_status" =~ ^[0-9]+$ ]] && [ "$user_profile_status" -eq 200 ]; then
        local user_id=$(echo "$user_profile_body" | jq -r '.data.id // empty')
        if [ -n "$user_id" ]; then
          # Add credits for gacha testing
          local add_credits_data=$(jq -n --arg user_id "$user_id" '{user_id: $user_id, amount: 10}')
          local temp_file=$(mktemp)
          local status_file=$(mktemp)
          
          curl -s -X "POST" -H "Content-Type: application/json" -H "Authorization: Bearer $user_token" -d "$add_credits_data" "$BASE_URL/v1/gacha/credits/add" \
            -D "$status_file" -o "$temp_file"
          
          local add_credits_body=$(cat "$temp_file")
          local add_credits_status=$(head -n 1 "$status_file" | cut -d' ' -f2)
          
          rm -f "$temp_file" "$status_file"
          
          if [[ "$add_credits_status" =~ ^[0-9]+$ ]] && [ "$add_credits_status" -eq 200 ]; then
            write_test_log "SUCCESS" "✓ Credits added for $fullname (user_id: $user_id)"

            # Verify credits were added correctly
            local get_credits_response=$(curl -s -w "\nHTTP_STATUS:%{http_code}" -X GET "$BASE_URL/v1/gacha/credits" \
              -H "Authorization: Bearer $user_token")
            local get_credits_body=$(echo "$get_credits_response" | head -n -1)
            local get_credits_status=$(echo "$get_credits_response" | tail -n 1 | sed 's/HTTP_STATUS://')

            if [[ "$get_credits_status" =~ ^[0-9]+$ ]] && [ "$get_credits_status" -eq 200 ]; then
              local available_rolls=$(echo "$get_credits_body" | jq -r '.data.available_rolls // 0')
              if [ "$available_rolls" -ge 10 ]; then
                write_test_log "SUCCESS" "✓ Credits verified for $fullname: $available_rolls rolls available"
              else
                write_test_log "ERROR" "✗ Credits not added correctly for $fullname: expected >=10, got $available_rolls"
                return 1
              fi
            else
              write_test_log "ERROR" "✗ Failed to get credits for $fullname - HTTP $get_credits_status: $get_credits_body"
              return 1
            fi
          else
            write_test_log "ERROR" "✗ Failed to add credits for $fullname - HTTP $add_credits_status: $add_credits_body"
            return 1
          fi
        else
          write_test_log "ERROR" "✗ Failed to get user_id for $fullname from profile response"
        fi
      else
        write_test_log "ERROR" "✗ Failed to get user profile for $fullname - HTTP $user_profile_status"
      fi
      
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

          # Team endpoints for admin
          # Admin teams endpoint should only be accessible to admins
          if [ "$email" = "admin@example.com" ]; then
            test_api_endpoint "Get Admin Teams List - $fullname" "GET" "/v1/teams/admin" 200 "" true
          else
            test_api_endpoint "Get Admin Teams List - $fullname" "GET" "/v1/teams/admin" 403 "" true
          fi
          test_api_endpoint "Get Public Teams List - $fullname" "GET" "/v1/teams" 200 "" true
          test_api_endpoint "Search Teams - $fullname" "GET" "/v1/teams/search?query=Development" 200 "" true
          
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

          # Team endpoints for admin
          # Admin teams endpoint should only be accessible to admins
          if [ "$email" = "admin@example.com" ]; then
            test_api_endpoint "Get Admin Teams List - $fullname" "GET" "/v1/teams/admin" 200 "" true
          else
            test_api_endpoint "Get Admin Teams List - $fullname" "GET" "/v1/teams/admin" 403 "" true
          fi
          test_api_endpoint "Get Public Teams List - $fullname" "GET" "/v1/teams" 200 "" true
          test_api_endpoint "Search Teams - $fullname" "GET" "/v1/teams/search?query=Development" 200 "" true
          
          local testimonial_data
          testimonial_data=$(jq -n --arg content "Test testimonial by $fullname $(date +%s)" '{role: "Student", content: $content}')
          test_api_endpoint "Create Testimonial - $fullname" "POST" "/v1/cms/landing/testimonials/create" 201 "$testimonial_data" true
          ;;
          
        "mentor@example.com")
          test_api_endpoint "Get Users List - $fullname" "GET" "/v1/users" 200 "" true
          test_api_endpoint "Get Roles List - $fullname" "GET" "/v1/roles" 403 "" true
          test_api_endpoint "Get Permissions List - $fullname" "GET" "/v1/permissions" 403 "" true
          test_api_endpoint "Get Mentors List - $fullname" "GET" "/v1/mentors" 200 "" true
          test_api_endpoint "Get Mentor Me - $fullname" "GET" "/v1/mentors/me" 200 "" true
          test_api_endpoint "Get Mentor Status - $fullname" "GET" "/v1/mentors/status" 200 "" true
          test_api_endpoint "Get Gacha Items - $fullname" "GET" "/v1/gacha/items" 200 "" true
          test_api_endpoint "Execute Gacha Roll - $fullname" "POST" "/v1/gacha/rolls/execute" 200 "" true

          # Team endpoints for admin
          # Admin teams endpoint should only be accessible to admins
          if [ "$email" = "admin@example.com" ]; then
            # Admin teams endpoint should only be accessible to admins
            if [ "$email" = "admin@example.com" ]; then
              test_api_endpoint "Get Admin Teams List - $fullname" "GET" "/v1/teams/admin" 200 "" true
            else
              test_api_endpoint "Get Admin Teams List - $fullname" "GET" "/v1/teams/admin" 403 "" true
            fi
          else
            test_api_endpoint "Get Admin Teams List - $fullname" "GET" "/v1/teams/admin" 403 "" true
          fi
          test_api_endpoint "Get Public Teams List - $fullname" "GET" "/v1/teams" 200 "" true
          test_api_endpoint "Search Teams - $fullname" "GET" "/v1/teams/search?query=Development" 200 "" true
          
          local testimonial_data
          testimonial_data=$(jq -n --arg content "Test testimonial by $fullname $(date +%s)" '{role: "Student", content: $content}')
          test_api_endpoint "Create Testimonial - $fullname" "POST" "/v1/cms/landing/testimonials/create" 201 "$testimonial_data" true
          ;;
          
        "user@example.com")
          test_api_endpoint "Get Users List - $fullname" "GET" "/v1/users" 200 "" true
          test_api_endpoint "Get Roles List - $fullname" "GET" "/v1/roles" 403 "" true
          test_api_endpoint "Get Permissions List - $fullname" "GET" "/v1/permissions" 403 "" true
          test_api_endpoint "Get Mentors List - $fullname" "GET" "/v1/mentors" 200 "" true
          test_api_endpoint "Get Mentor Me - $fullname" "GET" "/v1/mentors/me" 403 "" true # User is not a mentor
          test_api_endpoint "Get Mentor Status - $fullname" "GET" "/v1/mentors/status" 403 "" true # User is not a mentor
          test_api_endpoint "Get Gacha Items - $fullname" "GET" "/v1/gacha/items" 200 "" true
          test_api_endpoint "Execute Gacha Roll - $fullname" "POST" "/v1/gacha/rolls/execute" 200 "" true

          # Team endpoints for admin
          test_api_endpoint "Get Admin Teams List - $fullname" "GET" "/v1/teams/admin" 403 "" true
          test_api_endpoint "Get Public Teams List - $fullname" "GET" "/v1/teams" 200 "" true
          test_api_endpoint "Search Teams - $fullname" "GET" "/v1/teams/search?query=Development" 200 "" true
          
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
          test_api_endpoint "Users with Sort - $fullname" "GET" "/v1/users?sort_by=created_at&order=DESC" 200 "" true
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
  test_api_endpoint "Forgot Password Test" "POST" "/v1/auth/forgot" 200 "$forgot_password_data"

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

  # Create user and capture response directly
  local temp_file=$(mktemp)
  local status_file=$(mktemp)

  curl -s -X "POST" -H "Content-Type: application/json" -H "Authorization: Bearer $AUTH_TOKEN" -d "$create_user_data" "$BASE_URL/v1/users/create" \
    -D "$status_file" -o "$temp_file"

  local create_response_body=$(cat "$temp_file")
  local create_status=$(head -n 1 "$status_file" | cut -d' ' -f2)

  rm -f "$temp_file" "$status_file"

  if [[ "$create_status" =~ ^[0-9]+$ ]] && [ "$create_status" -eq 201 ]; then
    ((PASS_COUNT++))
    write_test_log "SUCCESS" "✓ Create New User - Sukses (Status: $create_status, Waktu: ${duration}ms)"

    # Extract user ID directly from create response
    local created_user_id=$(echo "$create_response_body" | jq -r '.data.id // empty')
  else
    ((FAIL_COUNT++))
    write_test_log "ERROR" "✗ Create New User - Gagal (Status: $create_status)"
    write_test_log "ERROR" "  Response Body: $create_response_body"
    FAILED_TESTS_SUMMARY+=("✗ Create New User - HTTP $create_status")
    return
  fi

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
  local testimonial_response
  testimonial_response=$(test_api_endpoint "Create Testimonial" "POST" "/v1/cms/landing/testimonials/create" 201 "$testimonial_data" true)
  TEST_TESTIMONIAL_ID=$(echo "$testimonial_response" | jq -r '.data.id // empty')
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
  local event_response
  event_response=$(test_api_endpoint "Create Event" "POST" "/v1/cms/landing/events/create" 201 "$event_data" true)
  TEST_EVENT_ID=$(echo "$event_response" | jq -r '.data.id // empty')
  write_test_log "INFO" "Captured Event ID: $TEST_EVENT_ID"
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
  
  # Ensure TEST_EVENT_ID is not empty before testing
  if [ -n "$TEST_EVENT_ID" ]; then
    test_api_endpoint "Get Event By ID" "GET" "/v1/cms/landing/events/detail/$TEST_EVENT_ID" 200
  else
    write_test_log "WARN" "✗ Get Event By ID - Dilewati: TEST_EVENT_ID tidak tersedia"
  fi
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

test_team_endpoints() {
  printf "\n${CYAN}=== Menguji Team Endpoints ===${NC}\n"
  test_api_endpoint "Get Public Teams List" "GET" "/v1/teams" 200 "" true
  test_api_endpoint "Search Teams" "GET" "/v1/teams/search?query=Development" 200 "" true
  
  # Test team creation (admin only)
  if [ "$email" = "admin@example.com" ]; then
    local team_data
    team_data=$(jq -n --arg name "Test Team $(date +%s)" '{
      name: $name,
      description: "Test team description",
      is_open: true,
      max_members: 10,
      skills_required: ["Rust", "Backend Development"],
      location: "Remote",
      website_url: "https://example.com/team",
      github_url: "https://github.com/example/team"
    }')
    test_api_endpoint "Create Team (Admin)" "POST" "/v1/teams/create" 201 "$team_data" true
    
    # Test team update (admin only)
    local test_team_id="test-team-001"
    local update_team_data
    update_team_data=$(jq -n --arg name "Updated Test Team" '{
      name: $name,
      description: "Updated test team description",
      is_open: false,
      max_members: 15,
      skills_required: ["Rust", "Backend Development", "DevOps"],
      location: "Hybrid"
    }')
    test_api_endpoint "Update Team (Admin)" "PUT" "/v1/teams/update/$test_team_id" 200 "$update_team_data" true
    
    # Test team member management
    local member_data
    member_data=$(jq -n --arg user_id "user-123" '{user_id: $user_id, role: "MEMBER"}')
    test_api_endpoint "Add Team Member" "POST" "/v1/teams/$test_team_id/members" 200 "$member_data" true
    
    test_api_endpoint "Get Team Members" "GET" "/v1/teams/$test_team_id/members" 200 "" true
    test_api_endpoint "Remove Team Member" "DELETE" "/v1/teams/$test_team_id/members/user-123" 200 "" true
  else
    # Regular users should get 403 for admin endpoints
    test_api_endpoint "Get Admin Teams List" "GET" "/v1/teams/admin" 403 "" true
    
    # Regular users can still access public team operations
    local team_data
    team_data=$(jq -n --arg name "Public Test Team" '{
      name: $name,
      description: "Public test team description"
    }')
    test_api_endpoint "Get Team Details" "GET" "/v1/teams/detail/test-team-001" 200 "" true
  fi
}

test_hackathon_endpoints() {
  printf "\n${CYAN}=== Menguji Hackathon Endpoints ===${NC}\n"
  test_api_endpoint "Get Hackathons List" "GET" "/v1/hackathons" 200 "" true
  test_api_endpoint "Get Hackathons with Pagination" "GET" "/v1/hackathons?page=1&per_page=5" 200 "" true
  test_api_endpoint "Search Hackathons" "GET" "/v1/hackathons?search=test" 200 "" true

  # Test specific hackathon by ID (assuming test hackathon exists from seeder)
  local test_hackathon_id="test-hackathon-001"
  test_api_endpoint "Get Hackathon By ID" "GET" "/v1/hackathons/$test_hackathon_id" 200 "" true

  # Test participant management
  local participant_data=$(jq -n --arg user_id "test-participant@example.com" '{user_id: $user_id}')
  test_api_endpoint "Register Participant" "POST" "/v1/hackathons/$test_hackathon_id/participants" 200 "$participant_data" true
  test_api_endpoint "Get Participants List" "GET" "/v1/hackathons/$test_hackathon_id/participants" 200 "" true

  # Test submission endpoints
  test_api_endpoint "Get Submissions List" "GET" "/v1/hackathons/$test_hackathon_id/submissions" 200 "" true
  test_api_endpoint "Get Submissions with Pagination" "GET" "/v1/hackathons/$test_hackathon_id/submissions?page=1&per_page=5" 200 "" true

  # Test creating a submission (assuming test user is logged in)
  local submission_data
  submission_data=$(jq -n --arg hackathon_id "$test_hackathon_id" --arg team_id "test-team-001" '{
    hackathon_id: $hackathon_id,
    team_id: $team_id,
    project_name: "Test Project Submission",
    description: "This is a test project submission for hackathon testing",
    technologies: ["Rust", "React", "PostgreSQL"],
    repository_url: "https://github.com/test/test-project",
    demo_url: "https://demo.test.com",
    presentation_url: "https://slides.test.com"
  }')
  local create_response
  create_response=$(test_api_endpoint "Create Hackathon Submission" "POST" "/v1/hackathons/$test_hackathon_id/teams/test-team-001/submissions" 201 "$submission_data" true)
  local test_submission_id=$(echo "$create_response" | jq -r '.data.id // empty')

  # Test getting submission by ID
  if [ -n "$test_submission_id" ]; then
    test_api_endpoint "Get Submission By ID" "GET" "/v1/hackathons/submissions/$test_submission_id" 200 "" true
    
    # Test submitting project (finalization)
    test_api_endpoint "Submit Project" "POST" "/v1/hackathons/submissions/$test_submission_id/submit" 200 "" true
    
    # Test updating submission
    local update_submission_data
    update_submission_data=$(jq -n --arg hackathon_id "$test_hackathon_id" --arg team_id "test-team-001" '{
      hackathon_id: $hackathon_id,
      team_id: $team_id,
      project_name: "Updated Test Project Submission",
      description: "This is an updated test project submission for hackathon testing",
      technologies: ["Rust", "React", "PostgreSQL", "Docker"],
      repository_url: "https://github.com/test/updated-test-project",
      demo_url: "https://updated-demo.test.com",
      presentation_url: "https://updated-slides.test.com"
    }')
    test_api_endpoint "Update Hackathon Submission" "PUT" "/v1/hackathons/submissions/$test_submission_id" 200 "$update_submission_data" true

    # Test deleting submission
    test_api_endpoint "Delete Hackathon Submission" "DELETE" "/v1/hackathons/submissions/$test_submission_id" 200 "" true
  else
    write_test_log "WARN" "✗ Submission tests - Dilewati: Failed to capture submission ID from creation response"
  fi
}

test_end_to_end_team_workflow() {
  printf "\n${CYAN}=== End-to-End Team Management Workflow ===${NC}\n"
  
  if [ "$email" != "admin@example.com" ]; then
    write_test_log "WARN" "✗ Workflow hanya dijalankan untuk admin"
    return
  fi
  
  local workflow_start=$(date +%s)
  local team_id=""
  local member_ids=()
  
  printf "\n${BLUE}1. Membuat Tim Baru${NC}\n"
  local team_name="Test Team Workflow $(date +%s)"
  local create_team_data=$(jq -n --arg name "$team_name" '{
    name: $name,
    description: "Team untuk testing end-to-end workflow",
    is_open: true,
    max_members: 5,
    skills_required: ["Rust", "Backend", "Testing"],
    location: "Remote"
  }')
  
  local create_response=$(test_api_endpoint "Create Team" "POST" "/v1/teams/create" 201 "$create_team_data" true)
  team_id=$(echo "$create_response" | jq -r '.data.id // empty')
  
  if [ -z "$team_id" ]; then
    write_test_log "ERROR" "✗ Gagal mendapatkan ID tim dari respons"
    return
  fi
  
  write_test_log "SUCCESS" "Tim berhasil dibuat dengan ID: $team_id"
  
  printf "\n${BLUE}2. Menambahkan Anggota Tim${NC}\n"
  local member_count=3
  for i in $(seq 1 $member_count); do
    local member_email="team_member_$i@example.com"
    local member_data=$(jq -n --arg user_id "$member_email" '{user_id: $user_id, role: "MEMBER"}')
    
    test_api_endpoint "Add Member $i" "POST" "/v1/teams/$team_id/members" 200 "$member_data" true
    member_ids+=("$member_email")
  done
  
  printf "\n${BLUE}3. Memverifikasi Anggota Tim${NC}\n"
  local members_response=$(test_api_endpoint "Get Team Members" "GET" "/v1/teams/$team_id/members" 200 "" true)
  local actual_member_count=$(echo "$members_response" | jq '.data | length')
  
  if [ "$actual_member_count" -eq "$member_count" ]; then
    write_test_log "SUCCESS" "✓ Jumlah anggota sesuai: $actual_member_count/$member_count"
  else
    write_test_log "ERROR" "✗ Jumlah anggota tidak sesuai: $actual_member_count/$member_count"
  fi
  
  printf "\n${BLUE}4. Memperbarui Tim${NC}\n"
  local update_team_data=$(jq -n --arg name "Updated: $team_name" '{
    name: $name,
    description: "Deskripsi tim yang telah diperbarui",
    is_open: false,
    max_members: 10,
    skills_required: ["Rust", "Backend", "Testing", "DevOps"]
  }')
  
  test_api_endpoint "Update Team" "PUT" "/v1/teams/update/$team_id" 200 "$update_team_data" true
  
  printf "\n${BLUE}5. Menghapus Anggota Tim${NC}\n"
  local member_to_remove="${member_ids[0]}"
  test_api_endpoint "Remove Member" "DELETE" "/v1/teams/$team_id/members/$member_to_remove" 200 "" true
  
  printf "\n${BLUE}6. Menghapus Tim${NC}\n"
  test_api_endpoint "Delete Team" "DELETE" "/v1/teams/delete/$team_id" 200 "" true
  
  local workflow_end=$(date +%s)
  local workflow_duration=$((workflow_end - workflow_start))
  
  printf "\n${GREEN}=== Workflow Selesai ===${NC}\n"
  printf "Durasi: %d detik\n" "$workflow_duration"
  printf "Tim: %s\n" "$team_name"
  printf "Anggota awal: %d\n" "$member_count"
  printf "Status: ✅ Selesai\n"
}

test_end_to_end_hackathon_workflow() {
  printf "\n${CYAN}=== End-to-End Hackathon Management Workflow ===${NC}\n"
  
  local workflow_start=$(date +%s)
  local hackathon_id=""
  local submission_id=""
  
  printf "\n${BLUE}1. Membuat Hackathon Baru${NC}\n"
  local hackathon_name="Hackathon Test $(date +%s)"
  local create_hackathon_data=$(jq -n --arg name "$hackathon_name" '{
    name: $name,
    description: "Hackathon untuk testing end-to-end workflow",
    start_date: "'$(date -d "+2 days" +%Y-%m-%dT%H:%M:%SZ)'",
    end_date: "'$(date -d "+3 days" +%Y-%m-%dT%H:%M:%SZ)'",
    registration_deadline: "'$(date -d "+1 day" +%Y-%m-%dT%H:%M:%SZ)'",
    max_participants: 20,
    theme: "Backend Development",
    rules: "Buat sesuatu yang berfaedah!",
    prizes: [{"name": "Juara 1", "description": "Hadiah utama"}],
    organizers: ["admin@example.com"]
  }')
  
  local create_response=$(test_api_endpoint "Create Hackathon" "POST" "/v1/hackathons" 201 "$create_hackathon_data" true)
  hackathon_id=$(echo "$create_response" | jq -r '.data.id // empty')
  
  if [ -z "$hackathon_id" ]; then
    write_test_log "ERROR" "✗ Gagal mendapatkan ID hackathon dari respons"
    return
  fi
  
  write_test_log "SUCCESS" "Hackathon berhasil dibuat dengan ID: $hackathon_id"
  
  printf "\n${BLUE}2. Mendaftarkan Peserta${NC}\n"
  local participant_data=$(jq -n --arg user_id "participant_1@example.com" '{user_id: $user_id}')
  test_api_endpoint "Register Participant" "POST" "/v1/hackathons/$hackathon_id/participants" 200 "$participant_data" true
  
  printf "\n${BLUE}3. Membuat Submission Proyek${NC}\n"
  local submission_data=$(jq -n --arg hackathon_id "$hackathon_id" --arg team_id "test-team-001" '{
    hackathon_id: $hackathon_id,
    team_id: $team_id,
    project_name: "Proyek Test Workflow",
    description: "Proyek contoh untuk testing submission",
    technologies: ["Rust", "PostgreSQL", "Docker"],
    repository_url: "https://github.com/test/proyek-workflow",
    demo_url: "https://demo.test.com",
    presentation_url: "https://slides.test.com"
  }')
  
  local submission_response=$(test_api_endpoint "Create Submission" "POST" "/v1/hackathons/$hackathon_id/teams/test-team-001/submissions" 201 "$submission_data" true)
  submission_id=$(echo "$submission_response" | jq -r '.data.id // empty')
  
  if [ -n "$submission_id" ]; then
    write_test_log "SUCCESS" "Submission berhasil dibuat dengan ID: $submission_id"
    
    printf "\n${BLUE}4. Memperbarui Submission${NC}\n"
    local update_submission_data=$(jq -n --arg hackathon_id "$hackathon_id" --arg team_id "test-team-001" '{
      hackathon_id: $hackathon_id,
      team_id: $team_id,
      project_name: "Proyek Test Workflow (Diperbarui)",
      description: "Proyek contoh untuk testing submission yang telah diperbarui",
      technologies: ["Rust", "PostgreSQL", "Docker", "Kubernetes"]
    }')
    
    test_api_endpoint "Update Submission" "PUT" "/v1/hackathons/submissions/$submission_id" 200 "$update_submission_data" true
    
    printf "\n${BLUE}5. Mengirim Submission (Finalisasi)${NC}\n"
    test_api_endpoint "Submit Project" "POST" "/v1/hackathons/submissions/$submission_id/submit" 200 "" true
  fi
  
  printf "\n${BLUE}6. Memverifikasi Semua Data${NC}\n"
  test_api_endpoint "Get Hackathon Details" "GET" "/v1/hackathons/$hackathon_id" 200 "" true
  test_api_endpoint "Get Participants List" "GET" "/v1/hackathons/$hackathon_id/participants" 200 "" true
  
  if [ -n "$submission_id" ]; then
    test_api_endpoint "Get Submission Details" "GET" "/v1/hackathons/submissions/$submission_id" 200 "" true
  fi
  
  local workflow_end=$(date +%s)
  local workflow_duration=$((workflow_end - workflow_start))
  
  printf "\n${GREEN}=== Workflow Selesai ===${NC}\n"
  printf "Durasi: %d detik\n" "$workflow_duration"
  printf "Hackathon: %s\n" "$hackathon_name"
  printf "Status: ✅ Selesai\n"
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
if [ "$SKIP_SEED" = true ]; then
  write_test_log "INFO" "Melewatkan seeding database."
  # Still seed permissions, teams, and gacha rolls even if skipping other seeds
  if ! RUST_LOG=debug cargo run --bin seed_permissions; then
    write_test_log "ERROR" "Gagal menjalankan seed permissions."
    exit 1
  fi
  write_test_log "SUCCESS" "Permissions seeded."
  if ! RUST_LOG=debug cargo run --bin seed_teams; then
    write_test_log "ERROR" "Gagal menjalankan seed teams."
    exit 1
  fi
  write_test_log "SUCCESS" "Teams seeded."
  if ! RUST_LOG=debug cargo run --bin seed_gacha_rolls; then
    write_test_log "ERROR" "Gagal menjalankan seed gacha rolls."
    exit 1
  fi
  write_test_log "SUCCESS" "Gacha rolls seeded."
  if ! RUST_LOG=debug cargo run --bin seed_test_submission; then
    write_test_log "ERROR" "Gagal menjalankan seed test submission."
    exit 1
  fi
  write_test_log "SUCCESS" "Test submission seeded."
else
  if ! RUST_LOG=debug cargo run --bin seeder; then
    write_test_log "ERROR" "Gagal menjalankan seeder roles permissions."
    exit 1
  fi
  write_test_log "SUCCESS" "Seeders selesai."
  if ! RUST_LOG=debug cargo run --bin seed_test_submission; then
    write_test_log "ERROR" "Gagal menjalankan seed test submission."
    exit 1
  fi
  write_test_log "SUCCESS" "Test submission seeded."
fi


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
  test_team_endpoints
  test_hackathon_endpoints
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