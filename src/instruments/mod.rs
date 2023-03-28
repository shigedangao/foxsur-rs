use crate::database::instrument::Instrument as DBInstrument;
use anyhow::Result;
use log::info;
use std::collections::{HashMap, HashSet};

pub mod deribit;
pub mod paxos;

#[derive(Debug, Clone)]
pub struct Instrument {
    pub symbol: String,
    pub base: String,
    pub quote: String,
    pub class: Option<String>,
}

pub trait GetInstrument {
    fn get_instrument() -> Result<(Vec<Instrument>, HashSet<String>)>;
}

impl Instrument {
    /// Check whether the instrument exist in the following situations:
    ///     - db_instruments (provided in the postgres database)
    ///     - instrument_mapping (provided in the RestSource)
    ///     - by using a prefix and checking in the database again
    ///
    /// # Arguments
    ///
    /// * `&self` - Instrument
    /// * `db_instruments` - &HashMap<String, DBInstrument>
    /// * `instrument_mapping` - &HashMap<String, String>
    /// * `prefix` - &Option<String>
    pub fn exist(
        &self,
        db_instruments: &HashMap<String, DBInstrument>,
        instrument_mapping: &HashMap<String, String>,
        prefix: &Option<String>,
    ) -> bool {
        if db_instruments.get(&self.symbol).is_some() {
            info!("instrument exist in database {}", self.symbol);

            return true;
        }

        if instrument_mapping.get(&self.symbol).is_some() {
            info!("instrument exist in database mapping {}", self.symbol);

            return true;
        }

        if let Some(p) = prefix {
            let generated_name = format!("{p}{}", self.symbol);
            if db_instruments.get(&generated_name).is_some() {
                info!(
                    "instrument exist in database mapping with prefix {}",
                    generated_name
                );

                return true;
            }
        }

        false
    }

    pub fn has_same_fa(
        &self,
        fa: &HashMap<String, i32>,
        auto_map: bool,
        target: &str,
    ) -> Option<i32> {
        match fa.get(target) {
            Some(v) => {
                if auto_map {
                    return Some(*v);
                }

                None
            }
            None => {
                info!(
                    "unable to find base asset {} while creating instrument {}",
                    self.base, self.symbol
                );
                None
            }
        }
    }
}
