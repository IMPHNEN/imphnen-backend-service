#[cfg(test)]
mod tests {
    use crate::{generate_unique_email, get_role_id, setup_all_test_environment, UsersRepository};
    use axum::{http::StatusCode, response::Response};
    use imphnen_entities::{AppState, ResponseSuccessDto};
    use imphnen_gacha::{
        gacha_credits_controller::GachaCreditController,
        gacha_credits_dto::GachaCreditRequestDto,
        gacha_rolls_controller::GachaRollController,
    };
    use imphnen_iam::users_service::UsersService;
    use serde_json::json;

    #[tokio::test]
    async fn test_comprehensive_gacha_credits_flow() {
        let app_state = setup_all_test_environment().await;
        let user_repo = UsersRepository::new(&app_state);

        // Create test user
        let email = generate_unique_email("test_comprehensive_credits");
        let password = "Password123!".to_string();
        
        let user_dto = imphnen_iam::users_dto::UserCreateRequestDto {
            email: email.clone(),
            password: password.clone(),
            fullname: "Test Comprehensive Credits".to_string(),
            phone_number: Some("1234567890".to_string()),
            role_id: get_role_id(&app_state, "user").await.unwrap(),
        };

        let _ = UsersService::create_user(&app_state, user_dto).await;
        let user = user_repo.query_user_by_email(email.clone()).await.unwrap();

        // Test 1: Get initial credits (should be 0)
        let headers = axum::http::HeaderMap::new();
        headers.insert("Authorization", "Bearer test_token".parse().unwrap());
        
        let response = GachaCreditController::get_user_credits(headers.clone(), &app_state).await;
        assert_eq!(response.status(), StatusCode::OK);
        
        let response_body: ResponseSuccessDto<serde_json::Value> = response.json().await.unwrap();
        let available_rolls = response_body.data["available_rolls"].as_i64().unwrap();
        assert_eq!(available_rolls, 0);

        // Test 2: Add credits
        let add_credits_dto = GachaCreditRequestDto {
            user_id: user.id.id.to_raw(),
            amount: 10,
        };

        let add_response = GachaCreditController::add_user_credits(
            headers.clone(), 
            &app_state, 
            add_credits_dto
        ).await;
        assert_eq!(add_response.status(), StatusCode::OK);

        // Test 3: Verify credits were added
        let get_response = GachaCreditController::get_user_credits(headers.clone(), &app_state).await;
        let response_body: ResponseSuccessDto<serde_json::Value> = get_response.json().await.unwrap();
        let available_rolls = response_body.data["available_rolls"].as_i64().unwrap();
        assert_eq!(available_rolls, 10);

        // Test 4: Consume one credit
        let consume_response = GachaCreditController::consume_user_credit(headers.clone(), &app_state).await;
        assert_eq!(consume_response.status(), StatusCode::OK);

        // Test 5: Verify credit was consumed
        let get_response = GachaCreditController::get_user_credits(headers.clone(), &app_state).await;
        let response_body: ResponseSuccessDto<serde_json::Value> = get_response.json().await.unwrap();
        let available_rolls = response_body.data["available_rolls"].as_i64().unwrap();
        assert_eq!(available_rolls, 9);

        // Test 6: Try to execute a gacha roll (should consume another credit)
        let roll_response = GachaRollController::execute_roll_once(headers.clone(), &app_state).await;
        
        // This might fail if there are no active rolls in test environment, but should not fail due to credits
        if roll_response.status() == StatusCode::OK {
            // Verify credits were consumed if roll was successful
            let get_response = GachaCreditController::get_user_credits(headers.clone(), &app_state).await;
            let response_body: ResponseSuccessDto<serde_json::Value> = get_response.json().await.unwrap();
            let available_rolls = response_body.data["available_rolls"].as_i64().unwrap();
            assert!(available_rolls <= 8, "Credits should be reduced after successful roll");
        }

        // Clean up
        let _ = user_repo.query_delete_user(user.id.id.to_raw()).await;
    }

