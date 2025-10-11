#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};
    use chrono::Days;
    use imphnen_hackathon::v1::hackathon::hackathon_dto::HackathonCreateRequestDto;
    use imphnen_hackathon::v1::hackathon::hackathon_repository::HackathonRepository;
    use std::sync::Arc;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_basic_hackathon_operations() {
        // This is a minimal test to verify basic functionality
        // In a real scenario, you would need proper test setup with a test database
        
        println!("Testing basic hackathon operations...");
        
        // Test data
        let user_id = Uuid::new_v4().to_string();
        
        // Create a hackathon request with only required fields
        let hackathon_request = HackathonCreateRequestDto {
            name: "Test Hackathon".to_string(),
            registration_deadline: Utc::now()
                .checked_add_days(Days::new(7))
                .unwrap()
                .to_rfc3339(),
            max_participants: 100,
            theme: "Backend Development".to_string(),
            previous_winners: None,
            organizers: vec![user_id.clone()],
        };
        
        println!("Created hackathon request: {:?}", hackathon_request);
        
        // Create a mock repository for testing (simplified)
        let mock_repo = HackathonRepository::new(&Arc::new(()));
        
        // In a real test, you would call the actual service methods:
        // This is just a compilation test - we don't actually execute the DB operations
        // let create_result = mock_repo.create_hackathon(hackathon_request, user_id.clone()).await;
        // assert!(create_result.is_ok());
        
        println!("Basic hackathon test completed successfully!");
    }
}