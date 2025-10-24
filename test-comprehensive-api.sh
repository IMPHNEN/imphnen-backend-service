#!/bin/bash

# ==============================================================================
# IMPHNEN API Comprehensive Test Suite - Extended Coverage
# Tests untuk semua endpoint yang belum tercakup di test.sh
# ==============================================================================

# Source the main test.sh for shared variables and functions
# Assuming test.sh exports needed variables

BASE_URL="${BASE_URL:-http://127.0.0.1:4099}"
AUTH_TOKEN=""

# Colors
CYAN='\033[0;36m'
YELLOW='\033[0;33m'
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m'

PASS_COUNT=0
FAIL_COUNT=0
TEST_RESULTS=()
FAILED_TESTS_SUMMARY=()

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

  printf "[$(date +'%H:%M:%S')] [${color}%-7s${NC}] %s\n" "$level" "$message" >&2
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
  login_data=$(jq -n '{email: "admin@example.com", password: "password"}')
  
  local temp_file=$(mktemp)
  local status_file=$(mktemp)
  
  curl -s -X "POST" -H "Content-Type: application/json" -d "$login_data" "$BASE_URL/v1/auth/login" \
    -D "$status_file" -o "$temp_file"
  
  local response_body=$(cat "$temp_file")
  local http_status=$(head -n 1 "$status_file" | cut -d' ' -f2)
  
  rm -f "$temp_file" "$status_file"
  
  if [[ "$http_status" =~ ^[0-9]+$ ]] && [ "$http_status" -eq 200 ]; then
    AUTH_TOKEN=$(echo "$response_body" | jq -r '.data.token.access_token // empty')
    if [[ -n "$AUTH_TOKEN" ]]; then
      write_test_log "SUCCESS" "Autentikasi berhasil"
      ((PASS_COUNT++))
    else
      write_test_log "ERROR" "Token tidak ditemukan"
      ((FAIL_COUNT++))
    fi
  else
    write_test_log "ERROR" "Login gagal dengan status: $http_status"
    ((FAIL_COUNT++))
  fi
}

# ==============================================================================
# IAM ENDPOINTS - AUTH
# ==============================================================================
test_auth_comprehensive() {
  printf "\n${CYAN}=== Testing Auth Endpoints (Comprehensive) ===${NC}\n"
  
  # Login
  local login_data=$(jq -n '{email: "admin@example.com", password: "password"}')
  test_api_endpoint "POST /v1/auth/login" "POST" "/v1/auth/login" 200 "$login_data" false
  
  # Login Mentor
  local mentor_login=$(jq -n '{email: "mentor@example.com", password: "password"}')
  test_api_endpoint "POST /v1/auth/login-mentor" "POST" "/v1/auth/login-mentor" 200 "$mentor_login" false
  
  # Register (will fail without valid data, but tests endpoint)
  local register_data=$(jq -n '{
    email: "newuser'$(date +%s)'@test.com",
    password: "Password123!",
    fullname: "Test User",
    phone_number: "081234567890"
  }')
  test_api_endpoint "POST /v1/auth/register" "POST" "/v1/auth/register" 200 "$register_data" false
  
  # Verify Email (expect failure with fake OTP)
  local verify_data=$(jq -n '{email: "test@test.com", otp: 123456}')
  test_api_endpoint "POST /v1/auth/verify-email" "POST" "/v1/auth/verify-email" 401 "$verify_data" false
  
  # Resend OTP
  local resend_data=$(jq -n '{email: "admin@example.com"}')
  test_api_endpoint "POST /v1/auth/send-otp" "POST" "/v1/auth/send-otp" 200 "$resend_data" false
  
  # Forgot Password
  test_api_endpoint "POST /v1/auth/forgot" "POST" "/v1/auth/forgot" 200 "$resend_data" false
  
  # New Password (expect failure with invalid token)
  local new_pass_data=$(jq -n '{token: "invalid", password: "NewPass123!"}')
  test_api_endpoint "POST /v1/auth/new-password" "POST" "/v1/auth/new-password" 400 "$new_pass_data" false
  
  # Refresh Token (get token first)
  local refresh_token=$(curl -s -X POST -H "Content-Type: application/json" \
    -d "$login_data" "$BASE_URL/v1/auth/login" | jq -r '.data.token.refresh_token // empty')
  
  if [ -n "$refresh_token" ]; then
    local refresh_data=$(jq -n --arg token "$refresh_token" '{refresh_token: $token}')
    test_api_endpoint "POST /v1/auth/refresh" "POST" "/v1/auth/refresh" 200 "$refresh_data" false
  fi
  
  # Logout
  test_api_endpoint "POST /v1/auth/logout" "POST" "/v1/auth/logout" 200 "" true
}

