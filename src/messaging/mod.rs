mod slack;

use crate::cli::options::CliArgs;
use anyhow::Result;
use slack::Slack;

pub enum MessageHandlerKind {
    Slack,
}

pub trait Messaging {
    /// Init the message handler (e.g: Slack, Discord...)
    ///
    ///
    /// # Arguments
    ///
    /// * `opts` - &Opts
    fn init(opts: &CliArgs) -> Result<Self>
    where
        Self: Sized;
    /// Send a message to the message handler
    ///
    /// # Arguments
    ///
    /// * `&self` - Messaging
    /// * `message` - &str
    fn send(&self, message: &str) -> Result<()>;
}

/// Get a message handler for the targeted messaging tool
///
/// # Arguments
///
/// * `handler` - MessageHandler
/// * `options` - &Opts
pub fn get_message_handler(
    handler: MessageHandlerKind,
    options: &CliArgs,
) -> Result<Box<dyn Messaging>> {
    match handler {
        MessageHandlerKind::Slack => {
            let slack = Slack::init(options)?;
            Ok(Box::new(slack))
        }
    }
}

/// Build foxsur message to be send to Slack
///
/// # Arguments
///
/// * `name` - &str
/// * `created` - usize
/// * `exists` - i64
/// * `not_found` - usize
pub fn build_foxsur_message(name: &str, created: usize, exists: i64, not_found: usize) -> String {
    format!(
        r#"
        Foxsur report for {}
        • {} instruments created
        • {} instruments already existing
        • {} unknown assets
        "#,
        name, created, exists, not_found
    )
}
