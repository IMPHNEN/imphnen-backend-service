#!/bin/bash

# ==============================================================================
# IMPHNEN API - Master Test Runner
# Menjalankan semua test suite untuk coverage lengkap
# ==============================================================================

# Colors
CYAN='\033[0;36m'
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[0;33m'
NC='\033[0m'

echo -e "${CYAN}================================================================${NC}"
echo -e "${CYAN}         IMPHNEN Backend - Complete API Test Suite${NC}"
echo -e "${CYAN}================================================================${NC}"
echo ""

# Check if server is running
BASE_URL="http://127.0.0.1:4099"
if curl -s --head "$BASE_URL/v1/cms/landing/events" > /dev/null 2>&1; then
    echo -e "${GREEN}✓ Server is running at $BASE_URL${NC}"
else
    echo -e "${RED}✗ Server is not running!${NC}"
    echo -e "${YELLOW}Please start the server first with: cargo run --bin api --release${NC}"
    exit 1
fi

echo ""

# Counters
TOTAL_SUITES=0
PASSED_SUITES=0
FAILED_SUITES=0

run_test_suite() {
    local suite_name=$1
    local suite_command=$2
    local suite_description=$3
    
    ((TOTAL_SUITES++))
    
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${CYAN}Test Suite #${TOTAL_SUITES}: ${suite_name}${NC}"
    echo -e "${CYAN}Description: ${suite_description}${NC}"
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
    
    # Run the test suite
    if eval "$suite_command"; then
        echo ""
        echo -e "${GREEN}✓ Suite Passed: ${suite_name}${NC}"
        ((PASSED_SUITES++))
    else
        echo ""
        echo -e "${RED}✗ Suite Failed: ${suite_name}${NC}"
        ((FAILED_SUITES++))
    fi
    
    echo ""
}

# ==============================================================================
# RUN ALL TEST SUITES
# ==============================================================================

echo -e "${YELLOW}Starting comprehensive API testing...${NC}"
echo ""
sleep 1

# Suite 1: Main Test Suite (dari test.sh)
run_test_suite \
    "Main API Test Suite" \
    "bash test.sh -dk" \
    "Comprehensive tests covering all major endpoints with multiple user roles"

# Suite 2: Comprehensive API Coverage
run_test_suite \
    "Extended API Coverage" \
    "bash test-comprehensive-api.sh" \
    "Detailed tests for all CRUD operations on every endpoint"

# Suite 3: Cargo Unit Tests
run_test_suite \
    "Rust Unit Tests" \
    "cargo test --lib 2>&1 | grep -E '(test result|running)'" \
    "Unit tests for Rust codebase"

# Suite 4: Cargo Integration Tests
run_test_suite \
    "Rust Integration Tests" \
    "cargo test --test '*' 2>&1 | grep -E '(test result|running)'" \
    "Integration tests for full system behavior"

# ==============================================================================
# FINAL SUMMARY
# ==============================================================================

echo ""
echo -e "${CYAN}================================================================${NC}"
echo -e "${CYAN}                     FINAL TEST SUMMARY${NC}"
echo -e "${CYAN}================================================================${NC}"
echo ""
echo -e "Total Test Suites Run: ${TOTAL_SUITES}"
echo -e "${GREEN}Passed Suites: ${PASSED_SUITES}${NC}"
echo -e "${RED}Failed Suites: ${FAILED_SUITES}${NC}"
echo ""

SUCCESS_RATE=0
if [ "$TOTAL_SUITES" -gt 0 ]; then
    SUCCESS_RATE=$(( (PASSED_SUITES * 100) / TOTAL_SUITES ))
fi

echo -e "Overall Success Rate: ${SUCCESS_RATE}%"
echo ""

if [ "$FAILED_SUITES" -eq 0 ]; then
    echo -e "${GREEN}========================================${NC}"
    echo -e "${GREEN}  ✓ ALL TEST SUITES PASSED!${NC}"
    echo -e "${GREEN}========================================${NC}"
    exit 0
else
    echo -e "${RED}========================================${NC}"
    echo -e "${RED}  ✗ SOME TEST SUITES FAILED${NC}"
    echo -e "${RED}========================================${NC}"
    echo ""
    echo -e "${YELLOW}Please review the output above to identify failed tests.${NC}"
    exit 1
fi