# ==============================================================================
# IAM ENDPOINTS - USERS
# ==============================================================================
test_users_comprehensive() {
  printf "\n${CYAN}=== Testing Users Endpoints (Comprehensive) ===${NC}\n"
  
  # Get Users List
  test_api_endpoint "GET /v1/users" "GET" "/v1/users" 200 "" true
  test_api_endpoint "GET /v1/users?page=1&limit=10" "GET" "/v1/users?page=1&limit=10" 200 "" true
  test_api_endpoint "GET /v1/users?search=admin" "GET" "/v1/users?search=admin" 200 "" true
  test_api_endpoint "GET /v1/users?sort_by=created_at&order=DESC" "GET" "/v1/users?sort_by=created_at&order=DESC" 200 "" true
  
  # Get User Me
  test_api_endpoint "GET /v1/users/me" "GET" "/v1/users/me" 200 "" true
  
  # Update User Me
  local update_me_data=$(jq -n '{
    fullname: "Updated Admin",
    phone_number: "081234567890",
    gender: "Male",
    birthdate: "1990-01-01"
  }')
  test_api_endpoint "PUT /v1/users/me" "PUT" "/v1/users/me" 200 "$update_me_data" true
  
  # Get User By ID (use a known ID from seed data)
  local test_user_id="c3b1d6a8-8d4f-4b36-b789-2e532ec7a7b2"
  test_api_endpoint "GET /v1/users/detail/:id" "GET" "/v1/users/detail/$test_user_id" 200 "" true
  
  # Create User
  local create_user_data=$(jq -n '{
    email: "testuser'$(date +%s)'@test.com",
    password: "Password123!",
    fullname: "Test User Created",
    phone_number: "081234567891",
    is_active: true,
    role_id: "5713cb37-dc02-4e87-8048-d7a41d352059"
  }')
  local create_response=$(test_api_endpoint "POST /v1/users/create" "POST" "/v1/users/create" 201 "$create_user_data" true)
  local created_user_id=$(echo "$create_response" | jq -r '.data.id // empty')
  
  if [ -n "$created_user_id" ]; then
    # Update User
    local update_user_data=$(jq -n '{
      email: "updated'$(date +%s)'@test.com",
      fullname: "Updated Test User",
      phone_number: "081234567892",
      is_active: true,
      role_id: "5713cb37-dc02-4e87-8048-d7a41d352059"
    }')
    test_api_endpoint "PUT /v1/users/update/:id" "PUT" "/v1/users/update/$created_user_id" 200 "$update_user_data" true
    
    # Activate/Deactivate User
    local activate_data=$(jq -n '{is_active: false}')
    test_api_endpoint "PATCH /v1/users/activate/:id" "PATCH" "/v1/users/activate/$created_user_id" 200 "$activate_data" true
    
    local reactivate_data=$(jq -n '{is_active: true}')
    test_api_endpoint "PATCH /v1/users/activate/:id (reactivate)" "PATCH" "/v1/users/activate/$created_user_id" 200 "$reactivate_data" true
    
    # Delete User
    test_api_endpoint "DELETE /v1/users/delete/:id" "DELETE" "/v1/users/delete/$created_user_id" 200 "" true
  fi
  
  # Upload File (requires multipart, skip for now)
  # test_api_endpoint "POST /v1/users/upload" "POST" "/v1/users/upload" 200 "" true
}

