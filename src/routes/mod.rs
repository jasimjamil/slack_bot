pub mod auth;
pub mod slack;
pub mod attendance;
pub mod report;

// Re-export routes to be used in main.rs
pub use auth::{login, register};
pub use slack::{handle_slack_event, verify_slack_user}; 