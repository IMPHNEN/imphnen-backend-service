use axum::http::StatusCode;
use imphnen_gacha::v1::gacha_credits::gacha_credits_dto::GachaCreditRequestDto;
use imphnen_gacha::v1::gacha_rolls::gacha_rolls_dto::GachaRollRequestDto;
use imphnen_gacha::v1::gacha_claims::gacha_claims_dto::GachaClaimRequestDto;
use imphnen_gacha::v1::gacha_items::gacha_items_dto::{GachaItemRequestDto, GachaItemUpdateRequestDto};
use imphnen_cms::v1::landing::events::events_dto::{EventsCreateRequestDto, validate_url};
use imphnen_utils::validator::validate_request;
use chrono::{DateTime, Utc};
use validator::ValidationError;

#[tokio::test]
async fn test_gacha_credit_request_validation() {
    // Test valid case
    let valid_dto = GachaCreditRequestDto {
        user_id: "user-123".to_string(),
        amount: 10,
    };
    
    let result = validate_request(&valid_dto);
    assert!(result.is_ok());

    // Test empty user_id
    let invalid_dto = GachaCreditRequestDto {
        user_id: "".to_string(),
        amount: 10,
    };
    
    let result = validate_request(&invalid_dto);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().0, StatusCode::BAD_REQUEST);
    assert!(result.unwrap_err().1.contains("User ID must not be empty"));

    // Test negative amount
    let invalid_dto = GachaCreditRequestDto {
        user_id: "user-123".to_string(),
        amount: -5,
    };
    
    let result = validate_request(&invalid_dto);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().0, StatusCode::BAD_REQUEST);
    assert!(result.unwrap_err().1.contains("Amount must be at least 1 credit"));
}

#[tokio::test]
async fn test_gacha_roll_request_validation() {
    // Test valid case
    let valid_dto = GachaRollRequestDto {
        item_id: "item-123".to_string(),
        weight: 0.5,
        quantity: 5,
    };
    
    let result = validate_request(&valid_dto);
    assert!(result.is_ok());

    // Test empty item_id
    let invalid_dto = GachaRollRequestDto {
        item_id: "".to_string(),
        weight: 0.5,
        quantity: 5,
    };
    
    let result = validate_request(&invalid_dto);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().0, StatusCode::BAD_REQUEST);
    assert!(result.unwrap_err().1.contains("Item ID must not be empty"));

    // Test invalid weight range
    let invalid_dto = GachaRollRequestDto {
        item_id: "item-123".to_string(),
        weight: 1.5,
        quantity: 5,
    };
    
    let result = validate_request(&invalid_dto);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().0, StatusCode::BAD_REQUEST);
    assert!(result.unwrap_err().1.contains("Weight must be between 0.0 and 1.0"));
}

#[tokio::test]
async fn test_gacha_claim_request_validation() {
    // Test valid case
    let valid_dto = GachaClaimRequestDto {
        user_id: "user-123".to_string(),
        item_id: "item-456".to_string(),
    };
    
    let result = validate_request(&valid_dto);
    assert!(result.is_ok());

    // Test empty item_id
    let invalid_dto = GachaClaimRequestDto {
        user_id: "user-123".to_string(),
        item_id: "".to_string(),
    };
    
    let result = validate_request(&invalid_dto);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().0, StatusCode::BAD_REQUEST);
    assert!(result.unwrap_err().1.contains("Item ID must not be empty"));
}

#[tokio::test]
async fn test_gacha_item_request_validation() {
    // Test valid case
    let valid_dto = GachaItemRequestDto {
        name: "Test Item".to_string(),
        image_url: "https://example.com/image.jpg".to_string(),
    };
    
    let result = validate_request(&valid_dto);
    assert!(result.is_ok());

    // Test empty name
    let invalid_dto = GachaItemRequestDto {
        name: "".to_string(),
        image_url: "https://example.com/image.jpg".to_string(),
    };
    
    let result = validate_request(&invalid_dto);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().0, StatusCode::BAD_REQUEST);
    assert!(result.unwrap_err().1.contains("Item name must not be empty"));

    // Test invalid image URL
    let invalid_dto = GachaItemRequestDto {
        name: "Test Item".to_string(),
        image_url: "not-a-url".to_string(),
    };
    
    let result = validate_request(&invalid_dto);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().0, StatusCode::BAD_REQUEST);
    assert!(result.unwrap_err().1.contains("Image URL must be a valid URL"));
}

