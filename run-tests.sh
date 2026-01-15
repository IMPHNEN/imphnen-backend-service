#!/bin/bash

# ==============================================================================
# IMPHNEN API Test Runner - Modular Test Suite
# ==============================================================================

# Disable MSYS path conversion for Windows compatibility
export MSYS_NO_PATHCONV=1

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BASE_URL="${BASE_URL:-http://127.0.0.1:4099}"
TEST_EMAIL="${TEST_EMAIL:-admin@example.com}"
TEST_PASSWORD="${TEST_PASSWORD:-password}"
SPECIFIC_SUITE=""
SERVER_PID=""

# Colors
CYAN='\033[0;36m'
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
YELLOW='\033[0;33m'
NC='\033[0m'

# Parse command line arguments
while getopts "s:" opt; do
  case $opt in
    s)
      SPECIFIC_SUITE="$OPTARG"
      ;;
    \?)
      echo "Usage: $0 [-s suite_name]"
      echo "  -s suite_name: Run only a specific test suite"
      echo "  Available suites: auth, users, roles, security, mentors, cms, gacha"
      exit 1
      ;;
  esac
done

# Export variables for child scripts
export BASE_URL TEST_EMAIL TEST_PASSWORD

# Ensure cargo is in PATH for all child processes
export PATH="$HOME/.cargo/bin:/c/Users/$USER/.cargo/bin:/c/msys64/home/$USER/.cargo/bin:$PATH"

# Extract port from BASE_URL to ensure server listens on the correct port
PORT=$(echo "$BASE_URL" | grep -oE ':[0-9]+$' | tr -d ':')
if [ -z "$PORT" ]; then
  PORT=4099
fi
export PORT

echo -e "${CYAN}"
cat << 'EOF'
╔═══════════════════════════════════════════════════════════════════════╗
║                    IMPHNEN API TEST SUITE                             ║
║                     Modular Test Runner                               ║
╚═══════════════════════════════════════════════════════════════════════╝
EOF
echo -e "${NC}"

echo -e "${BLUE}Configuration:${NC}"
echo -e "  Base URL: ${GREEN}$BASE_URL${NC}"
echo -e "  Test User: ${GREEN}$TEST_EMAIL${NC}"
if [ -n "$SPECIFIC_SUITE" ]; then
  echo -e "  Mode: ${YELLOW}Single Suite ($SPECIFIC_SUITE)${NC}"
else
  echo -e "  Mode: ${YELLOW}All Suites${NC}"
fi
echo ""

# ==============================================================================
# Start Server (ALWAYS)
# ==============================================================================

echo -e "${YELLOW}Starting API server...${NC}"

# Force kill any existing api processes first
echo -e "${CYAN}Cleaning up any existing API processes...${NC}"
if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" || "$OSTYPE" == "cygwin" ]]; then
  # Windows - use taskkill
  taskkill //F //IM api.exe 2>/dev/null || true
else
  # Linux/Mac - use kill
  ps aux | grep "target/release/api" | grep -v grep | awk '{print $2}' | xargs kill -9 2>/dev/null || true
  ps aux | grep "cargo run --bin api" | grep -v grep | awk '{print $2}' | xargs kill -9 2>/dev/null || true
fi
sleep 2

# Check if binary already exists - detect Windows environment
if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" || "$OSTYPE" == "cygwin" ]]; then
  API_BINARY="./target/release/api.exe"
else
  API_BINARY="./target/release/api"
fi

echo -e "${CYAN}Building all binaries in release mode...${NC}"
# Ensure cargo is in PATH
export PATH="$HOME/.cargo/bin:/c/Users/$USER/.cargo/bin:/c/msys64/home/$USER/.cargo/bin:$PATH"
rustup default stable
cargo build --release

if [ $? -ne 0 ]; then
  echo -e "${RED}Failed to compile binaries${NC}"
  exit 1
fi
  
echo -e "${CYAN}Starting server in background...${NC}"

# Detect OS and use appropriate binary
if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" || "$OSTYPE" == "cygwin" ]]; then
  # Windows - start .exe directly
  ./target/release/api.exe > server.log 2>&1 &
  SERVER_PID=$!