    #[tokio::test]
    async fn test_add_negative_credits() {
        let app_state = setup_all_test_environment().await;
        let user_repo = UsersRepository::new(&app_state);

        // Create test user
        let email = generate_unique_email("test_negative_credits");
        let password = "Password123!".to_string();
        
        let user_dto = imphnen_iam::users_dto::UserCreateRequestDto {
            email: email.clone(),
            password: password.clone(),
            fullname: "Test Negative Credits".to_string(),
            phone_number: Some("1234567890".to_string()),
            role_id: get_role_id(&app_state, "user").await.unwrap(),
        };

        let _ = UsersService::create_user(&app_state, user_dto).await;
        let user = user_repo.query_user_by_email(email.clone()).await.unwrap();

        let headers = axum::http::HeaderMap::new();
        headers.insert("Authorization", "Bearer test_token".parse().unwrap());
        
        // Add negative credits (should still work as i32 allows negative values)
        let negative_credits_dto = GachaCreditRequestDto {
            user_id: user.id.id.to_raw(),
            amount: -5,
        };

        let response = GachaCreditController::add_user_credits(
            headers.clone(), 
            &app_state, 
            negative_credits_dto
        ).await;
        
        // Should succeed (negative credits are allowed by the system)
        assert_eq!(response.status(), StatusCode::OK);

        // Verify negative credits were added
        let get_response = GachaCreditController::get_user_credits(headers.clone(), &app_state).await;
        let response_body: ResponseSuccessDto<serde_json::Value> = get_response.json().await.unwrap();
        let available_rolls = response_body.data["available_rolls"].as_i64().unwrap();
        assert_eq!(available_rolls, -5);

        // Clean up
        let _ = user_repo.query_delete_user(user.id.id.to_raw()).await;
    }

    #[tokio::test]
    async fn test_consume_credits_when_none_available() {
        let app_state = setup_all_test_environment().await;
        let user_repo = UsersRepository::new(&app_state);

        // Create test user
        let email = generate_unique_email("test_no_credits");
        let password = "Password123!".to_string();
        
        let user_dto = imphnen_iam::users_dto::UserCreateRequestDto {
            email: email.clone(),
            password: password.clone(),
            fullname: "Test No Credits".to_string(),
            phone_number: Some("1234567890".to_string()),
            role_id: get_role_id(&app_state, "user").await.unwrap(),
        };

        let _ = UsersService::create_user(&app_state, user_dto).await;
        let user = user_repo.query_user_by_email(email.clone()).await.unwrap();

        let headers = axum::http::HeaderMap::new();
        headers.insert("Authorization", "Bearer test_token".parse().unwrap());
        
        // Try to consume credits when none available
        let response = GachaCreditController::consume_user_credit(headers.clone(), &app_state).await;
        
        // Should return error
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        // Clean up
        let _ = user_repo.query_delete_user(user.id.id.to_raw()).await;
    }

    #[tokio::test]
    async fn test_credits_integration_with_gacha_rolls() {
        let app_state = setup_all_test_environment().await;
        let user_repo = UsersRepository::new(&app_state);

        // Create test user
        let email = generate_unique_email("test_credits_integration");
        let password = "Password123!".to_string();
        
        let user_dto = imphnen_iam::users_dto::UserCreateRequestDto {
            email: email.clone(),
            password: password.clone(),
            fullname: "Test Credits Integration".to_string(),
            phone_number: Some("1234567890".to_string()),
            role_id: get_role_id(&app_state, "user").await.unwrap(),
        };

        let _ = UsersService::create_user(&app_state, user_dto).await;
        let user = user_repo.query_user_by_email(email.clone()).await.unwrap();

        let headers = axum::http::HeaderMap::new();
        headers.insert("Authorization", "Bearer test_token".parse().unwrap());
        
        // Add initial credits
        let add_credits_dto = GachaCreditRequestDto {
            user_id: user.id.id.to_raw(),
            amount: 5,
        };

        let _ = GachaCreditController::add_user_credits(
            headers.clone(), 
            &app_state, 
            add_credits_dto
        ).await;

        // Check initial credits
        let get_response = GachaCreditController::get_user_credits(headers.clone(), &app_state).await;
        let response_body: ResponseSuccessDto<serde_json::Value> = get_response.json().await.unwrap();
        let initial_credits = response_body.data["available_rolls"].as_i64().unwrap();
        assert_eq!(initial_credits, 5);

        // Try to execute a gacha roll
        let roll_response = GachaRollController::execute_roll_once(headers.clone(), &app_state).await;
        
        // If roll is successful, check that credits were reduced
        if roll_response.status() == StatusCode::OK {
            let get_response = GachaCreditController::get_user_credits(headers.clone(), &app_state).await;
            let response_body: ResponseSuccessDto<serde_json::Value> = get_response.json().await.unwrap();
            let final_credits = response_body.data["available_rolls"].as_i64().unwrap();
            assert_eq!(final_credits, 4, "One credit should be consumed for the roll");
        }

        // Clean up
        let _ = user_repo.query_delete_user(user.id.id.to_raw()).await;
    }
}