# ==============================================================================
# IAM ENDPOINTS - ROLES
# ==============================================================================
test_roles_comprehensive() {
  printf "\n${CYAN}=== Testing Roles Endpoints (Comprehensive) ===${NC}\n"
  
  # Get Roles List
  test_api_endpoint "GET /v1/roles" "GET" "/v1/roles" 200 "" true
  test_api_endpoint "GET /v1/roles?page=1&limit=10" "GET" "/v1/roles?page=1&limit=10" 200 "" true
  
  # Get Role By ID
  local test_role_id="5713cb37-dc02-4e87-8048-d7a41d352059"
  test_api_endpoint "GET /v1/roles/:id" "GET" "/v1/roles/$test_role_id" 200 "" true
  
  # Create Role
  local create_role_data=$(jq -n '{
    name: "Test Role '$(date +%s)'",
    description: "Test role description",
    permissions: []
  }')
  local create_role_response=$(test_api_endpoint "POST /v1/roles" "POST" "/v1/roles" 201 "$create_role_data" true)
  local created_role_id=$(echo "$create_role_response" | jq -r '.data.id // empty')
  
  if [ -n "$created_role_id" ]; then
    # Update Role
    local update_role_data=$(jq -n '{
      name: "Updated Test Role",
      description: "Updated description",
      permissions: []
    }')
    test_api_endpoint "PUT /v1/roles/:id" "PUT" "/v1/roles/$created_role_id" 200 "$update_role_data" true
    
    # Delete Role
    test_api_endpoint "DELETE /v1/roles/:id" "DELETE" "/v1/roles/$created_role_id" 200 "" true
  fi
}

# ==============================================================================
# IAM ENDPOINTS - PERMISSIONS
# ==============================================================================
test_permissions_comprehensive() {
  printf "\n${CYAN}=== Testing Permissions Endpoints (Comprehensive) ===${NC}\n"
  
  # Get Permissions List
  test_api_endpoint "GET /v1/permissions" "GET" "/v1/permissions" 200 "" true
  test_api_endpoint "GET /v1/permissions?page=1&limit=10" "GET" "/v1/permissions?page=1&limit=10" 200 "" true
  
  # Get Permission By ID (use known ID)
  local test_perm_id="00000000-0000-0000-0000-000000000001"
  test_api_endpoint "GET /v1/permissions/:id" "GET" "/v1/permissions/$test_perm_id" 200 "" true
  
  # Create Permission
  local create_perm_data=$(jq -n '{
    name: "Test Permission '$(date +%s)'",
    description: "Test permission description"
  }')
  local create_perm_response=$(test_api_endpoint "POST /v1/permissions" "POST" "/v1/permissions" 201 "$create_perm_data" true)
  local created_perm_id=$(echo "$create_perm_response" | jq -r '.data.id // empty')
  
  if [ -n "$created_perm_id" ]; then
    # Update Permission
    local update_perm_data=$(jq -n '{
      name: "Updated Test Permission",
      description: "Updated description"
    }')
    test_api_endpoint "PUT /v1/permissions/:id" "PUT" "/v1/permissions/$created_perm_id" 200 "$update_perm_data" true
    
    # Delete Permission
    test_api_endpoint "DELETE /v1/permissions/:id" "DELETE" "/v1/permissions/$created_perm_id" 200 "" true
  fi
}

# ==============================================================================
# IAM ENDPOINTS - TEAMS
# ==============================================================================
test_teams_comprehensive() {
  printf "\n${CYAN}=== Testing Teams Endpoints (Comprehensive) ===${NC}\n"
  
  # Admin Endpoints
  test_api_endpoint "GET /v1/teams/admin" "GET" "/v1/teams/admin" 200 "" true
  test_api_endpoint "GET /v1/teams/admin?page=1&limit=10" "GET" "/v1/teams/admin?page=1&limit=10" 200 "" true
  
  # Public Endpoints
  test_api_endpoint "GET /v1/teams" "GET" "/v1/teams" 200 "" false
  test_api_endpoint "GET /v1/teams?search=test" "GET" "/v1/teams?search=test" 200 "" false
  test_api_endpoint "GET /v1/teams/search?query=dev" "GET" "/v1/teams/search?query=dev" 200 "" false
  
  # Get Team By ID
  local test_team_id="team-001"
  test_api_endpoint "GET /v1/teams/admin/:id" "GET" "/v1/teams/admin/$test_team_id" 200 "" true
  test_api_endpoint "GET /v1/teams/admin/:id/members" "GET" "/v1/teams/admin/$test_team_id/members" 200 "" true
  
  # Create Team
  local create_team_data=$(jq -n '{
    name: "Test Team '$(date +%s)'",
    description: "Test team description",
    is_open: true,
    max_members: 5
  }')
  local create_team_response=$(test_api_endpoint "POST /v1/teams/admin" "POST" "/v1/teams/admin" 201 "$create_team_data" true)
  local created_team_id=$(echo "$create_team_response" | jq -r '.data.id // empty')
  
  if [ -n "$created_team_id" ]; then
    # Update Team
    local update_team_data=$(jq -n '{
      name: "Updated Test Team",
      description: "Updated description",
      is_open: false,
      max_members: 10
    }')
    test_api_endpoint "PUT /v1/teams/admin/:id" "PUT" "/v1/teams/admin/$created_team_id" 200 "$update_team_data" true
    
    # Invite Members
    local invite_data=$(jq -n '{
      user_ids: ["c3b1d6a8-8d4f-4b36-b789-2e532ec7a7b2"]
    }')
    test_api_endpoint "POST /v1/teams/admin/:id/invite" "POST" "/v1/teams/admin/$created_team_id/invite" 200 "$invite_data" true
    
    # Delete Team
    test_api_endpoint "DELETE /v1/teams/admin/:id" "DELETE" "/v1/teams/admin/$created_team_id" 200 "" true
  fi
}

