#!/bin/bash

# ==============================================================================
# Common Functions and Variables for IMPHNEN API Tests
# ==============================================================================

#!/bin/bash

# Common configuration and functions for API testing

# Colors for output
export RED='\033[0;31m'
export GREEN='\033[0;32m'
export YELLOW='\033[1;33m'
export NC='\033[0m' # No Color

# Base configuration
export BASE_URL="${BASE_URL:-http://127.0.0.1:4099}"
export TEST_USER_EMAIL="${TEST_USER_EMAIL:-admin@example.com}"
export TEST_USER_PASSWORD="${TEST_USER_PASSWORD:-Admin@123}"

# Global variables for auth
export AUTH_TOKEN=""
export AUTH_USER_ID=""
TEST_RESULTS=()
FAILED_TESTS_SUMMARY=()
PASS_COUNT=0
FAIL_COUNT=0

# Colors
CYAN='\033[0;36m'
YELLOW='\033[0;33m'
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m'

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
  
  local temp_file=$(mktemp)
  local status_file=$(mktemp)
  
  curl -s -X "$method" "${headers[@]}" -d "$body" "$BASE_URL$endpoint" \
    -D "$status_file" -o "$temp_file"
  
  response_body=$(cat "$temp_file")
  http_status=$(head -n 1 "$status_file" | cut -d' ' -f2)
  
  rm -f "$temp_file" "$status_file"

  local end_req_time=$(date +%s%3N)
  local duration=$((end_req_time - start_req_time))

  local status="FAIL"
  local error_msg=""

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
  printf "%s" "$response_body"
}

get_auth_token() {
  write_test_log "INFO" "Mengautentikasi test user..."
  local login_data
  login_data=$(jq -n --arg email "${TEST_EMAIL:-admin@example.com}" --arg pass "${TEST_PASSWORD:-password}" '{email: $email, password: $pass}')
  
  local temp_file=$(mktemp)
  local status_file=$(mktemp)
  
  curl -s -X "POST" -H "Content-Type: application/json" -d "$login_data" "$BASE_URL/v1/auth/login" \
    -D "$status_file" -o "$temp_file"
  
  local response_body=$(cat "$temp_file")
  local http_status=$(head -n 1 "$status_file" | cut -d' ' -f2)
  
  rm -f "$temp_file" "$status_file"
  
  if [[ "$http_status" =~ ^[0-9]+$ ]] && [ "$http_status" -eq 200 ]; then
    if echo "$response_body" | jq . > /dev/null 2>&1; then
      AUTH_TOKEN=$(echo "$response_body" | jq -r '.data.token.access_token // empty')
      AUTH_USER_ID=$(echo "$response_body" | jq -r '.data.user.id // empty')
      if [[ -n "$AUTH_TOKEN" && "$AUTH_TOKEN" != "null" ]]; then
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
    write_test_log "ERROR" "Login gagal dengan status: $http_status"
    AUTH_TOKEN=""
    ((FAIL_COUNT++))
  fi
}

print_test_summary() {
  local total_tests=$((PASS_COUNT + FAIL_COUNT))
  local success_rate=0
  if [ "$total_tests" -gt 0 ]; then
    success_rate=$(( (PASS_COUNT * 100) / total_tests ))
  fi
  
  printf "\n${CYAN}=== Test Summary ===${NC}\n"
  printf "Total Tests: %d\n" "$total_tests"
  printf "${GREEN}Passed: %d${NC}\n" "$PASS_COUNT"
  printf "${RED}Failed: %d${NC}\n" "$FAIL_COUNT"
  printf "Success Rate: %d%%\n\n" "$success_rate"
  
  if [ "$FAIL_COUNT" -gt 0 ]; then
    printf "${RED}Failed Tests:${NC}\n"
    for summary in "${FAILED_TESTS_SUMMARY[@]}"; do
      printf "  %s\n" "$summary"
    done
    printf "\n"
  fi
}
