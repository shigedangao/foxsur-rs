use crate::sources::paxos::Paxox;
use crate::sources::{BulkOps, SourceOps};
use crate::messaging::MessageHandlerKind;

mod database;
mod instruments;
mod options;
mod sources;
mod messaging;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // @TODO use the logger instead of println...
    // @TODO wrap this in a run function of something to boostrap it...
    let opts = options::Opts::new().expect("Expect to load environment variable");
    let db_handler = database::init_database_handler(&opts).await?;
    let message_handler = messaging::get_message_handler(MessageHandlerKind::Slack, &opts)?;

    println!("starting up");

    // create a dumb source
    let mut sources = sources::Sources::new();

    //let rest_source = sources::rest_source::RestSource {
    //    url: "http://localhost:8080".to_string(),
    //};

    sources.register(Paxox::new(), "foo");
    let mut foo = sources.load("foo").unwrap().to_owned();

    // get the list of assets
    let assets = database::asset::Assets::get_assets(&db_handler).await?;

    let instruments =
        database::instrument::Instrument::get_instruments(&db_handler, &foo.code).await?;

    let inst_to_insert = foo.fetch(assets, instruments, &opts).unwrap();    
    let errs = foo.insert_bulk(inst_to_insert, &db_handler).await?;

    // this would be treat in the run method or something else...
    if errs.is_empty() {
        errs.into_iter().for_each(|e| println!("{:?}", e.unwrap_err()));
        
        return Ok(());
    }

    // Send slack notif if everything went fine
    let msg = foo.build_message();
    message_handler.send(&msg).await?;

    println!("end");

    Ok(())
}
