use sqlx::MySqlPool;
use reqwest::Client;
use std::env;
use crate::db::queries::DatabaseQueries;


pub struct SlackService;

impl SlackService {
    pub async fn verify_slack_user(
        pool: &MySqlPool,
        slack_user_id: &str,
        email: &str
    ) -> Result<bool, sqlx::Error> {
        // Check if user exists and update Slack User ID
        let user = DatabaseQueries::find_user_by_email(pool, email).await?;
        
        match user {
            Some( user) => {
                // Update Slack User ID if not already set
                if user.slack_user_id.is_none() {
                    sqlx::query!(
                        "UPDATE users SET slack_user_id = ? WHERE id = ?",
                        slack_user_id,
                        user.id
                    )
                    .execute(pool)
                    .await?;
                }
                Ok(true)
            },
            None => Ok(false)
        }
    }

    pub async fn fetch_slack_user_info(
        slack_user_id: &str
    ) -> Result<(String, Option<String>), reqwest::Error> {
        let slack_token = env::var("SLACK_BOT_TOKEN")
            .expect("SLACK_BOT_TOKEN not set");
        
        let client = Client::new();
        let response = client
            .get("https://slack.com/api/users.info")
            .bearer_auth(&slack_token)
            .query(&[("user", slack_user_id)])
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let real_name = response["user"]["profile"]["real_name"]
            .as_str()
            .unwrap_or("Unknown")
            .to_string();
        
        let email = response["user"]["profile"]["email"]
            .as_str()
            .map(|s| s.to_string());

        Ok((real_name, email))
    }
}
