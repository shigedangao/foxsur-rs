use crate::database::instrument::Instrument as DBInstrument;
use crate::database::Handler;
use crate::options::Opts;
use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;

pub mod paxos;
pub mod rest_source;

pub trait SourceOps {
    fn fetch(
        &mut self,
        db_assets: HashMap<String, i32>,
        db_instruments: HashMap<String, DBInstrument>,
        opts: &Opts,
    ) -> Result<Vec<(DBInstrument, String)>>;

    fn build_message(&self) -> String;
}

#[async_trait]
pub trait BulkOps {
    async fn create_bulk(
        &mut self,
        sources: Vec<(DBInstrument, String)>,
        handler: &Handler,
    ) -> Result<Vec<Result<(), anyhow::Error>>>;
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
