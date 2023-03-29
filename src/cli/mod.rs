use crate::database;
use crate::messaging::{self, MessageHandlerKind};
use crate::sources::{deribit, paxos};
use crate::sources::{Sources, Src};
use anyhow::{anyhow, Result};
use clap::Parser;
use options::CliArgs;

pub mod options;

pub struct Cli {
    pub args: CliArgs,
    pub sources: Sources,
}

impl Cli {
    pub fn start() -> Cli {
        let args = CliArgs::parse();

        Cli {
            args,
            sources: Sources::new(),
        }
    }

    /// Load the sources that may be used by foxsur
    pub fn register_source(&mut self) {
        self.sources
            .register(Box::new(paxos::Paxos::get_source()), paxos::NAME)
            .register(Box::new(deribit::Deribit::get_source()), deribit::NAME);
    }

    /// Run foxsur by:
    ///     - Loading the target source
    ///     - Fetching assets & instruments from the database
    ///     - Fetching the targeted source that we may need to found
    ///     - Generate a list of instruments to be inserted
    ///     - Insert the instruments that's missing
    ///     - Send a notification with the number of instruments that has been inserted
    pub fn run(&self) -> Result<()> {
        let Some(target_source) = self.sources.load(&self.args.source) else {
            return Err(anyhow!("Source not found"));
        };

        let db_handler = database::init_database_handler(&self.args)?;
        let message_handler =
            messaging::get_message_handler(MessageHandlerKind::Slack, &self.args)?;

        // Get the necessary data from the database...
        let assets = database::asset::Assets::get_assets(db_handler.client.clone())?;
        let instruments = database::instrument::Instrument::get_instruments(
            db_handler.client.clone(),
            &target_source.get_code(),
        )?;

        // Fetch the data from the source and the instrument that we may need to insert
        let (inst_to_insert, exists_count, not_found_count) =
            target_source.fetch(assets, instruments, &self.args)?;
        // Insert the data into the database
        if inst_to_insert.is_empty() {
            return Ok(());
        }

        let inserted_count = target_source.insert_bulk(inst_to_insert, db_handler.client)?;

        // Send message notif if everything went fine
        let msg =
            messaging::build_foxsur_message("Paxos", inserted_count, exists_count, not_found_count);
        message_handler.send(&msg)?;

        Ok(())
    }
}
