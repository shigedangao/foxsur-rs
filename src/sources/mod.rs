use crate::database::instrument::Instrument as DBInstrument;
use crate::database::Handler;
use crate::options::Opts;
use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;

pub mod paxos;
pub mod rest_source;

pub trait SourceOps {
    /// Fetch retrieve the & handler list of instruments & assets
    ///
    /// # Arguments
    ///
    /// * `db_assets` - HashMap<String, i32>
    /// * `db_instruments` - HashMap<String, DBInstrument>
    /// * `opts` - &Opts
    fn fetch(
        &mut self,
        db_assets: HashMap<String, i32>,
        db_instruments: HashMap<String, DBInstrument>,
        opts: &Opts,
    ) -> Result<Vec<(DBInstrument, String)>>;
    /// Build the message to be sent when the source has been procssed
    fn build_message(&self) -> String;
}

#[async_trait]
pub trait BulkOps {
    /// Insert the list of instruments asynchronously
    ///
    /// # Arguments
    ///
    /// * `sources` - Vec<(DBInstrument, String)>
    /// * `handler` - &Handler
    /// * `opts` - &Opts
    async fn insert_bulk(
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
    /// Create a new Source handler
    pub fn new() -> Self {
        Self {
            sources: HashMap::new(),
        }
    }
    /// Register a source within the list of source
    ///
    /// # Arguments
    ///
    /// * `source` - T
    /// * `source_name` - &str
    pub fn register(&mut self, source: T, source_name: &str) {
        self.sources.insert(source_name.to_string(), source);
    }
    /// Load a source from the list of source
    ///
    /// # Arguments
    ///
    /// * `source_name` - &str
    pub fn load(&self, source_name: &str) -> Option<&T> {
        self.sources.get(source_name)
    }
}
