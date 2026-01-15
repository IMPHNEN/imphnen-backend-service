#!/bin/bash

# ==============================================================================
# Common Functions and Variables for IMPHNEN API Tests
# ==============================================================================

# Disable MSYS path conversion for Windows compatibility
export MSYS_NO_PATHCONV=1

# Common configuration and functions for API testing

# Colors for output
export RED='\033[0;31m'
export GREEN='\033[0;32m'
export YELLOW='\033[1;33m'
export NC='\033[0m' # No Color

# Base configuration
export BASE_URL="${BASE_URL:-http://127.0.0.1:4099}"
export TEST_USER_EMAIL="${TEST_USER_EMAIL:-admin@example.com}"
export TEST_USER_PASSWORD="${TEST_USER_PASSWORD:-password}"

# Global variables for auth
export AUTH_TOKEN=""
export AUTH_USER_ID=""
TEST_RESULTS=()
FAILED_TESTS_SUMMARY=()
PASS_COUNT=0
FAIL_COUNT=0
# A cross-subshell results accumulator so command substitutions $(...) still record results
# Each line is a compact JSON object describing one API call result
RESULTS_FILE=${RESULTS_FILE:-"$(mktemp)"}
export RESULTS_FILE

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

# ==============================================================================
# Response Validation Functions
# ==============================================================================

# Validate that a JSON field exists and optionally matches a value
# Usage: assert_json_field "response_json" "field_path" "expected_value" (optional)
# Example: assert_json_field "$response" ".data.id" 
# Example: assert_json_field "$response" ".data.name" "Test User"
assert_json_field() {
  local json_response=$1
  local field_path=$2
  local expected_value=$3
  
  if ! echo "$json_response" | jq -e . >/dev/null 2>&1; then
    echo "ERROR: Response is not valid JSON"
    return 1
  fi
  
  local actual_value
  actual_value=$(echo "$json_response" | jq -r "$field_path // \"__FIELD_NOT_FOUND__\"")
  
  if [[ "$actual_value" == "__FIELD_NOT_FOUND__" || "$actual_value" == "null" ]]; then
    echo "ERROR: Field '$field_path' not found in response"
    return 1
  fi
  
  if [[ -n "$expected_value" ]]; then
    if [[ "$actual_value" != "$expected_value" ]]; then
      echo "ERROR: Field '$field_path' expected '$expected_value' but got '$actual_value'"
      return 1
    fi
  fi
  
  return 0
}

# Validate that a JSON response contains specific key-value pairs
# Usage: assert_json_contains "response_json" "jq_filter" "description"
# Example: assert_json_contains "$response" '.data | length > 0' "data array is not empty"
assert_json_contains() {
  local json_response=$1
  local jq_filter=$2
  local description=$3
  
  if ! echo "$json_response" | jq -e . >/dev/null 2>&1; then
    echo "ERROR: Response is not valid JSON"
    return 1
  fi
  
  if ! echo "$json_response" | jq -e "$jq_filter" >/dev/null 2>&1; then
    echo "ERROR: Validation failed - $description (filter: $jq_filter)"
    return 1
  fi
  
  return 0
}

# Validate that response has expected structure
# Usage: assert_response_structure "response_json" "required_fields..."
# Example: assert_response_structure "$response" "data" "version"
assert_response_structure() {
  local json_response=$1
  shift
  local required_fields=("$@")
  
  if ! echo "$json_response" | jq -e . >/dev/null 2>&1; then
    echo "ERROR: Response is not valid JSON"
    return 1
  fi
  
  for field in "${required_fields[@]}"; do
    if ! echo "$json_response" | jq -e "has(\"$field\")" >/dev/null 2>&1; then
      echo "ERROR: Required field '$field' not found in response"
      return 1
    fi
  done
  
  return 0
}

