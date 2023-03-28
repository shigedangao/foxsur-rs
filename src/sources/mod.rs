use crate::database::instrument::Instrument as DBInstrument;
use crate::database::Handler;
use crate::options::Opts;
use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;

pub mod paxos;
pub mod rest_source;

#[async_trait]
pub trait SourceOps {
    /// Fetch retrieve the & handler list of instruments & assets
    ///
    /// # Arguments
    ///
    /// * `db_assets` - HashMap<String, i32>
    /// * `db_instruments` - HashMap<String, DBInstrument>
    /// * `opts` - &Opts
    fn fetch(
        &self,
        db_assets: HashMap<String, i32>,
        db_instruments: HashMap<String, DBInstrument>,
        opts: &Opts,
    ) -> Result<(Vec<(DBInstrument, String)>, i64, usize)>;
    /// Insert the list of instruments asynchronously
    ///
    /// # Arguments
    ///
    /// * `sources` - Vec<(DBInstrument, String)>
    /// * `handler` - &Handler
    /// * `opts` - &Opts
    async fn insert_bulk(
        &self,
        sources: Vec<(DBInstrument, String)>,
        handler: &Handler,
    ) -> Result<usize>;
}

// Use Box to allow different kind of source to be used
// Just need to implement the SourceOps trait
pub struct Sources {
    sources: HashMap<String, Box<dyn SourceOps>>,
}

impl Sources {
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
    pub fn register(&mut self, source: Box<dyn SourceOps>, source_name: &str) {
        self.sources.insert(source_name.to_string(), source);
    }
    /// Load a source from the list of source
    ///
    /// # Arguments
    ///
    /// * `source_name` - &str
    pub fn load(&self, source_name: &str) -> Option<&Box<dyn SourceOps>> {
        self.sources.get(source_name)
    }
}
