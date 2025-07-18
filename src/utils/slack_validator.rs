use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::env;

pub fn validate_slack_request(signature: &str, timestamp: &str, body: &str) -> bool {
    let slack_signing_secret = match env::var("SLACK_SIGNING_SECRET") {
        Ok(secret) => secret,
        Err(_) => return false,
    };

    // Slack signing version
    let version = "v0";

    // Create base string
    let base_string = format!("{}:{}:{}", version, timestamp, body);

    // Create HMAC-SHA256 signature
    let mut mac = match Hmac::<Sha256>::new_from_slice(slack_signing_secret.as_bytes()) {
        Ok(mac) => mac,
        Err(_) => return false,
    };
    mac.update(base_string.as_bytes());

    // Compute signature
    let computed_signature = format!(
        "{}={}",
        version,
        hex::encode(mac.finalize().into_bytes())
    );

    // Compare signatures
    computed_signature == signature
}

pub fn extract_standup_message(message: &str) -> Option<(String, String, String)> {
    // Basic format validation for standup message
    // Expects: "Yesterday: X\nToday: Y\nBlockers: Z"
    let sections: Vec<&str> = message.split('\n').collect();
    
    if sections.len() != 3 {
        return None;
    }

    let yesterday = sections[0].trim_start_matches("Yesterday:").trim();
    let today = sections[1].trim_start_matches("Today:").trim();
    let blockers = sections[2].trim_start_matches("Blockers:").trim();

    Some((yesterday.to_string(), today.to_string(), blockers.to_string()))
} 