# ==============================================================================
# DIMENTORIN ENDPOINTS - MENTORS
# ==============================================================================
test_mentors_comprehensive() {
  printf "\n${CYAN}=== Testing Mentors Endpoints (Comprehensive) ===${NC}\n"
  
  # Get Mentors List
  test_api_endpoint "GET /v1/mentors" "GET" "/v1/mentors" 200 "" true
  test_api_endpoint "GET /v1/mentors?page=1&limit=10" "GET" "/v1/mentors?page=1&limit=10" 200 "" true
  test_api_endpoint "GET /v1/mentors?search=mentor" "GET" "/v1/mentors?search=mentor" 200 "" true
  
  # Get Mentor By ID
  local test_mentor_id="mentor-001"
  test_api_endpoint "GET /v1/mentors/:id" "GET" "/v1/mentors/$test_mentor_id" 200 "" true
  
  # Get Mentor Me (requires mentor token)
  # test_api_endpoint "GET /v1/mentors/me" "GET" "/v1/mentors/me" 200 "" true
  
  # Get Mentor Status
  # test_api_endpoint "GET /v1/mentors/status" "GET" "/v1/mentors/status" 200 "" true
  
  # Register Mentor
  local register_mentor_data=$(jq -n '{
    expertise: ["Rust", "Backend"],
    bio: "Test mentor bio",
    linkedin_url: "https://linkedin.com/in/test",
    github_url: "https://github.com/test",
    portfolio_url: "https://test.com"
  }')
  # test_api_endpoint "POST /v1/mentors/register" "POST" "/v1/mentors/register" 201 "$register_mentor_data" true
  
  # Update Mentor (admin)
  # local update_mentor_data=$(jq -n '{...}')
  # test_api_endpoint "PUT /v1/mentors/:id" "PUT" "/v1/mentors/$test_mentor_id" 200 "$update_mentor_data" true
  
  # Verify Mentor (admin)
  local verify_data=$(jq -n '{is_verified: true}')
  test_api_endpoint "PUT /v1/mentors/:id/verify" "PUT" "/v1/mentors/$test_mentor_id/verify" 200 "$verify_data" true
  
  # Delete Mentor (admin)
  # test_api_endpoint "DELETE /v1/mentors/:id" "DELETE" "/v1/mentors/$test_mentor_id" 200 "" true
}

# ==============================================================================
# CMS ENDPOINTS - EVENTS
# ==============================================================================
test_events_comprehensive() {
  printf "\n${CYAN}=== Testing Events Endpoints (Comprehensive) ===${NC}\n"
  
  # Public Endpoints
  test_api_endpoint "GET /v1/cms/landing/events" "GET" "/v1/cms/landing/events" 200 "" false
  test_api_endpoint "GET /v1/cms/landing/events?page=1&limit=10" "GET" "/v1/cms/landing/events?page=1&limit=10" 200 "" false
  test_api_endpoint "GET /v1/cms/landing/events?search=test" "GET" "/v1/cms/landing/events?search=test" 200 "" false
  
  # Get Event By ID
  # Need to get an event ID first
  local events_response=$(curl -s "$BASE_URL/v1/cms/landing/events")
  local test_event_id=$(echo "$events_response" | jq -r '.data[0].id // empty')
  
  if [ -n "$test_event_id" ]; then
    test_api_endpoint "GET /v1/cms/landing/events/:id" "GET" "/v1/cms/landing/events/$test_event_id" 200 "" false
  fi
  
  # Create Event (protected)
  local create_event_data=$(jq -n '{
    title: "Test Event '$(date +%s)'",
    description: "Test event description",
    event_date: "'$(date -u +%Y-%m-%dT%H:%M:%SZ)'",
    location: "Online",
    image_url: "https://example.com/image.jpg",
    is_online: true
  }')
  local create_event_response=$(test_api_endpoint "POST /v1/cms/landing/events/create" "POST" "/v1/cms/landing/events/create" 201 "$create_event_data" true)
  local created_event_id=$(echo "$create_event_response" | jq -r '.data.id // empty')
  
  if [ -n "$created_event_id" ]; then
    # Update Event
    local update_event_data=$(jq -n '{
      title: "Updated Test Event",
      description: "Updated description",
      event_date: "'$(date -u +%Y-%m-%dT%H:%M:%SZ)'",
      location: "Jakarta",
      is_online: false
    }')
    test_api_endpoint "PATCH /v1/cms/landing/events/:id" "PATCH" "/v1/cms/landing/events/$created_event_id" 200 "$update_event_data" true
    
    # Delete Event
    test_api_endpoint "DELETE /v1/cms/landing/events/:id" "DELETE" "/v1/cms/landing/events/$created_event_id" 200 "" true
  fi
}

