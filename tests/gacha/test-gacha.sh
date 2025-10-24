#!/bin/bash

# Get directory of this script
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../common/test-common.sh"

test_gacha_endpoints() {
  echo ""
  echo "=== Testing Gacha Endpoints ==="
  
  # Gacha Items - use correct endpoints /create, /detail/{id}, /update/{id}, /delete/{id}
  test_api_endpoint "GET Gacha Items" "GET" "/v1/gacha/items?page=1&per_page=10" 200 "" true
  test_api_endpoint "GET Gacha Items (Paginated)" "GET" "/v1/gacha/items?page=1&per_page=5" 200 "" true

  # Get first gacha item ID to test detail endpoint
  local items_response=$(curl -s -H "Authorization: Bearer $AUTH_TOKEN" "$BASE_URL/v1/gacha/items?page=1&per_page=1")
  local test_item_id=$(echo "$items_response" | jq -r '.data[0].id // empty')
  
  if [ -n "$test_item_id" ]; then
    # Get item detail - use correct endpoint /detail/{id}
    test_api_endpoint "GET Gacha Item By ID" "GET" "/v1/gacha/items/detail/$test_item_id" 200 "" true
  fi

  # Create gacha item - use correct endpoint /create
  local create_item_data=$(jq -n '{
    name: "Test Item '$EPOCHSECONDS'",
    description: "Test gacha item",
    image_url: "https://example.com/gacha-item.png",
    rarity: "COMMON",
    weight: 100
  }')
  test_api_endpoint "POST Create Gacha Item" "POST" "/v1/gacha/items/create" 201 "$create_item_data" true
  
  # Get created item ID from response
  local create_response=$(curl -s -X POST -H "Authorization: Bearer $AUTH_TOKEN" -H "Content-Type: application/json" -d "$create_item_data" "$BASE_URL/v1/gacha/items/create")
  local created_item_id=$(echo "$create_response" | jq -r '.data.id // empty')
  
  if [ -n "$created_item_id" ]; then
    # Update gacha item - use correct endpoint /update/{id}
    local update_item_data=$(jq -n '{
      name: "Updated Test Item",
      description: "Updated description",
      image_url: "https://example.com/updated-gacha-item.png",
      rarity: "RARE",
      weight: 50
    }')
    test_api_endpoint "PUT Update Gacha Item" "PUT" "/v1/gacha/items/update/$created_item_id" 200 "$update_item_data" true

    # Delete gacha item - use correct endpoint /delete/{id}
    test_api_endpoint "DELETE Gacha Item" "DELETE" "/v1/gacha/items/delete/$created_item_id" 200 "" true
  fi

  # Gacha Rolls - need to get an existing item first
  local items_response=$(curl -s -H "Authorization: Bearer $AUTH_TOKEN" "$BASE_URL/v1/gacha/items?page=1&per_page=1")
  local test_item_id=$(echo "$items_response" | jq -r '.data[0].id // empty')
  
  if [ -n "$test_item_id" ]; then
    # Create gacha roll with item_id - use correct endpoint /create
    local create_roll_data=$(jq -n --arg item_id "$test_item_id" '{item_id: $item_id, weight: 1.0, quantity: 1}')
    test_api_endpoint "POST Create Gacha Roll" "POST" "/v1/gacha/rolls/create" 201 "$create_roll_data" true
    
    # Get roll ID to execute it
    local create_roll_response=$(curl -s -X POST -H "Authorization: Bearer $AUTH_TOKEN" -H "Content-Type: application/json" -d "$create_roll_data" "$BASE_URL/v1/gacha/rolls/create")
    local roll_id=$(echo "$create_roll_response" | jq -r '.data.id // empty')
    
    if [ -n "$roll_id" ]; then
      test_api_endpoint "POST Execute Gacha Roll" "POST" "/v1/gacha/rolls/execute" 200 "{\"roll_id\": \"$roll_id\"}" true
    fi
  fi

  # Gacha Credits
  # Note: These endpoints may require special permissions or internal access
  # test_api_endpoint "GET User Credits" "GET" "/v1/gacha/credits" 200 "" true
  # local add_credits_data=$(jq -n '{amount: 100}')
  # test_api_endpoint "POST Add Credits" "POST" "/v1/gacha/credits/add" 200 "$add_credits_data" true
  # local consume_credits_data=$(jq -n '{amount: 1}')
  # test_api_endpoint "POST Consume Credits" "POST" "/v1/gacha/credits/consume" 200 "$consume_credits_data" true

  # Gacha Claims
  # test_api_endpoint "POST Create Gacha Claim" "POST" "/v1/gacha/claims" 201 "{}" true
}

# Run if executed directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
  get_auth_token
  test_gacha_endpoints
  print_test_summary
  [ "$FAIL_COUNT" -eq 0 ] && exit 0 || exit 1
fi
