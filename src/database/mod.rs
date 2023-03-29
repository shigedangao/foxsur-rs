use crate::cli::options::CliArgs;
use anyhow::Result;
use postgres::{Client, NoTls};
use std::sync::{Arc, Mutex};

pub mod asset;
pub mod instrument;

#[derive(Clone)]
pub struct Handler {
    pub client: Arc<Mutex<Client>>,
}

/// Create a pool of connection to the database
///
/// # Arguments
///
/// * `opts` - &CliArgs
pub fn init_database_handler(opts: &CliArgs) -> Result<Handler> {
    let d = &opts.database;
    let url = format!(
        "postgres://{}:{}@{}/{}",
        d.username, d.password, d.host, d.database
    );

    let client = Client::connect(&url, NoTls)?;

    Ok(Handler {
        client: Arc::new(Mutex::new(client)),
    })
}
