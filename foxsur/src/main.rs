use log::{error, info};

mod cli;
mod database;
mod instruments;
mod sources;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("starting up");

    let mut cli = cli::Cli::start();
    // Load all the sources
    cli.register_source();
    // Run the command line source
    match cli.run() {
        Ok(_) => info!("success"),
        Err(e) => error!("error: {}", e),
    }

    info!("end");

    Ok(())
}
