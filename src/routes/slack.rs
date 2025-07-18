use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use crate::db::queries::DatabaseQueries;
use crate::utils::slack_validator;

#[derive(Deserialize)]
pub struct SlackUserVerificationRequest {
    slack_user_id: String,
    email: String,
}

#[derive(Serialize)]
pub struct SlackUserVerificationResponse {
    is_verified: bool,
    user_id: Option<u64>,
    name: Option<String>,
}

#[post("/slack/verify-user")]
pub async fn verify_slack_user(
    pool: web::Data<MySqlPool>,
    verification_data: web::Json<SlackUserVerificationRequest>
) -> impl Responder {
    // Find user by email
    match DatabaseQueries::find_user_by_email(&pool, &verification_data.email).await {
        Ok(Some(user)) => {
            // In a real-world scenario, you might want to store Slack User ID in the database
            HttpResponse::Ok().json(SlackUserVerificationResponse {
                is_verified: true,
                user_id: Some(user.id),
                name: Some(user.name),
            })
        },
        Ok(None) => HttpResponse::Ok().json(SlackUserVerificationResponse {
            is_verified: false,
            user_id: None,
            name: None,
        }),
        Err(_) => HttpResponse::InternalServerError().body("Database error")
    }
}

#[post("/slack/events")]
pub async fn handle_slack_event(
    pool: web::Data<MySqlPool>,
    payload: web::Json<serde_json::Value>
) -> impl Responder {
    // Extract message details
    let event = match payload.get("event") {
        Some(event) => event,
        None => return HttpResponse::BadRequest().body("Invalid Slack event")
    };

    // Validate message is in daily standup channel
    let channel = event.get("channel")
        .and_then(|c| c.as_str())
        .unwrap_or("");
    
    if channel != "#daily-standup" {
        return HttpResponse::Ok().body("Not a daily standup message");
    }

    // Extract message text and user
    let message_text = event.get("text")
        .and_then(|t| t.as_str())
        .unwrap_or("");
    
    let slack_user_id = event.get("user")
        .and_then(|u| u.as_str())
        .unwrap_or("");

    // Parse standup message
    match slack_validator::extract_standup_message(message_text) {
        Some((yesterday, today, blockers)) => {
            // Find user by Slack ID (you'd need to extend your user model)
            // For now, we'll just log the message
            println!("Standup Message from {}: ", slack_user_id);
            println!("Yesterday: {}", yesterday);
            println!("Today: {}", today);
            println!("Blockers: {}", blockers);

            // TODO: Log attendance for the user
            HttpResponse::Ok().body("Standup message processed")
        },
        None => HttpResponse::BadRequest().body("Invalid standup message format")
    }
} 