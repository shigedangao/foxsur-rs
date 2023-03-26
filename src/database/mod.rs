use crate::options::Opts;
use anyhow::{anyhow, Result};
use sqlx::pool::Pool;
use sqlx::postgres::PgPoolOptions;
use sqlx::Postgres;

pub mod asset;
pub mod instrument;

#[derive(Debug, Clone)]
pub struct Handler {
    pub pool: Pool<Postgres>,
}

pub async fn init_database_handler(opts: &Opts) -> Result<Handler> {
    let url = match &opts.database {
        Some(d) => format!(
            "postgres://{}:{}@{}/{}",
            d.username, d.password, d.host, d.database
        ),
        None => {
            return Err(anyhow!(
                "Database parameters are empty. Unable to construct the url"
            ))
        }
    };

    let pool = PgPoolOptions::new()
        .max_connections(opts.max_con)
        .connect(&url)
        .await?;

    Ok(Handler { pool })
}
