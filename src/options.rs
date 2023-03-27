use anyhow::Result;
use serde::Deserialize;
use serde_env::from_env;

#[derive(Debug, Default, Deserialize)]
pub struct Opts {
    pub max_con: u32,
    pub database: Option<DatabaseOpts>,
    pub auto_map: bool,
    pub slack: Option<Slack>,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseOpts {
    pub host: String,
    pub username: String,
    pub password: String,
    pub database: String,
}

#[derive(Debug, Deserialize)]
pub struct Slack {
    pub bot_token: String,
    pub channel_id: String,
}

impl Opts {
    /// Create a new environment variable
    pub fn new() -> Result<Self> {
        let env = match from_env() {
            Ok(res) => res,
            // Ignore error for now
            Err(_) => Opts::default(),
        };

        Ok(env)
    }
}