else
  # Linux/Mac - use nohup
  nohup ./target/release/api > server.log 2>&1 &
  SERVER_PID=$!
fi

echo -e "${CYAN}Server started with PID: $SERVER_PID${NC}"
  
# Wait for server to be ready
echo -e "${CYAN}Waiting for server to be ready...${NC}"
MAX_WAIT=30
WAIT_COUNT=0
while true; do
  # Check if any HTTP status code is returned (even 404/405 means server is up)
  HTTP_CODE=$(MSYS_NO_PATHCONV=1 curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/health" 2>/dev/null || echo "000")
  if [ "$HTTP_CODE" != "000" ] && [ "$HTTP_CODE" != "" ]; then
    break
  fi
  
  sleep 1
  ((WAIT_COUNT++))
  if [ $WAIT_COUNT -ge $MAX_WAIT ]; then
    echo -e "${RED}Server failed to start within $MAX_WAIT seconds${NC}"
    echo -e "${RED}Server log:${NC}"
    tail -20 server.log
    if [ -n "$SERVER_PID" ]; then
      if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" || "$OSTYPE" == "cygwin" ]]; then
        taskkill //F //PID $SERVER_PID 2>/dev/null || true
      else
        kill $SERVER_PID 2>/dev/null
      fi
    fi
    exit 1
  fi
  printf "."
done
echo ""
echo -e "${GREEN}✓ Server is ready!${NC}"

# Cleanup function
cleanup() {
  if [ -n "$SERVER_PID" ]; then
    echo -e "\n${YELLOW}Stopping server (PID: $SERVER_PID)...${NC}"
    if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" || "$OSTYPE" == "cygwin" ]]; then
      # Windows - use taskkill
      taskkill //F //PID $SERVER_PID 2>/dev/null || true
    else
      # Linux/Mac - use kill
      kill $SERVER_PID 2>/dev/null
      sleep 1
      # Force kill if still running
      if kill -0 $SERVER_PID 2>/dev/null; then
        kill -9 $SERVER_PID 2>/dev/null
      fi
    fi
    echo -e "${GREEN}✓ Server stopped${NC}"
  fi
}

# Set trap to cleanup on exit
trap cleanup EXIT INT TERM

# Run seeder to populate test data
echo -e "${CYAN}Running database schema creation...${NC}"
if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" || "$OSTYPE" == "cygwin" ]]; then
  # Windows - use existing binary directly
  if ./target/release/create_schema.exe; then
    echo -e "${GREEN}✓ Database schema created${NC}"
  else
    echo -e "${RED}✗ Database schema creation failed${NC}"
    cleanup
    exit 1
  fi
else
  # Linux/Mac - use cargo
  if cargo run --bin create_schema --release; then
    echo -e "${GREEN}✓ Database schema created${NC}"
  else
    echo -e "${RED}✗ Database schema creation failed${NC}"
    cleanup
    exit 1
  fi
fi

# Run seeder to populate test data
echo -e "${CYAN}Running database seeding...${NC}"
if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" || "$OSTYPE" == "cygwin" ]]; then
  # Windows - use existing binary directly
  if ./target/release/seeder.exe; then
    echo -e "${GREEN}✓ Database seeded${NC}"
  else
    echo -e "${RED}✗ Database seeding failed${NC}"
    cleanup
    exit 1
  fi
else
  # Linux/Mac - use cargo
  if cargo run --bin seeder --release; then
    echo -e "${GREEN}✓ Database seeded${NC}"
  else
    echo -e "${RED}✗ Database seeding failed${NC}"
    cleanup
    exit 1
  fi
fi

# Test suite tracking
declare -A SUITE_RESULTS
declare -A SUITE_TEST_COUNTS
TOTAL_SUITES=0
PASSED_SUITES=0
FAILED_SUITES=0
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

run_test_suite() {
  local suite_name=$1
  local test_script=$2
  
  ((TOTAL_SUITES++))
  
  printf "\n${CYAN}════════════════════════════════════════════════════════════════${NC}\n"
  printf "${BLUE}Running Test Suite: ${YELLOW}%s${NC}\n" "$suite_name"
  printf "${CYAN}════════════════════════════════════════════════════════════════${NC}\n"
  
  if [ ! -f "$test_script" ]; then
    printf "${RED}✗ Test script not found: %s${NC}\n" "$test_script"
    SUITE_RESULTS["$suite_name"]="NOT_FOUND"
    SUITE_TEST_COUNTS["$suite_name"]="0:0:0"
    ((FAILED_SUITES++))
    return 1
  fi
  
  # Make script executable
  chmod +x "$test_script"
  
  # Capture test output to extract test counts
  local output_file=$(mktemp)
  
  if bash "$test_script" 2>&1 | tee "$output_file"; then
    local script_exit_status=0
  else
    local script_exit_status=1
  fi
  
  # Extract test counts from output (prioritize Total Tests from summary)
  # Look for the test summary block specifically
  declare -i suite_total
  declare -i suite_passed
  declare -i suite_failed
  
  suite_total=$(grep -A 3 "=== Test Summary ===" "$output_file" | grep -oP "Total Tests: \K\d+" | tail -1 || echo "0")
  suite_passed=$(grep -A 3 "=== Test Summary ===" "$output_file" | grep -oP "Passed: \K\d+" | tail -1 || echo "0")
  suite_failed=$(grep -A 3 "=== Test Summary ===" "$output_file" | grep -oP "Failed: \K\d+" | tail -1 || echo "0")
  
  # If no Test Summary found, try API Requests line as fallback
  if [ "$suite_total" = "0" ]; then
    local api_line=$(grep -oP "API Requests: \K\d+ \(Passed: \d+, Failed: \d+\)" "$output_file" | tail -1 || echo "0 (Passed: 0, Failed: 0)")
    suite_total=$(echo "$api_line" | grep -oP "^\d+" || echo "0")
    suite_passed=$(echo "$api_line" | grep -oP "Passed: \K\d+" || echo "0")
    suite_failed=$(echo "$api_line" | grep -oP "Failed: \K\d+" || echo "0")
  fi

  local suite_exit=0
  if [ "$suite_failed" -gt 0 ] || [ "$script_exit_status" -ne 0 ]; then
    SUITE_RESULTS["$suite_name"]="FAILED"
    ((FAILED_SUITES++))
    printf "${RED}✗ Suite '%s' failed${NC}\n" "$suite_name"
    suite_exit=1
  else
    SUITE_RESULTS["$suite_name"]="PASSED"
    ((PASSED_SUITES++))
    printf "${GREEN}✓ Suite '%s' completed successfully${NC}\n" "$suite_name"
  fi
  # Store suite test counts
  SUITE_TEST_COUNTS["$suite_name"]="$suite_total:$suite_passed:$suite_failed"
  
  # Accumulate totals
  ((TOTAL_TESTS += suite_total))
  ((PASSED_TESTS += suite_passed))
  ((FAILED_TESTS += suite_failed))
  
  rm -f "$output_file"
  
  return $suite_exit
}

# ==============================================================================
# Run Test Suites
# ==============================================================================

START_TIME=$(date +%s)

# Determine which suites to run
if [ -n "$SPECIFIC_SUITE" ]; then
  # Run only the specified suite
  case "$SPECIFIC_SUITE" in
    auth)
      run_test_suite "IAM - Authentication" "$SCRIPT_DIR/tests/iam/test-auth.sh"
      ;;
    users)
      run_test_suite "IAM - Users" "$SCRIPT_DIR/tests/iam/test-users.sh"
      ;;
    roles)
      run_test_suite "IAM - Roles & Permissions" "$SCRIPT_DIR/tests/iam/test-roles-permissions.sh"
      ;;
    security)
      run_test_suite "IAM - Security & Authorization" "$SCRIPT_DIR/tests/iam/test-security.sh"
      ;;
    mentors)
      run_test_suite "Dimentorin - Mentors" "$SCRIPT_DIR/tests/dimentorin/test-mentors.sh"
      ;;
    cms)
      run_test_suite "CMS - Events & Testimonials" "$SCRIPT_DIR/tests/cms/test-cms.sh"
      ;;
    gacha)
      run_test_suite "Gacha - Items & Rolls" "$SCRIPT_DIR/tests/gacha/test-gacha.sh"
      ;;
    *)
      echo -e "${RED}Unknown suite: $SPECIFIC_SUITE${NC}"
      echo -e "${YELLOW}Available suites: auth, users, roles, security, mentors, cms, gacha${NC}"
      cleanup
      exit 1
      ;;
  esac
