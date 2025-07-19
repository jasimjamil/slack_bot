use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use std::env;
use chrono::Utc;

use crate::db::models::AttendanceStatus;
use crate::db::queries::DatabaseQueries;
use crate::utils::slack_validator::{SlackRequestValidator, StandupMessage};

#[derive(Deserialize, Serialize, Debug)]
struct SlackEventPayload {
    event: SlackEvent,
    #[serde(rename = "type")]
    event_type: Option<String>,
    challenge: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
struct SlackEvent {
    #[serde(rename = "type")]
    event_type: String,
    user: Option<String>,
    text: Option<String>,
    channel: Option<String>,
    ts: Option<String>,
}

#[derive(Serialize)]
struct SlackResponse {
    status: String,
    message: String,
}

#[post("/slack/events")]
pub async fn handle_slack_event(
    req: actix_web::HttpRequest,
    pool: web::Data<MySqlPool>,
    payload: web::Json<SlackEventPayload>,
) -> impl Responder {
    // 🔐 Slack Challenge (for URL verification)
    if let Some(challenge) = payload.challenge.clone() {
        return HttpResponse::Ok().body(challenge);
    }

    // 🔐 Slack Signature Validation
    let signature = req
        .headers()
        .get("X-Slack-Signature")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    let timestamp = req
        .headers()
        .get("X-Slack-Request-Timestamp")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
        let body = serde_json::to_string(&*payload).unwrap_or_default();


    match SlackRequestValidator::validate_request(signature, timestamp, &body) {
        Ok(true) => {}
        _ => {
            return HttpResponse::Unauthorized().json(SlackResponse {
                status: "error".to_string(),
                message: "Invalid Slack signature".to_string(),
            });
        }
    }

    // 📨 Only process message events
    if payload.event.event_type != "message" {
        return HttpResponse::Ok().json(SlackResponse {
            status: "skipped".to_string(),
            message: "Not a message event".to_string(),
        });
    }

    let SlackEvent {
        user,
        text,
        channel,
        ..
    } = &payload.event;
    

    let (user_id, message, channel_id) = match (user, text, channel) {
        (Some(u), Some(t), Some(c)) => (u, t, c),
        _ => {
            return HttpResponse::BadRequest().json(SlackResponse {
                status: "error".to_string(),
                message: "Incomplete event data".to_string(),
            });
        }
    };

    // ✅ Check if channel matches
    let expected_channel = env::var("SLACK_CHANNEL_ID").unwrap_or_else(|_| "standup-channel-id".to_string());
    if channel_id != &expected_channel {
        return HttpResponse::Ok().json(SlackResponse {
            status: "skipped".to_string(),
            message: "Not a standup channel".to_string(),
        });
    }

    // ✅ Parse standup message format
    let standup_msg = match SlackRequestValidator::extract_standup_message(message) {
        Some(parsed) => parsed,
        None => {
            return HttpResponse::BadRequest().json(SlackResponse {
                status: "error".to_string(),
                message: "Invalid standup format".to_string(),
            });
        }
    };

    // ✅ Store in DB
    match log_slack_attendance(&pool, user_id, &standup_msg).await {
        Ok(_) => HttpResponse::Ok().json(SlackResponse {
            status: "success".to_string(),
            message: "Attendance recorded".to_string(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(SlackResponse {
            status: "error".to_string(),
            message: format!("Failed to log attendance: {}", e),
        }),
    }
}

async fn log_slack_attendance(
    pool: &MySqlPool,
    slack_user_id: &str,
    standup: &StandupMessage,
) -> Result<(), String> {
    let user = DatabaseQueries::find_user_by_slack_id(pool, slack_user_id)
        .await
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or("User not found")?;

    let user_id = user.id.try_into().map_err(|_| "User ID conversion failed")?;

    let summary = format!(
        "Yesterday: {}\nToday: {}\nBlockers: {}",
        standup.yesterday, standup.today, standup.blockers
    );

    DatabaseQueries::log_attendance(
        pool,
        user_id,
        Utc::now().date_naive(),
        AttendanceStatus::Present,
        Some(summary),
    )
    .await
    .map_err(|e| format!("Log failed: {}", e))?;

    Ok(())
}
