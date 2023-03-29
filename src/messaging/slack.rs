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
        Ok(())
    }
}