else
  # Run all suites
  run_test_suite "IAM - Authentication" "$SCRIPT_DIR/tests/iam/test-auth.sh"
  run_test_suite "IAM - Users" "$SCRIPT_DIR/tests/iam/test-users.sh"
  run_test_suite "IAM - Roles & Permissions" "$SCRIPT_DIR/tests/iam/test-roles-permissions.sh"
  run_test_suite "IAM - Security & Authorization" "$SCRIPT_DIR/tests/iam/test-security.sh"
  run_test_suite "Dimentorin - Mentors" "$SCRIPT_DIR/tests/dimentorin/test-mentors.sh"
  run_test_suite "CMS - Events & Testimonials" "$SCRIPT_DIR/tests/cms/test-cms.sh"
  run_test_suite "Gacha - Items & Rolls" "$SCRIPT_DIR/tests/gacha/test-gacha.sh"
fi

END_TIME=$(date +%s)
DURATION=$((END_TIME - START_TIME))

# ==============================================================================
# Final Summary
# ==============================================================================

printf "\n${CYAN}════════════════════════════════════════════════════════════════${NC}\n"
printf "${BLUE}                    FINAL TEST SUMMARY                          ${NC}\n"
printf "${CYAN}════════════════════════════════════════════════════════════════${NC}\n\n"

