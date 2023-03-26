use crate::database::instrument::Instrument as DBInstrument;
use crate::{
    database::{instrument::Instrument},
    options::Opts,
};
use std::collections::HashMap;

pub mod paxos;
pub mod rest_source;

pub trait SourceOps {
    fn fetch(
        &self,
        db_assets: HashMap<String, i64>,
        db_instruments: HashMap<String, Instrument>,
        opts: &Opts,
    ) -> Result<Vec<(DBInstrument, String)>, Box<dyn std::error::Error>>;
}

pub struct Sources<T>
where
    T: SourceOps,
{
    sources: HashMap<String, T>,
}

impl<T> Sources<T>
where
    T: SourceOps,
{
    pub fn new() -> Self {
        Self {
            sources: HashMap::new(),
        }
    }

    pub fn register(&mut self, source: T, source_name: &str) {
        self.sources.insert(source_name.to_string(), source);
    }

    pub fn load(&self, source_name: &str) -> Option<&T> {
        self.sources.get(source_name)
    }
}
