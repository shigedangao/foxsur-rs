use super::Messaging;
use crate::cli::options::CliArgs;
use anyhow::Result;

#[derive(Debug)]
pub struct Slack {
    bot_token: String,
    channel_id: String,
}

impl Messaging for Slack {
    fn init(options: &CliArgs) -> Result<Self>
    where
        Self: Sized,
    {
        match &options.slack {
            Some(s) => Ok(Slack {
                bot_token: s.bot_token.clone(),
                channel_id: s.channel_id.clone(),
            }),
            None => Err(anyhow::format_err!("Slack options not found")),
        }
    }

    fn send(&self, message: &str) -> Result<()> {
        let response = reqwest::blocking::Client::new()
            .post("https://slack.com/api/chat.postMessage")
            .header("Content-type", "application/json")
            .header("Authorization", format!("Bearer {}", self.bot_token))
            .json(&serde_json::json!({
                "channel": self.channel_id,
                "text": message,
            }))
            .send()?;

        if let Err(err) = response.error_for_status() {
            return Err(anyhow::format_err!("Slack API returned an error: {}", err));
        }

        Ok(())
    }
}
