use crate::sources::paxos::Paxox;
use crate::sources::SourceOps;

mod database;
mod instruments;
mod options;
mod sources;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // @TODO use the logger instead of println...
    // @TODO wrap this in a run function of something to boostrap it...
    let opts = options::Opts::new().expect("Expect to load environment variable");

    let db_handler = database::init_database_handler(&opts).await?;

    println!("starting up");

    // create a dumb source
    let mut sources = sources::Sources::new();

    //let rest_source = sources::rest_source::RestSource {
    //    url: "http://localhost:8080".to_string(),
    //};

    sources.register(Paxox::new(), "foo");
    let foo = sources.load("foo").unwrap();

    // get the list of assets
    let assets = database::asset::Assets::get_assets(&db_handler).await?;
    dbg!(&assets);

    let instruments =
        database::instrument::Instrument::get_instruments(&db_handler, &foo.code).await?;
    dbg!(&instruments);

    let inst_to_insert = foo.fetch(assets, instruments, &opts).unwrap();
    foo.create_bulk(inst_to_insert, &db_handler).await?;

    println!("end");

    Ok(())
}