test_api_endpoint() {
  local test_name=$1
  local method=$2
  local endpoint=$3
  local expected_status=$4
  local body=$5
  local require_auth=$6
  local validation_func=$7  # Optional: function to validate response content
  
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
  
  # MSYS path conversion disabled via export at top of file
  curl -s -v -X "$method" "${headers[@]}" -d "$body" "$BASE_URL$endpoint" \
    -D "$status_file" -o "$temp_file" 2> curl_debug.log
  
  local curl_exit=$?
  if [ $curl_exit -ne 0 ]; then
    write_test_log "ERROR" "curl failed with exit code $curl_exit"
    cat curl_debug.log
  fi

  response_body=$(cat "$temp_file")
  if [ ! -f "$status_file" ]; then
     write_test_log "ERROR" "status_file not found"
  else
     # Debug: print first line of status file
     head -n 1 "$status_file" > status_debug.log
  fi
  http_status=$(head -n 1 "$status_file" | cut -d' ' -f2)
  
  rm -f "$temp_file" "$status_file"

  local end_req_time=$(date +%s%3N)
  local duration=$((end_req_time - start_req_time))

  local status="FAIL"
  local error_msg=""

  # First check HTTP status code
  if [[ "$http_status" =~ ^[0-9]+$ ]] && [ "$http_status" -eq "$expected_status" ]; then
    # If validation function is provided, run it
    if [[ -n "$validation_func" && "$(type -t "$validation_func")" == "function" ]]; then
      local validation_result
      validation_result=$($validation_func "$response_body" 2>&1)
      local validation_exit=$?
      
      if [ $validation_exit -eq 0 ]; then
        status="PASS"
        ((PASS_COUNT++))
        write_test_log "SUCCESS" "✓ $test_name - Sukses (Status: $http_status, Waktu: ${duration}ms)"
      else
        status="FAIL"
        ((FAIL_COUNT++))
        error_msg="Response validation failed: $validation_result"
        write_test_log "ERROR" "✗ $test_name - Gagal: $error_msg"
        write_test_log "ERROR" "  Response Body: $response_body"
        FAILED_TESTS_SUMMARY+=("✗ $test_name - $error_msg")
      fi
    else
      # No validation function, just check status code
      status="PASS"
      ((PASS_COUNT++))
      write_test_log "SUCCESS" "✓ $test_name - Sukses (Status: $http_status, Waktu: ${duration}ms)"
    fi
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

  result_json=$(jq -c -n --arg name "$test_name" --arg ep "$endpoint" --arg meth "$method" \
                        --arg stat "$status" --arg code "$http_status" --arg dur "$duration" \
                        --arg err "$error_msg" \
                        '{TestName: $name, Endpoint: $ep, Method: $meth, Status: $stat, StatusCode: $code, ResponseTimeMs: $dur, Error: $err}')
  # Append to in-memory array for same-shell calls
  TEST_RESULTS+=("$result_json")
  # Also append to file so subshell calls (via command substitution) are not lost
  printf "%s\n" "$result_json" >> "$RESULTS_FILE"
  # Ensure we always print valid JSON to avoid jq parse errors downstream
  if echo "$response_body" | jq . >/dev/null 2>&1; then
    printf "%s" "$response_body"
  else
    jq -n --arg raw "$response_body" '{raw: $raw}'
  fi
}

