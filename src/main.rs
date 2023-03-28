use log::info;

mod cli;
mod database;
mod instruments;
mod messaging;
mod sources;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("starting up");

    let mut cli = cli::Cli::start();
    // Load all the sources
    cli.register_source();
    // Run the command line source
    cli.run().await?;

    info!("end");

    Ok(())
}
