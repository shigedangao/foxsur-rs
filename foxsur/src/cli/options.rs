use clap::{Args, Parser};

#[derive(Parser, Debug)]
#[command(
    author = "marc.inthaamnouay@gmail.com & jean.grimonprez@gmail.com",
    version = "0.1.0",
    about = "Foxsur RS demonstration"
)]
pub struct CliArgs {
    #[arg(short, long, env = "MAX_CON", default_value = "10")]
    pub max_con: u32,
    #[arg(short, long, env = "AUTO_MAP", default_value = "true")]
    pub auto_map: bool,
    #[command(flatten)]
    pub database: DatabaseOpts,
    #[arg(short, long, env = "SOURCE")]
    pub source: String,
}

#[derive(Debug, Args, Clone)]
pub struct DatabaseOpts {
    #[arg(long, env = "DATABASE_HOST", default_value = "localhost")]
    pub host: String,
    #[arg(long, env = "DATABASE_USERNAME")]
    pub username: String,
    #[arg(long, env = "DATABASE_PASSWORD")]
    pub password: String,
    #[arg(long, env = "DATABASE_NAME", default_value = "foxsur")]
    pub database: String,
}
