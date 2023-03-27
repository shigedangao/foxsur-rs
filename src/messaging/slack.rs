use super::Messaging;
use crate::options::Opts;
use anyhow::Result;
use async_trait::async_trait;
use slack_rust::chat::post_message::post_message;
use slack_rust::{
    chat::post_message::PostMessageRequest, http_client::default_client, http_client::Client,
};

#[derive(Debug)]
pub struct Slack {
    bot_token: String,
    channel_id: String,
    client: Client,
}

#[async_trait]
impl Messaging for Slack {
    fn init(options: &Opts) -> Result<Self>
    where
        Self: Sized,
    {
        let client = default_client();
        match &options.slack {
            Some(s) => Ok(Slack {
                bot_token: s.bot_token.clone(),
                channel_id: s.channel_id.clone(),
                client,
            }),
            None => Err(anyhow::format_err!("Slack options not found")),
        }
    }

    async fn send(&self, message: &str) -> Result<()> {
        let payload = PostMessageRequest {
            channel: self.channel_id.to_string(),
            text: Some(message.to_string()),
            ..Default::default()
        };

        let _ = post_message(&self.client, &payload, &self.bot_token)
            .await
            .map_err(|e| anyhow::format_err!("Slack error: {}", e))?;

        Ok(())
    }
}