#[tokio::test]
async fn test_custom_url_validator() {
    // Test valid URLs
    let valid_urls = [
        "https://example.com",
        "http://example.com",
        "https://example.com/path",
        "https://example.com/path?query=value",
    ];
    
    for url in valid_urls.iter() {
        let result = validate_url(url);
        assert!(result.is_ok(), "URL should be valid: {}", url);
    }

    // Test invalid URLs
    let invalid_urls = [
        "not-a-url",
        "example.com",
        "https://",
        "http://.com",
    ];
    
    for url in invalid_urls.iter() {
        let result = validate_url(url);
        assert!(result.is_err(), "URL should be invalid: {}", url);
        assert_eq!(result.unwrap_err().code(), "invalid_url");
    }
}

#[tokio::test]
async fn test_events_create_request_validation() {
    let now = Utc::now();
    let future = now + chrono::Duration::days(1);
    
    // Test valid case
    let valid_dto = EventsCreateRequestDto {
        name: "Test Event".to_string(),
        description: "Test description".to_string(),
        detail_link: "https://example.com/event".to_string(),
        price: 99.99,
        end_date: future,
        start_date: now,
        location: Some("Test Location".to_string()),
        is_online: false,
    };
    
    let result = validate_request(&valid_dto);
    assert!(result.is_ok());

    // Test empty name
    let mut invalid_dto = valid_dto.clone();
    invalid_dto.name = "".to_string();
    
    let result = validate_request(&invalid_dto);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().0, StatusCode::BAD_REQUEST);
    assert!(result.unwrap_err().1.contains("Name must be between 1 and 100 characters"));

    // Test negative price
    let mut invalid_dto = valid_dto.clone();
    invalid_dto.price = -10.0;
    
    let result = validate_request(&invalid_dto);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().0, StatusCode::BAD_REQUEST);
    assert!(result.unwrap_err().1.contains("Price cannot be negative"));
}

#[tokio::test]
async fn test_gacha_item_update_request_validation() {
    // Test valid case with Some values
    let valid_dto = GachaItemUpdateRequestDto {
        name: Some("Updated Item".to_string()),
        image_url: Some("https://example.com/updated.jpg".to_string()),
    };
    
    let result = validate_request(&valid_dto);
    assert!(result.is_ok());

    // Test invalid image URL
    let invalid_dto = GachaItemUpdateRequestDto {
        name: Some("Updated Item".to_string()),
        image_url: Some("not-a-url".to_string()),
    };
    
    let result = validate_request(&invalid_dto);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().0, StatusCode::BAD_REQUEST);
    assert!(result.unwrap_err().1.contains("Image URL must be a valid URL"));
}

#[tokio::test]
async fn test_all_dto_types_have_validation() {
    // Test that all DTOs derive Validate trait
    let _: &dyn Validate = &GachaCreditRequestDto { user_id: "".to_string(), amount: 0 };
    let _: &dyn Validate = &GachaRollRequestDto { item_id: "".to_string(), weight: 0.0, quantity: 0 };
    let _: &dyn Validate = &GachaClaimRequestDto { user_id: "".to_string(), item_id: "".to_string() };
    let _: &dyn Validate = &GachaItemRequestDto { name: "".to_string(), image_url: "".to_string() };
    let _: &dyn Validate = &GachaItemUpdateRequestDto { name: None, image_url: None };
    let _: &dyn Validate = &EventsCreateRequestDto {
        name: "".to_string(),
        description: "".to_string(),
        detail_link: "".to_string(),
        price: 0.0,
        end_date: Utc::now(),
        start_date: Utc::now(),
        location: None,
        is_online: false,
    };
    
    // If we get here without panicking, all DTOs implement Validate
    assert!(true);
}