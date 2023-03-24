use crate::sources::SourceOps;
use crate::sources::paxos::Paxox;

mod sources;
mod instruments;

fn main() {
    env_logger::init();

    println!("starting up");

    // create a dumb source
    let mut sources = sources::Sources::new();

    //let rest_source = sources::rest_source::RestSource {
    //    url: "http://localhost:8080".to_string(),
    //};

    sources.register(Paxox::new(), "foo");

    let foo = sources.load("foo").unwrap();
    foo.fetch().unwrap();

    println!("end");
}