# ==============================================================================
# CMS ENDPOINTS - TESTIMONIALS
# ==============================================================================
test_testimonials_comprehensive() {
  printf "\n${CYAN}=== Testing Testimonials Endpoints (Comprehensive) ===${NC}\n"
  
  # Public Endpoints
  test_api_endpoint "GET /v1/cms/landing/testimonials" "GET" "/v1/cms/landing/testimonials" 200 "" false
  test_api_endpoint "GET /v1/cms/landing/testimonials?page=1&limit=10" "GET" "/v1/cms/landing/testimonials?page=1&limit=10" 200 "" false
  
  # Get Testimonial By ID
  local testimonials_response=$(curl -s "$BASE_URL/v1/cms/landing/testimonials")
  local test_testimonial_id=$(echo "$testimonials_response" | jq -r '.data[0].id // empty')
  
  if [ -n "$test_testimonial_id" ]; then
    test_api_endpoint "GET /v1/cms/landing/testimonials/:id" "GET" "/v1/cms/landing/testimonials/$test_testimonial_id" 200 "" false
  fi
  
  # Create Testimonial (protected)
  local create_testimonial_data=$(jq -n '{
    role: "Student",
    content: "Test testimonial content '$(date +%s)'"
  }')
  local create_testimonial_response=$(test_api_endpoint "POST /v1/cms/landing/testimonials/create" "POST" "/v1/cms/landing/testimonials/create" 201 "$create_testimonial_data" true)
  local created_testimonial_id=$(echo "$create_testimonial_response" | jq -r '.data.id // empty')
  
  if [ -n "$created_testimonial_id" ]; then
    # Update Testimonial
    local update_testimonial_data=$(jq -n '{
      role: "Alumni",
      content: "Updated testimonial content"
    }')
    test_api_endpoint "PATCH /v1/cms/landing/testimonials/:id" "PATCH" "/v1/cms/landing/testimonials/$created_testimonial_id" 200 "$update_testimonial_data" true
    
    # Delete Testimonial
    test_api_endpoint "DELETE /v1/cms/landing/testimonials/:id" "DELETE" "/v1/cms/landing/testimonials/$created_testimonial_id" 200 "" true
  fi
}