get_auth_token() {
  write_test_log "INFO" "Mengautentikasi test user..."
  local login_data
  login_data=$(jq -n --arg email "${TEST_EMAIL:-admin@example.com}" --arg pass "${TEST_PASSWORD:-password}" '{email: $email, password: $pass}')
  
  local temp_file=$(mktemp)
  local status_file=$(mktemp)
  
  local curl_debug_file=$(mktemp)

  # MSYS path conversion disabled via export at top of file
  curl -s -v -X "POST" -H "Content-Type: application/json" -d "$login_data" "$BASE_URL/v1/auth/login" \
    -D "$status_file" -o "$temp_file" 2> "$curl_debug_file"
  
  local response_body=$(cat "$temp_file")
  local http_status=$(head -n 1 "$status_file" | cut -d' ' -f2)
  
  rm -f "$temp_file" "$status_file"
  
  if [[ "$http_status" =~ ^[0-9]+$ ]] && [ "$http_status" -eq 200 ]; then
    if echo "$response_body" | jq . > /dev/null 2>&1; then
      AUTH_TOKEN=$(echo "$response_body" | jq -r '.data.token.access_token // empty')
      AUTH_USER_ID=$(echo "$response_body" | jq -r '.data.user.id // empty')
      if [[ -n "$AUTH_TOKEN" && "$AUTH_TOKEN" != "null" ]]; then
          write_test_log "SUCCESS" "Autentikasi berhasil"
          
          # Decode and print token payload
          local token_payload=$(echo "$AUTH_TOKEN" | cut -d. -f2 | tr -d '\n' | sed 's/-/+/g; s/_/\//g')
          # Add padding if needed
          local pad=$(( 4 - ${#token_payload} % 4 ))
          if [ $pad -ne 4 ]; then
            token_payload="${token_payload}$(printf '%*s' $pad | tr ' ' '=')"
          fi
          local decoded_payload=$(echo "$token_payload" | base64 -d 2>/dev/null)
          write_test_log "SUCCESS" "Token Payload: $decoded_payload"
          
          ((PASS_COUNT++))
      else
          write_test_log "ERROR" "Autentikasi gagal - token tidak ditemukan dalam response"
          AUTH_TOKEN=""
          ((FAIL_COUNT++))
          write_test_log "ERROR" "Curl Debug Output:"
          cat "$curl_debug_file"
          write_test_log "ERROR" "Response Body:"
          echo "$response_body"
      fi
    else
      write_test_log "ERROR" "Autentikasi gagal - response bukan JSON valid"
      AUTH_TOKEN=""
      ((FAIL_COUNT++))
      write_test_log "ERROR" "Curl Debug Output:"
      cat "$curl_debug_file"
      write_test_log "ERROR" "Response Body:"
      echo "$response_body"
    fi
  else
    write_test_log "ERROR" "Login gagal dengan status: $http_status"
    AUTH_TOKEN=""
    ((FAIL_COUNT++))
    write_test_log "ERROR" "Curl Debug Output:"
    cat "$curl_debug_file"
    write_test_log "ERROR" "Response Body:"
    echo "$response_body"
  fi
  rm -f "$curl_debug_file" # Clean up debug file
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

  # Optional debug: show results file path and a preview when DEBUG_RESULTS=1
  if [[ "$DEBUG_RESULTS" = "1" || "$DEBUG_RESULTS" = "true" ]]; then
    printf "${YELLOW}Debug: RESULTS_FILE=${NC} %s\n" "$RESULTS_FILE"
    if [[ -f "$RESULTS_FILE" ]]; then
      printf "${YELLOW}Debug: RESULTS_FILE size=${NC} %s bytes\n" "$(wc -c < "$RESULTS_FILE" 2>/dev/null || echo 0)"
      printf "${YELLOW}Debug: RESULTS_FILE head (up to 5 lines):${NC}\n"
      head -n 5 "$RESULTS_FILE" | sed 's/^/  /'
    else
      printf "${YELLOW}Debug: RESULTS_FILE does not exist${NC}\n"
    fi
    printf "\n"
  fi

  # Detailed API results (from test_api_endpoint calls only)
  # Prefer the persisted file so subshell calls are included
  local api_total=0
  local api_pass=0
  local api_fail=0
  if [[ -s "$RESULTS_FILE" ]]; then
    # shellcheck disable=SC2162
    while IFS= read -r r; do
      [[ -z "$r" ]] && continue
      # Skip non-JSON or malformed lines to avoid jq errors
      if ! echo "$r" | jq -e 'type=="object" and has("Status")' >/dev/null 2>&1; then
        continue
      fi
      ((api_total++))
      local st
      st=$(echo "$r" | jq -r '.Status')
      if [[ "$st" == "PASS" ]]; then
        ((api_pass++))
      else
        ((api_fail++))
      fi
    done < "$RESULTS_FILE"
  else
    # Fallback to in-memory array (should be rare)
    api_total=${#TEST_RESULTS[@]}
    for r in "${TEST_RESULTS[@]}"; do
      local st
      st=$(echo "$r" | jq -r '.Status')
      if [[ "$st" == "PASS" ]]; then
        ((api_pass++))
      else
        ((api_fail++))
      fi
    done
  fi

  if [ "$api_total" -gt 0 ]; then
    printf "API Requests: %d (Passed: %d, Failed: %d)\n" "$api_total" "$api_pass" "$api_fail"
    printf "\n${BLUE}API Results:${NC}\n"
    if [[ -s "$RESULTS_FILE" ]]; then
      # shellcheck disable=SC2162
      while IFS= read -r r; do
        [[ -z "$r" ]] && continue
        if ! echo "$r" | jq -e 'type=="object" and has("Status")' >/dev/null 2>&1; then
          continue
        fi
        local name method ep status code dur
        name=$(echo "$r" | jq -r '.TestName')
        method=$(echo "$r" | jq -r '.Method')
        ep=$(echo "$r" | jq -r '.Endpoint')
        status=$(echo "$r" | jq -r '.Status')
        code=$(echo "$r" | jq -r '.StatusCode')
        dur=$(echo "$r" | jq -r '.ResponseTimeMs')
        if [[ "$status" == "PASS" ]]; then
          printf "  ${GREEN}[%s]${NC} %s %s (status: %s, time: %sms) — %s\n" "$status" "$method" "$ep" "$code" "$dur" "$name"
        else
          printf "  ${RED}[%s]${NC} %s %s (status: %s, time: %sms) — %s\n" "$status" "$method" "$ep" "$code" "$dur" "$name"
        fi
      done < "$RESULTS_FILE"
    else
      for r in "${TEST_RESULTS[@]}"; do
        local name method ep status code dur
        name=$(echo "$r" | jq -r '.TestName')
        method=$(echo "$r" | jq -r '.Method')
        ep=$(echo "$r" | jq -r '.Endpoint')
        status=$(echo "$r" | jq -r '.Status')
        code=$(echo "$r" | jq -r '.StatusCode')
        dur=$(echo "$r" | jq -r '.ResponseTimeMs')
        if [[ "$status" == "PASS" ]]; then
          printf "  ${GREEN}[%s]${NC} %s %s (status: %s, time: %sms) — %s\n" "$status" "$method" "$ep" "$code" "$dur" "$name"
        else
          printf "  ${RED}[%s]${NC} %s %s (status: %s, time: %sms) — %s\n" "$status" "$method" "$ep" "$code" "$dur" "$name"
        fi
      done
    fi
    printf "\n"
  fi

  # Align global PASS/FAIL counters with computed API results so exit codes reflect failures
  # This ensures failures inside subshells are not ignored
  if [ "$api_total" -gt 0 ]; then
    PASS_COUNT=$api_pass
    FAIL_COUNT=$api_fail
  fi
  
  if [ "$FAIL_COUNT" -gt 0 ]; then
    printf "${RED}Failed Tests:${NC}\n"
    for summary in "${FAILED_TESTS_SUMMARY[@]}"; do
      printf "  %s\n" "$summary"
    done
    printf "\n"
  fi
}
