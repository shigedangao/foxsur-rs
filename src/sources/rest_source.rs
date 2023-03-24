use std::collections::{HashMap, HashSet};

use crate::instruments::Instrument;

use super::SourceOps;

pub trait RestSourceOps {
    fn normalize(&self, n: &str) -> String; 
}

pub struct RestSource {
    pub asset_mapping: Option<HashMap<String, String>>,
    pub code: String,
    pub get_from_exchange: fn(&str) -> Result<(Vec<Instrument>, HashSet<String>), Box<dyn std::error::Error>>,
    pub instrument_mapping: HashMap<String, String>,
    pub name: String,
    pub normalizer: Option<Box<dyn RestSourceOps>>,
    pub prefix: Option<String>
}

impl SourceOps for RestSource {
    fn fetch(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Fetching from {}", self.code);

        // @TODO check whether we should put that in a trait... like the example above
        let (instruments, assets) = (self.get_from_exchange)("foo")?;

        if let Some(normalizer) = &self.normalizer {
            normalizer.normalize("foo");
        }

        Ok(())
    }
}