# ==============================================================================
# GACHA ENDPOINTS
# ==============================================================================
test_gacha_comprehensive() {
  printf "\n${CYAN}=== Testing Gacha Endpoints (Comprehensive) ===${NC}\n"
  
  # Gacha Items
  test_api_endpoint "GET /v1/gacha/items" "GET" "/v1/gacha/items" 200 "" true
  test_api_endpoint "GET /v1/gacha/items?page=1&limit=10" "GET" "/v1/gacha/items?page=1&limit=10" 200 "" true
  
  # Get Gacha Item By ID
  local items_response=$(curl -s -H "Authorization: Bearer $AUTH_TOKEN" "$BASE_URL/v1/gacha/items")
  local test_item_id=$(echo "$items_response" | jq -r '.data[0].id // empty')
  
  if [ -n "$test_item_id" ]; then
    test_api_endpoint "GET /v1/gacha/items/:id" "GET" "/v1/gacha/items/$test_item_id" 200 "" true
  fi
  
  # Create Gacha Item
  local create_item_data=$(jq -n '{
    name: "Test Item '$(date +%s)'",
    description: "Test item description",
    rarity: "COMMON",
    image_url: "https://example.com/item.jpg",
    weight: 100
  }')
  local create_item_response=$(test_api_endpoint "POST /v1/gacha/items" "POST" "/v1/gacha/items" 201 "$create_item_data" true)
  local created_item_id=$(echo "$create_item_response" | jq -r '.data.id // empty')
  
  if [ -n "$created_item_id" ]; then
    # Update Gacha Item
    local update_item_data=$(jq -n '{
      name: "Updated Test Item",
      description: "Updated description",
      rarity: "RARE",
      weight: 50
    }')
    test_api_endpoint "PUT /v1/gacha/items/:id" "PUT" "/v1/gacha/items/$created_item_id" 200 "$update_item_data" true
    
    # Delete Gacha Item
    test_api_endpoint "DELETE /v1/gacha/items/:id" "DELETE" "/v1/gacha/items/$created_item_id" 200 "" true
  fi
  
  # Gacha Rolls
  test_api_endpoint "POST /v1/gacha/rolls" "POST" "/v1/gacha/rolls" 201 "{}" true
  test_api_endpoint "POST /v1/gacha/rolls/execute" "POST" "/v1/gacha/rolls/execute" 200 "{}" true
  
  # Gacha Credits (internal endpoints, may require special auth)
  # test_api_endpoint "GET /v1/gacha/credits" "GET" "/v1/gacha/credits" 200 "" true
  # test_api_endpoint "POST /v1/gacha/credits/add" "POST" "/v1/gacha/credits/add" 200 '{"amount": 10}' true
  # test_api_endpoint "POST /v1/gacha/credits/consume" "POST" "/v1/gacha/credits/consume" 200 '{"amount": 1}' true
  
  # Gacha Claims
  # test_api_endpoint "POST /v1/gacha/claims" "POST" "/v1/gacha/claims" 201 "{}" true
}