# Test Suites Summary
printf "${BLUE}Test Suites:${NC}\n"
printf "  Total Suites: ${BLUE}%d${NC}\n" "$TOTAL_SUITES"
printf "  ${GREEN}Passed Suites: %d${NC}\n" "$PASSED_SUITES"
printf "  ${RED}Failed Suites: %d${NC}\n" "$FAILED_SUITES"

if [ "$TOTAL_SUITES" -gt 0 ]; then
  SUCCESS_RATE=$(( (PASSED_SUITES * 100) / TOTAL_SUITES ))
  printf "  Suite Success Rate: ${BLUE}%d%%${NC}\n" "$SUCCESS_RATE"
fi

printf "\n"

# Individual Tests Summary
printf "${BLUE}Individual Tests:${NC}\n"
printf "  Total Tests: ${BLUE}%d${NC}\n" "$TOTAL_TESTS"
printf "  ${GREEN}Passed Tests: %d${NC}\n" "$PASSED_TESTS"
printf "  ${RED}Failed Tests: %d${NC}\n" "$FAILED_TESTS"

if [ "$TOTAL_TESTS" -gt 0 ]; then
  TEST_SUCCESS_RATE=$(( (PASSED_TESTS * 100) / TOTAL_TESTS ))
  printf "  Test Success Rate: ${BLUE}%d%%${NC}\n" "$TEST_SUCCESS_RATE"
fi

printf "\n"

printf "Total Duration: ${BLUE}%d seconds${NC}\n\n" "$DURATION"

# Print individual suite results
printf "${BLUE}Suite Results:${NC}\n"
for suite in "${!SUITE_RESULTS[@]}"; do
  result="${SUITE_RESULTS[$suite]}"
  if [ "$result" = "PASSED" ]; then
    printf "  ${GREEN}✓${NC} %s\n" "$suite"
  elif [ "$result" = "FAILED" ]; then
    printf "  ${RED}✗${NC} %s\n" "$suite"
  else
    printf "  ${YELLOW}?${NC} %s (${result})\n" "$suite"
  fi
done

printf "\n${CYAN}════════════════════════════════════════════════════════════════${NC}\n"

# Exit with appropriate code
if [ "$FAILED_SUITES" -eq 0 ]; then
  printf "\n${GREEN}All test suites passed! 🎉${NC}\n\n"
  exit 0
else
  printf "\n${RED}Some test suites failed. Please review the output above.${NC}\n\n"
  exit 1
fi
