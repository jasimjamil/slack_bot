use sqlx::MySqlPool;
use slack_attendance_backend::{
    services::slack_service::SlackService,
    services::attendance_service::AttendanceService,
    db::models::{User, UserRole, AttendanceStatus},
    db::queries::DatabaseQueries,
};
use chrono::Utc;

#[tokio::test]
async fn test_slack_user_verification() {
    // Setup test database connection
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let pool = MySqlPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // Create test user
    let test_email = format!("test_user_{}@example.com", Utc::now().timestamp());
    let user_id = DatabaseQueries::create_user(
        &pool, 
        "Test User", 
        &test_email, 
        "hashed_password", 
        UserRole::Employee,
        None
    )
    .await
    .expect("Failed to create test user");

    // Test Slack user verification
    let slack_user_id = "U_TEST_123";
    let verification_result = SlackService::verify_slack_user(
        &pool, 
        slack_user_id, 
        &test_email
    )
    .await
    .expect("Verification failed");

    // Assertions
    assert!(verification_result, "User should be verified");
}

#[tokio::test]
async fn test_attendance_logging() {
    // Setup test database connection
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let pool = MySqlPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // Create test user
    let test_email = format!("attendance_user_{}@example.com", Utc::now().timestamp());
    let user_id = DatabaseQueries::create_user(
        &pool, 
        "Attendance User", 
        &test_email, 
        "hashed_password", 
        UserRole::Employee,
        None
    )
    .await
    .expect("Failed to create test user");

    // Log attendance
    let today = chrono::Local::now().date_naive();
    let attendance_result = AttendanceService::log_attendance(
        &pool, 
        user_id, 
        today, 
        AttendanceStatus::Present, 
        Some("Daily standup completed".to_string())
    )
    .await;

    // Assertions
    assert!(attendance_result.is_ok(), "Attendance should be logged successfully");
}
