use crate::messaging::MessageHandlerKind;
use crate::sources::paxos::Paxox;

mod database;
mod instruments;
mod messaging;
mod options;
mod sources;

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

    sources.register(Box::new(Paxox::new()), "foo");
    let foo = sources.load("foo").unwrap();

    // get the list of assets
    let assets = database::asset::Assets::get_assets(&db_handler).await?;

    let instruments = database::instrument::Instrument::get_instruments(&db_handler, "foo").await?;

    let (inst_to_insert, exists_count, not_found_count) =
        foo.fetch(assets, instruments, &opts).unwrap();
    let inserted_count = foo.insert_bulk(inst_to_insert, &db_handler).await?;

    // Send slack notif if everything went fine
    let msg =
        messaging::build_foxsur_message("Paxos", inserted_count, exists_count, not_found_count);
    message_handler.send(&msg).await?;

    println!("end");

    Ok(())
}
