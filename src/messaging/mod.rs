mod slack;

use crate::options::Opts;
use anyhow::Result;
use async_trait::async_trait;
use slack::Slack;

pub enum MessageHandlerKind {
    Slack,
}

#[async_trait]
pub trait Messaging {
    /// Init the message handler (e.g: Slack, Discord...)
    ///
    ///
    /// # Arguments
    ///
    /// * `opts` - &Opts
    fn init(opts: &Opts) -> Result<Self>
    where
        Self: Sized;
    /// Send a message to the message handler
    ///
    /// # Arguments
    ///
    /// * `&self` - Messaging
    /// * `message` - &str
    async fn send(&self, message: &str) -> Result<()>;
}

/// Get a message handler for the targeted messaging tool
///
/// # Arguments
///
/// * `handler` - MessageHandler
/// * `options` - &Opts
pub fn get_message_handler(
    handler: MessageHandlerKind,
    options: &Opts,
) -> Result<Box<dyn Messaging>> {
    match handler {
        MessageHandlerKind::Slack => {
            let slack = Slack::init(options)?;
            Ok(Box::new(slack))
        }
    }
}
