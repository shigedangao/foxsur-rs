use std::collections::HashSet;

pub mod paxos;

#[derive(Debug)]
pub struct Instrument {
    symbol: String,
    base: String,
    quote: String
}

pub trait GetInstrument {
    fn get_instrument() -> Result<(Vec<Instrument>, HashSet<String>), Box<dyn std::error::Error>>;
}
