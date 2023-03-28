use crate::cli::options::CliArgs;
use anyhow::Result;
use sqlx::pool::Pool;
use sqlx::postgres::PgPoolOptions;
use sqlx::Postgres;

pub mod asset;
pub mod instrument;

#[derive(Debug, Clone)]
pub struct Handler {
    pub pool: Pool<Postgres>,
}

/// Create a pool of connection to the database
///
/// # Arguments
///
/// * `opts` - &CliArgs
pub async fn init_database_handler(opts: &CliArgs) -> Result<Handler> {
    let d = &opts.database;
    let url = format!(
        "postgres://{}:{}@{}/{}",
        d.username, d.password, d.host, d.database
    );

    let pool = PgPoolOptions::new()
        .max_connections(opts.max_con)
        .connect(&url)
        .await?;

    Ok(Handler { pool })
}
