use actix_web::{test, web, App, http};
use sqlx::MySqlPool;
use serde_json::json;
use std::env;

// Helper function to get test database URL
fn get_test_db_url() -> String {
    env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://slack_attendance_user:strong_password@localhost:3306/slack_attendance_db".to_string())
}

#[tokio::test]
async fn test_complete_user_flow() {
    // Load environment variables
    dotenv::dotenv().ok();

    // Setup test database connection
    let pool = MySqlPool::connect(&get_test_db_url())
        .await
        .expect("Failed to connect to test database");

    // Create test app with all routes
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(crate::routes::register)
            .service(crate::routes::login)
            .service(crate::routes::verify_slack_user)
            .service(crate::routes::handle_slack_event)
    ).await;

    // 1. User Registration
    let register_req = test::TestRequest::post()
        .uri("/register")
        .set_json(json!({
            "name": "Integration Test User",
            "email": "integration_test@example.com",
            "password": "TestPassword123!",
            "role": "Employee"
        }))
        .to_request();

    let register_response = test::call_service(&app, register_req).await;
    assert_eq!(register_response.status(), http::StatusCode::CREATED);

    // 2. User Login
    let login_req = test::TestRequest::post()
        .uri("/login")
        .set_json(json!({
            "email": "integration_test@example.com",
            "password": "TestPassword123!"
        }))
        .to_request();

    let login_response = test::call_service(&app, login_req).await;
    assert_eq!(login_response.status(), http::StatusCode::OK);

    // Extract JWT token
    let login_body = test::read_body_json(login_response).await;
    let jwt_token = login_body.get("token").expect("JWT token not found");
    assert!(jwt_token.is_string());

    // 3. Slack User Verification
    let verify_req = test::TestRequest::post()
        .uri("/slack/verify-user")
        .set_json(json!({
            "slack_user_id": "U_INTEGRATION_TEST",
            "email": "integration_test@example.com"
        }))
        .to_request();

    let verify_response = test::call_service(&app, verify_req).await;
    assert_eq!(verify_response.status(), http::StatusCode::OK);

    // 4. Slack Event Processing
    let slack_event_req = test::TestRequest::post()
        .uri("/slack/events")
        .set_json(json!({
            "event": {
                "type": "message",
                "user": "U_INTEGRATION_TEST",
                "channel": "#daily-standup",
                "text": "Yesterday: Completed integration tests\nToday: Refining backend\nBlockers: None"
            }
        }))
        .to_request();

    let slack_event_response = test::call_service(&app, slack_event_req).await;
    assert_eq!(slack_event_response.status(), http::StatusCode::OK);
}

#[tokio::test]
async fn test_invalid_registration() {
    // Load environment variables
    dotenv::dotenv().ok();

    // Setup test database connection
    let pool = MySqlPool::connect(&get_test_db_url())
        .await
        .expect("Failed to connect to test database");

    // Create test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(crate::routes::register)
    ).await;

    // Try registering with invalid data
    let invalid_req = test::TestRequest::post()
        .uri("/register")
        .set_json(json!({
            "name": "",  // Empty name
            "email": "invalid-email",  // Invalid email
            "password": "short",  // Too short password
            "role": "InvalidRole"
        }))
        .to_request();

    let response = test::call_service(&app, invalid_req).await;
    assert_eq!(response.status(), http::StatusCode::BAD_REQUEST);
} 