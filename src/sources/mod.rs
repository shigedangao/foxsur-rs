use crate::cli::options::CliArgs;
use crate::database::instrument::Instrument as DBInstrument;
use crate::database::Handler;
use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;

pub mod deribit;
pub mod paxos;
pub mod rest_source;

type FetchRes = (Vec<(DBInstrument, String)>, i64, usize);

pub trait Src<T> {
    /// Get Source return a source that can be used by Foxsur
    fn get_source() -> T;
}

#[async_trait]
pub trait SourceOps {
    /// Fetch retrieve the & handler list of instruments & assets
    ///
    /// # Arguments
    ///
    /// * `db_assets` - HashMap<String, i32>
    /// * `db_instruments` - HashMap<String, DBInstrument>
    /// * `opts` - &Opts
    async fn fetch(
        &self,
        db_assets: HashMap<String, i32>,
        db_instruments: HashMap<String, DBInstrument>,
        opts: &CliArgs,
    ) -> Result<FetchRes>;
    /// Get the code of the source
    fn get_code(&self) -> String;
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
    pub fn register(&mut self, source: Box<dyn SourceOps>, source_name: &str) -> &mut Self {
        self.sources.insert(source_name.to_string(), source);

        self
    }
    /// Load a source from the list of source
    ///
    /// # Arguments
    ///
    /// * `source_name` - &str
    pub fn load(&self, source_name: &str) -> Option<&dyn SourceOps> {
        match self.sources.get(source_name) {
            Some(source) => Some(source.as_ref()),
            None => None,
        }
    }
}
