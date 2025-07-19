// src/lib.rs

pub mod db;
pub mod routes;
pub mod services;
pub mod utils;

pub use db::models;
pub use db::queries::DatabaseQueries;

// ✅ Only include services if they exist
// Comment these out if those modules are missing
// pub use services::attendance_service::AttendanceService;
// pub use services::slack_service::SlackService;
// pub use services::report_service::ReportService;

pub type AppResult<T> = Result<T, SlackAttendanceError>;

#[derive(Debug, thiserror::Error)]
pub enum SlackAttendanceError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Slack API error: {0}")]
    SlackApiError(String),

    #[error("Authentication error: {0}")]
    AuthenticationError(String),

    #[error("Internal server error")]
    InternalError,
}
