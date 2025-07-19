use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

const SLACK_SIGNATURE_VERSION: &str = "v0";
const SLACK_REQUEST_TOLERANCE_SECONDS: u64 = 5 * 60; // 5 minutes

#[derive(Debug, Clone)]
pub struct StandupMessage {
    pub yesterday: String,
    pub today: String,
    pub blockers: String,
}

pub struct SlackRequestValidator;

impl SlackRequestValidator {
    /// Validate Slack webhook request
    pub fn validate_request(
        signature: &str,
        timestamp: &str,
        request_body: &str,
    ) -> Result<bool, String> {
        let slack_signing_secret = env::var("SLACK_SIGNING_SECRET")
            .map_err(|_| "Slack signing secret not set".to_string())?;

        let timestamp_num: u64 = timestamp.parse().map_err(|_| "Invalid timestamp".to_string())?;

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| "System time error".to_string())?
            .as_secs();

        if current_time > timestamp_num + SLACK_REQUEST_TOLERANCE_SECONDS {
            return Err("Request too old".to_string());
        }

        let base_string = format!("{}:{}:{}", SLACK_SIGNATURE_VERSION, timestamp, request_body);

        let mut mac = Hmac::<Sha256>::new_from_slice(slack_signing_secret.as_bytes())
            .map_err(|_| "HMAC initialization failed".to_string())?;
        mac.update(base_string.as_bytes());

        let computed_signature = format!(
            "{}={}",
            SLACK_SIGNATURE_VERSION,
            hex::encode(mac.finalize().into_bytes())
        );

        Ok(computed_signature == signature)
    }

    /// Extract standup message components from plain text
    pub fn extract_standup_message(message: &str) -> Option<StandupMessage> {
        let lines: Vec<&str> = message.split('\n').collect();

        if lines.len() < 3 {
            return None;
        }

        Some(StandupMessage {
            yesterday: lines[0].trim_start_matches("Yesterday:").trim().to_string(),
            today: lines[1].trim_start_matches("Today:").trim().to_string(),
            blockers: lines[2].trim_start_matches("Blockers:").trim().to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standup_message_extraction() {
        let message = "Yesterday: Fixed bugs\nToday: Writing unit tests\nBlockers: None";
        let parsed = SlackRequestValidator::extract_standup_message(message);

        assert!(parsed.is_some());
        let msg = parsed.unwrap();
        assert_eq!(msg.yesterday, "Fixed bugs");
        assert_eq!(msg.today, "Writing unit tests");
        assert_eq!(msg.blockers, "None");
    }
}
