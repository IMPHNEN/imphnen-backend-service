#!/bin/bash

# Test script for mentors PostgreSQL integration with SeaORM
# This script tests the full CRUD lifecycle of mentors in PostgreSQL

set -e

echo "=== Starting Mentors PostgreSQL Integration Test ==="

# Configuration
DB_URL=${DATABASE_URL:-"postgres://root:root@localhost:5432/localdb"}
TEST_EMAIL="test_mentor@example.com"
TEST_USERNAME="test_mentor_user"
TEST_FIRST_NAME="Test"
TEST_LAST_NAME="Mentor"

# Import necessary utilities
source "$(dirname "$0")/../test_utils.sh"

# Test 1: Database Connection
echo -e "\n1. Testing PostgreSQL connection..."
check_db_connection "$DB_URL"

# Test 2: Create test user
echo -e "\n2. Creating test user..."
USER_ID=$(create_test_user "$DB_URL" "$TEST_EMAIL" "$TEST_USERNAME" "$TEST_FIRST_NAME" "$TEST_LAST_NAME")
echo "Created user with ID: $USER_ID"

# Test 3: Create test mentor
echo -e "\n3. Creating test mentor..."
MENTOR_ID=$(create_test_mentor "$DB_URL" "$USER_ID")
echo "Created mentor with ID: $MENTOR_ID"

# Test 4: Retrieve mentor by ID
echo -e "\n4. Retrieving mentor by ID..."
retrieve_mentor_by_id "$DB_URL" "$MENTOR_ID"

# Test 5: Retrieve mentor by email
echo -e "\n5. Retrieving mentor by email..."
retrieve_mentor_by_email "$DB_URL" "$TEST_EMAIL"

# Test 6: Update mentor
echo -e "\n6. Updating mentor..."
update_mentor "$DB_URL" "$MENTOR_ID"

# Test 7: Retrieve updated mentor
echo -e "\n7. Retrieving updated mentor..."
retrieve_mentor_by_id "$DB_URL" "$MENTOR_ID"

# Test 8: Soft delete mentor
echo -e "\n8. Soft deleting mentor..."
soft_delete_mentor "$DB_URL" "$MENTOR_ID"

# Test 9: Verify mentor is soft deleted
echo -e "\n9. Verifying mentor is soft deleted..."
verify_soft_delete "$DB_URL" "$MENTOR_ID"

# Test 10: Clean up
echo -e "\n10. Cleaning up test data..."
cleanup_test_data "$DB_URL" "$USER_ID" "$MENTOR_ID"

echo -e "\n=== All tests passed successfully! ==="