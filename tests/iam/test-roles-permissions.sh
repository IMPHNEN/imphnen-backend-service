#!/bin/bash

# ==============================================================================
# IAM Tests - Roles and Permissions Endpoints
# ==============================================================================

source "$(dirname "$0")/../common/test-common.sh"

test_roles_and_permissions() {
  printf "\n${CYAN}=== Testing Roles and Permissions Endpoints ===${NC}\n"
  
  # Roles
  test_api_endpoint "GET Roles List" "GET" "/v1/roles" 200 "" true
  test_api_endpoint "GET Roles (Paginated)" "GET" "/v1/roles?page=1&limit=10" 200 "" true
  
  # Security: Test unauthorized access to roles
  local saved_token="$AUTH_TOKEN"
  AUTH_TOKEN=""
  test_api_endpoint "GET Roles without Auth (Should Fail)" "GET" "/v1/roles" 401 "" false
  AUTH_TOKEN="$saved_token"
  
  # Get role by ID - use correct endpoint /detail/{id}
  local test_role_id="5713cb37-dc02-4e87-8048-d7a41d352059"
  test_api_endpoint "GET Role By ID" "GET" "/v1/roles/detail/$test_role_id" 200 "" true
  
  # Security: Test access to non-existent role
  local fake_role_id="00000000-0000-0000-0000-000000000000"
  test_api_endpoint "GET Non-existent Role (Should Fail)" "GET" "/v1/roles/detail/$fake_role_id" 404 "" true
  
  # Create role - use correct endpoint /create
  local create_role_data=$(jq -n '{
    name: "Test Role '$(date +%s)'",
    description: "Auto-generated test role",
    permissions: []
  }')
  local create_role_response=$(test_api_endpoint "POST Create Role" "POST" "/v1/roles/create" 201 "$create_role_data" true)
  local created_role_id=$(echo "$create_role_response" | jq -r '.data.id // empty')
  
  if [ -n "$created_role_id" ]; then
    # Security: Test duplicate role creation
    test_api_endpoint "POST Create Duplicate Role (Should Fail)" "POST" "/v1/roles/create" 400 "$create_role_data" true
    
    # Update role - use correct endpoint /update/{id}
    local update_role_data=$(jq -n --arg ts "$EPOCHSECONDS" '{
      name: ("Updated Test Role " + $ts),
      description: "Updated description",
      permissions: []
    }')
    test_api_endpoint "PUT Update Role" "PUT" "/v1/roles/update/$created_role_id" 200 "$update_role_data" true
    
    # Security: Test unauthorized update
    AUTH_TOKEN=""
    test_api_endpoint "PUT Update Role without Auth (Should Fail)" "PUT" "/v1/roles/update/$created_role_id" 401 "$update_role_data" false
    AUTH_TOKEN="$saved_token"
    
    # Delete role - use correct endpoint /delete/{id}
    test_api_endpoint "DELETE Role" "DELETE" "/v1/roles/delete/$created_role_id" 200 "" true
    
    # Security: Test double delete
    test_api_endpoint "DELETE Already Deleted Role (Should Fail)" "DELETE" "/v1/roles/delete/$created_role_id" 404 "" true
  fi
  
  # Permissions
  test_api_endpoint "GET Permissions List" "GET" "/v1/permissions" 200 "" true
  test_api_endpoint "GET Permissions (Paginated)" "GET" "/v1/permissions?page=1&limit=10" 200 "" true
  
  # Security: Test unauthorized access to permissions
  AUTH_TOKEN=""
  test_api_endpoint "GET Permissions without Auth (Should Fail)" "GET" "/v1/permissions" 401 "" false
  AUTH_TOKEN="$saved_token"
  
  # Get permission by ID - use correct endpoint /detail/{id}
  local test_perm_id="023e2dfe-93c3-4008-94a8-b5dff403f73b"
  test_api_endpoint "GET Permission By ID" "GET" "/v1/permissions/detail/$test_perm_id" 200 "" true
  
  # Create permission - use correct endpoint /create
  local create_perm_data=$(jq -n '{
    name: "Test Permission '$(date +%s)'",
    description: "Auto-generated test permission"
  }')
  local create_perm_response=$(test_api_endpoint "POST Create Permission" "POST" "/v1/permissions/create" 201 "$create_perm_data" true)
  local created_perm_id=$(echo "$create_perm_response" | jq -r '.data.id // empty')
  
  if [ -n "$created_perm_id" ]; then
    # Update permission - use correct endpoint /update/{id}
    local update_perm_data=$(jq -n '{
      name: "Updated Test Permission",
      description: "Updated description"
    }')
    test_api_endpoint "PUT Update Permission" "PUT" "/v1/permissions/update/$created_perm_id" 200 "$update_perm_data" true
    
    # Delete permission - use correct endpoint /delete/{id}
    test_api_endpoint "DELETE Permission" "DELETE" "/v1/permissions/delete/$created_perm_id" 200 "" true
  fi
}

# Run if executed directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
  get_auth_token
  test_roles_and_permissions
  print_test_summary
  [ "$FAIL_COUNT" -eq 0 ] && exit 0 || exit 1
fi
