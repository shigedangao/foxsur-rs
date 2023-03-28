use super::SourceOps;
use crate::database::instrument::Instrument as DBInstrument;
use crate::database::Handler;
use crate::instruments::Instrument;
use crate::options::Opts;
use anyhow::Result;
use async_trait::async_trait;
use futures::future;
use log::info;
use std::collections::{HashMap, HashSet};

/// ResSource is a rest source that will fetch asset & instrument.
/// It'll also compare the asset & instruments with the one stored in the database
/// and prepare a set of new payload to be inserted.
#[derive(Clone)]
pub struct RestSource {
    pub asset_mapping: Option<HashMap<String, String>>,
    pub code: String,
    pub get_from_exchange: fn(&str) -> Result<(Vec<Instrument>, HashSet<String>)>,
    pub instrument_mapping: HashMap<String, String>,
    pub name: String,
    pub normalizer: fn(&str) -> String,
    pub prefix: Option<String>,
}

impl Default for RestSource {
    fn default() -> Self {
        Self {
            asset_mapping: None,
            code: String::new(),
            get_from_exchange: |_| Ok((Vec::new(), HashSet::new())),
            instrument_mapping: HashMap::new(),
            name: String::new(),
            normalizer: |s| s.to_string(),
            prefix: None,
        }
    }
}

#[async_trait]
impl SourceOps for RestSource {
    fn fetch(
        &self,
        db_asset: HashMap<String, i32>,
        db_insts: HashMap<String, DBInstrument>,
        opts: &Opts,
    ) -> Result<(Vec<(DBInstrument, String)>, i64, usize)> {
        // Fa is the list of asset which exists in the exchange & in the database
        let mut fa = HashMap::new();
        let mut not_found_asset = HashSet::new();

        info!("Fetching from {}", self.code);

        let (instruments, assets) = (self.get_from_exchange)("foo")?;

        // Compare the assets that has been retrieve from the exchange
        // With the one that we have in the mapping or the database
        for asset in assets {
            if let Some(asset_mapping) = &self.asset_mapping {
                if let Some(am) = asset_mapping.get(&asset) {
                    fa.insert(asset.clone(), *db_asset.get(am).unwrap_or(&0));
                    info!("replacing {asset} by {am}");

                    continue;
                }
            }

            // If it do not exist, we just store that information somewhere...
            if db_asset.get(&asset).is_none() {
                not_found_asset.insert(asset.to_owned());
                info!("asset not found for: {}", &asset);
            }

            // add the new asset
            fa.insert(asset.to_owned(), *db_asset.get(&asset).unwrap_or(&0));
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

            let normalized_symbol = (self.normalizer)(&inst.symbol);

            // Create a new DBInstrument which we'll push in the database as part of the new mapping
            let db_inst = DBInstrument {
                symbol: Some(inst.symbol),
                base_id: Some(bas),
                quote_id: Some(qas),
                class: inst.class,
                ..Default::default()
            };

            insts.push((db_inst, normalized_symbol));
        }

        Ok((insts, exists, not_found_asset.len()))
    }

    async fn insert_bulk(
        &self,
        sources: Vec<(DBInstrument, String)>,
        handler: &Handler,
    ) -> Result<usize> {
        let instruments_bulk: Vec<_> = sources
            .into_iter()
            .map(|(d, n)| d.insert_instrument(handler, &self.code, n))
            .collect();

        let bulk_length = instruments_bulk.len();

        let res = future::join_all(instruments_bulk).await;
        let mut errs = res
            .into_iter()
            .filter(|r| r.is_err())
            .map(|err| err.unwrap_err())
            .collect::<Vec<_>>();

        match errs.pop() {
            Some(err) => Err(err),
            _ => Ok(bulk_length),
        }
    }
}
