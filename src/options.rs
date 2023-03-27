use anyhow::Result;
use serde::Deserialize;
use serde_env::from_env;
use std::default::Default;

#[derive(Debug, Deserialize)]
pub struct Opts {
    pub max_con: u32,
    pub database: Option<DatabaseOpts>,
    pub auto_map: bool,
}

#[derive(Debug, Default, Deserialize)]
pub struct DatabaseOpts {
    pub host: String,
    pub username: String,
    pub password: String,
    pub database: String,
}

impl Default for Opts {
    fn default() -> Self {
        Opts {
            max_con: 5,
            database: None,
            auto_map: true,
        }
    }
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