# ==============================================================================
# HACKATHON ENDPOINTS
# ==============================================================================
test_hackathon_comprehensive() {
  printf "\n${CYAN}=== Testing Hackathon Endpoints (Comprehensive) ===${NC}\n"
  
  # Get Hackathons
  test_api_endpoint "GET /v1/hackathons" "GET" "/v1/hackathons" 200 "" false
  test_api_endpoint "GET /v1/hackathons?page=1&limit=10" "GET" "/v1/hackathons?page=1&limit=10" 200 "" false
  
  # Create Hackathon
  local create_hackathon_data=$(jq -n '{
    title: "Test Hackathon '$(date +%s)'",
    description: "Test hackathon description",
    start_date: "'$(date -u +%Y-%m-%dT%H:%M:%SZ)'",
    end_date: "'$(date -u -d '+7 days' +%Y-%m-%dT%H:%M:%SZ)'",
    registration_deadline: "'$(date -u -d '+1 day' +%Y-%m-%dT%H:%M:%SZ)'",
    max_teams: 100,
    max_team_size: 5
  }')
  local create_hackathon_response=$(test_api_endpoint "POST /v1/hackathons" "POST" "/v1/hackathons" 201 "$create_hackathon_data" true)
  local created_hackathon_id=$(echo "$create_hackathon_response" | jq -r '.data.id // empty')
  
  if [ -n "$created_hackathon_id" ]; then
    # Get Hackathon By ID
    test_api_endpoint "GET /v1/hackathons/:id" "GET" "/v1/hackathons/$created_hackathon_id" 200 "" false
    
    # Update Hackathon
    local update_hackathon_data=$(jq -n '{
      title: "Updated Test Hackathon",
      description: "Updated description",
      max_teams: 150
    }')
    test_api_endpoint "PUT /v1/hackathons/:id" "PUT" "/v1/hackathons/$created_hackathon_id" 200 "$update_hackathon_data" true
    
    # Hackathon Events
    local create_event_data=$(jq -n --arg hackathon_id "$created_hackathon_id" '{
      hackathon_id: $hackathon_id,
      title: "Test Event",
      description: "Test event description",
      event_date: "'$(date -u +%Y-%m-%dT%H:%M:%SZ)'",
      location: "Online",
      is_mandatory: false
    }')
    local create_event_response=$(test_api_endpoint "POST /v1/hackathons/:id/events" "POST" "/v1/hackathons/$created_hackathon_id/events" 201 "$create_event_data" true)
    local created_event_id=$(echo "$create_event_response" | jq -r '.data.id // empty')
    
    if [ -n "$created_event_id" ]; then
      # Update Event
      local update_event_data=$(jq -n '{
        title: "Updated Test Event",
        is_mandatory: true
      }')
      test_api_endpoint "PUT /v1/hackathons/events/:id" "PUT" "/v1/hackathons/events/$created_event_id" 200 "$update_event_data" true
      
      # Delete Event
      test_api_endpoint "DELETE /v1/hackathons/events/:id" "DELETE" "/v1/hackathons/events/$created_event_id" 200 "" true
    fi
    
    # Hackathon Timeline
    local create_timeline_data=$(jq -n --arg hackathon_id "$created_hackathon_id" '{
      hackathon_id: $hackathon_id,
      phase_name: "Registration",
      description: "Registration phase",
      start_date: "'$(date -u +%Y-%m-%dT%H:%M:%SZ)'",
      end_date: "'$(date -u -d '+1 day' +%Y-%m-%dT%H:%M:%SZ)'",
      allowed_operations: ["REGISTER"]
    }')
    local create_timeline_response=$(test_api_endpoint "POST /v1/hackathons/:id/timeline" "POST" "/v1/hackathons/$created_hackathon_id/timeline" 201 "$create_timeline_data" true)
    local created_timeline_id=$(echo "$create_timeline_response" | jq -r '.data.id // empty')
    
    if [ -n "$created_timeline_id" ]; then
      # Update Timeline
      local update_timeline_data=$(jq -n '{
        phase_name: "Updated Registration",
        description: "Updated description"
      }')
      test_api_endpoint "PUT /v1/hackathons/timeline/:id" "PUT" "/v1/hackathons/timeline/$created_timeline_id" 200 "$update_timeline_data" true
      
      # Delete Timeline
      test_api_endpoint "DELETE /v1/hackathons/timeline/:id" "DELETE" "/v1/hackathons/timeline/$created_timeline_id" 200 "" true
    fi
    
    # Hackathon Submissions
    # test_api_endpoint "GET /v1/hackathons/:id/submissions" "GET" "/v1/hackathons/$created_hackathon_id/submissions" 200 "" true
    # test_api_endpoint "GET /v1/hackathons/submissions/me" "GET" "/v1/hackathons/submissions/me" 200 "" true
    
    # Admin Results
    # test_api_endpoint "GET /v1/hackathons/:id/results" "GET" "/v1/hackathons/$created_hackathon_id/results" 200 "" true
    
    # Public Results
    # test_api_endpoint "GET /v1/hackathons/:id/results/public" "GET" "/v1/hackathons/$created_hackathon_id/results/public" 200 "" false
    
    # Delete Hackathon
    test_api_endpoint "DELETE /v1/hackathons/:id" "DELETE" "/v1/hackathons/$created_hackathon_id" 200 "" true
  fi
}

# ==============================================================================
# MAIN EXECUTION
# ==============================================================================
main() {
  printf "\n${CYAN}========================================${NC}\n"
  printf "${CYAN}  IMPHNEN Comprehensive API Test Suite${NC}\n"
  printf "${CYAN}========================================${NC}\n\n"
  
  write_test_log "INFO" "Starting comprehensive API tests..."
  write_test_log "INFO" "Base URL: $BASE_URL"
  
  # Get authentication token first
  get_auth_token
  
  if [ -z "$AUTH_TOKEN" ]; then
    write_test_log "ERROR" "Failed to get auth token. Cannot proceed with protected endpoint tests."
    exit 1
  fi
  
  # Run all comprehensive tests
  test_auth_comprehensive
  test_users_comprehensive
  test_roles_comprehensive
  test_permissions_comprehensive
  test_teams_comprehensive
  test_mentors_comprehensive
  test_events_comprehensive
  test_testimonials_comprehensive
  test_gacha_comprehensive
  test_hackathon_comprehensive
  
  # Print summary
  local total_tests=$((PASS_COUNT + FAIL_COUNT))
  local success_rate=0
  if [ "$total_tests" -gt 0 ]; then
    success_rate=$(( (PASS_COUNT * 100) / total_tests ))
  fi
  
  printf "\n${CYAN}========================================${NC}\n"
  printf "${CYAN}  Test Summary${NC}\n"
  printf "${CYAN}========================================${NC}\n"
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
    exit 1
  else
    printf "${GREEN}All tests passed!${NC}\n\n"
    exit 0
  fi
}

# Run main function
main "$@"
