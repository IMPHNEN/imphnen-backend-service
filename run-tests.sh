#!/bin/bash

# ==============================================================================
# IMPHNEN API Test Runner - Modular Test Suite
# ==============================================================================

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BASE_URL="${BASE_URL:-http://127.0.0.1:4099}"
TEST_EMAIL="${TEST_EMAIL:-admin@example.com}"
TEST_PASSWORD="${TEST_PASSWORD:-password}"
START_SERVER=false
SERVER_PID=""

# Colors
CYAN='\033[0;36m'
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
YELLOW='\033[0;33m'
NC='\033[0m'

# Parse command line arguments
while getopts "s" opt; do
  case $opt in
    s)
      START_SERVER=true
      ;;
    \?)
      echo "Usage: $0 [-s]"
      echo "  -s: Start the API server before running tests"
      exit 1
      ;;
  esac
done

# Export variables for child scripts
export BASE_URL TEST_EMAIL TEST_PASSWORD

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
echo ""

# ==============================================================================
# Start Server if requested
# ==============================================================================

if [ "$START_SERVER" = true ]; then
  echo -e "${YELLOW}Starting API server...${NC}"
  
  # Force kill any existing api processes first
  echo -e "${CYAN}Cleaning up any existing API processes...${NC}"
  ps aux | grep "target/release/api" | grep -v grep | awk '{print $1}' | xargs kill -9 2>/dev/null || true
  ps aux | grep "cargo run --bin api" | grep -v grep | awk '{print $1}' | xargs kill -9 2>/dev/null || true
  sleep 2
  
  echo -e "${CYAN}Building server in release mode...${NC}"
  cargo build --bin api --release
  
  if [ $? -ne 0 ]; then
    echo -e "${RED}Failed to compile server${NC}"
    exit 1
  fi
    
    echo -e "${CYAN}Starting server in background...${NC}"
    
    # Start server directly from binary in background
    nohup ./target/release/api > server.log 2>&1 &
    SERVER_PID=$!
    
    echo -e "${CYAN}Server started with PID: $SERVER_PID${NC}"
    
    # Wait for server to be ready
    echo -e "${CYAN}Waiting for server to be ready...${NC}"
    MAX_WAIT=30
    WAIT_COUNT=0
    while true; do
      # Check if any HTTP status code is returned (even 404/405 means server is up)
      HTTP_CODE=$(curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/v1/auth/login" 2>/dev/null || echo "000")
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
          kill $SERVER_PID 2>/dev/null
        fi
        exit 1
      fi
      printf "."
    done
    echo ""
    echo -e "${GREEN}✓ Server is ready!${NC}"
    
    # Run seeder to populate test data
    echo -e "${CYAN}Running database seeder...${NC}"
    cargo run --bin seeder --release > /dev/null 2>&1 || {
      echo -e "${YELLOW}⚠ Seeder failed or already populated${NC}"
    }
    echo -e "${GREEN}✓ Database seeded${NC}"
  echo ""
fi

# Cleanup function
cleanup() {
  if [ -n "$SERVER_PID" ] && [ "$START_SERVER" = true ]; then
    echo -e "\n${YELLOW}Stopping server (PID: $SERVER_PID)...${NC}"
    kill $SERVER_PID 2>/dev/null
    sleep 1
    # Force kill if still running
    if kill -0 $SERVER_PID 2>/dev/null; then
      kill -9 $SERVER_PID 2>/dev/null
    fi
    echo -e "${GREEN}✓ Server stopped${NC}"
  fi
}

# Set trap to cleanup on exit
trap cleanup EXIT INT TERM

# Test suite tracking
declare -A SUITE_RESULTS
TOTAL_SUITES=0
PASSED_SUITES=0
FAILED_SUITES=0

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
    ((FAILED_SUITES++))
    return 1
  fi
  
  # Make script executable
  chmod +x "$test_script"
  
  # Run test suite
  if bash "$test_script"; then
    SUITE_RESULTS["$suite_name"]="PASSED"
    ((PASSED_SUITES++))
    printf "${GREEN}✓ Suite '%s' completed successfully${NC}\n" "$suite_name"
  else
    SUITE_RESULTS["$suite_name"]="FAILED"
    ((FAILED_SUITES++))
    printf "${RED}✗ Suite '%s' failed${NC}\n" "$suite_name"
  fi
}

# ==============================================================================
# Run Test Suites
# ==============================================================================

START_TIME=$(date +%s)

# IAM Tests
run_test_suite "IAM - Authentication" "$SCRIPT_DIR/tests/iam/test-auth.sh"
run_test_suite "IAM - Users" "$SCRIPT_DIR/tests/iam/test-users.sh"
run_test_suite "IAM - Roles & Permissions" "$SCRIPT_DIR/tests/iam/test-roles-permissions.sh"
run_test_suite "IAM - Teams" "$SCRIPT_DIR/tests/iam/test-teams.sh"

# Dimentorin Tests
run_test_suite "Dimentorin - Mentors" "$SCRIPT_DIR/tests/dimentorin/test-mentors.sh"

# CMS Tests
run_test_suite "CMS - Events & Testimonials" "$SCRIPT_DIR/tests/cms/test-cms.sh"

# Gacha Tests
run_test_suite "Gacha - Items & Rolls" "$SCRIPT_DIR/tests/gacha/test-gacha.sh"

# Hackathon Tests
run_test_suite "Hackathon - Full Suite" "$SCRIPT_DIR/tests/hackathon/test-hackathon.sh"

END_TIME=$(date +%s)
DURATION=$((END_TIME - START_TIME))

# ==============================================================================
# Final Summary
# ==============================================================================

printf "\n${CYAN}════════════════════════════════════════════════════════════════${NC}\n"
printf "${BLUE}                    FINAL TEST SUMMARY                          ${NC}\n"
printf "${CYAN}════════════════════════════════════════════════════════════════${NC}\n\n"

printf "Total Test Suites: ${BLUE}%d${NC}\n" "$TOTAL_SUITES"
printf "${GREEN}Passed Suites: %d${NC}\n" "$PASSED_SUITES"
printf "${RED}Failed Suites: %d${NC}\n" "$FAILED_SUITES"
printf "\n"

if [ "$TOTAL_SUITES" -gt 0 ]; then
  SUCCESS_RATE=$(( (PASSED_SUITES * 100) / TOTAL_SUITES ))
  printf "Success Rate: ${BLUE}%d%%${NC}\n" "$SUCCESS_RATE"
fi

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
