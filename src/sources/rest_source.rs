use super::SourceOps;
use crate::cli::options::CliArgs;
use crate::database::instrument::Instrument as DBInstrument;
use crate::instruments::Instrument;
use anyhow::Result;
use log::{info, warn};
use postgres::Client;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::thread;

type ExchangeFnRes = Result<(Vec<Instrument>, HashSet<String>)>;

/// ResSource is a rest source that will fetch asset & instrument.
/// It'll also compare the asset & instruments with the one stored in the database
/// and prepare a set of new payload to be inserted.
#[derive(Clone)]
pub struct RestSource {
    pub asset_mapping: Option<HashMap<String, String>>,
    pub code: String,
    pub get_from_exchange: fn() -> ExchangeFnRes,
    pub instrument_mapping: HashMap<String, String>,
    pub name: String,
    pub normalizer: fn(&str, &Option<Regex>) -> String,
    pub prefix: Option<String>,
    pub regex: Option<String>,
}

// Default trait has been implemented manually as it can't default the function pointer
impl Default for RestSource {
    fn default() -> Self {
        Self {
            asset_mapping: None,
            code: String::new(),
            get_from_exchange: || Ok((vec![], HashSet::new())),
            instrument_mapping: HashMap::new(),
            name: String::new(),
            normalizer: |s, _| s.to_string(),
            prefix: None,
            regex: None,
        }
    }
}

impl SourceOps for RestSource {
    fn fetch(
        &self,
        db_asset: HashMap<String, i32>,
        db_insts: HashMap<String, DBInstrument>,
        opts: &CliArgs,
    ) -> Result<(Vec<(DBInstrument, String)>, i64, usize)> {
        // Fa is the list of asset which exists in the exchange & in the database
        let mut fa = HashMap::new();
        let mut not_found_asset = HashSet::new();

        info!("Fetching from {}", self.code);

        // Compiling the regex if needed
        let re = if let Some(rg) = &self.regex {
            let re = regex::Regex::new(rg)?;
            Some(re)
        } else {
            None
        };

        let (instruments, assets) = (self.get_from_exchange)()?;

        // Compare the assets that has been retrieve from the exchange
        // With the one that we have in the mapping or the database
        for asset in assets {
            // There's a mix of lowercase for the asset and uppercase for the instrument which makes it hard to compare
            // A general format would be nicer to avoid these cases
            let asset_lc = asset.to_lowercase();
            if let Some(asset_mapping) = &self.asset_mapping {
                if let Some(am) = asset_mapping.get(&asset_lc) {
                    fa.insert(asset.clone(), *db_asset.get(am).unwrap_or(&0));
                    info!("replacing {asset} by {am}");

                    continue;
                }
            }

            // If it do not exist, we just store that information somewhere...
            if db_asset.get(&asset_lc).is_none() {
                not_found_asset.insert(asset_lc.to_owned());
                info!("asset not found for: {}", &asset);
            }

            // add the new asset
            fa.insert(asset.to_owned(), *db_asset.get(&asset_lc).unwrap_or(&0));
        }

        let mut insts = Vec::new();
        let mut exists = 0_i64;
        for inst in instruments {
            // If the instrument already exists in the database, we skip it
            if inst.exist(&db_insts, &self.instrument_mapping, &self.prefix) {
                exists += 1;
                continue;
            }

            // Otherwise get the asset & quote id from the instrument
            let bas = inst.has_same_fa(&fa, opts.auto_map, &inst.base);
            let qas = inst.has_same_fa(&fa, opts.auto_map, &inst.quote);

            let normalized_symbol = (self.normalizer)(&inst.symbol, &re);

            // Create a new DBInstrument which we'll push in the database as part of the new mapping
            let db_inst = DBInstrument {
                symbol: Some(inst.symbol),
                base_id: bas,
                quote_id: qas,
                class: inst.class,
                ..Default::default()
            };

            insts.push((db_inst, normalized_symbol));
        }

        Ok((insts, exists, not_found_asset.len()))
    }

    fn get_code(&self) -> String {
        self.code.clone()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn insert_bulk(
        &self,
        sources: Vec<(DBInstrument, String)>,
        handler: Arc<Mutex<Client>>,
    ) -> Result<usize> {
        let mut handles = Vec::new();
        let handler_clone = handler;

        for (inst, symbol) in sources {
            let code = self.code.to_string();
            let handler_clone = handler_clone.clone();
            let handle = thread::spawn(move || inst.insert_instrument(handler_clone, code, symbol));

            handles.push(handle);
        }

        let mut inserted = 0;
        for handle in handles {
            let res = handle
                .join()
                .map_err(|_| anyhow::anyhow!("Unable to insert instruments"))?;

            match res {
                Ok(_) => inserted += 1,
                Err(e) => warn!("Error while inserting instrument: {}", e),
            };
        }

        Ok(inserted)
    }
}
