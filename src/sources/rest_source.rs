use super::SourceOps;
use crate::database::instrument::Instrument as DBInstrument;
use crate::database::Handler;
use crate::instruments::Instrument;
use crate::options::Opts;
use futures::future;
use std::collections::{HashMap, HashSet};

pub trait RestSourceOps {
    fn normalize(&self, n: &str) -> String;
}

// Based on the existing restsource.go
pub struct RestSource {
    pub asset_mapping: Option<HashMap<String, String>>,
    pub code: String,
    pub get_from_exchange:
        fn(&str) -> Result<(Vec<Instrument>, HashSet<String>), Box<dyn std::error::Error>>,
    pub instrument_mapping: HashMap<String, String>,
    pub name: String,
    pub normalizer: Option<Box<dyn RestSourceOps>>,
    pub prefix: Option<String>,
}

impl SourceOps for RestSource {
    fn fetch(
        &self,
        db_asset: HashMap<String, i64>,
        db_insts: HashMap<String, DBInstrument>,
        opts: &Opts,
    ) -> Result<Vec<(DBInstrument, String)>, Box<dyn std::error::Error>> {
        let mut fa = HashMap::new();
        let mut not_found_asset = HashSet::new();

        println!("Fetching from {}", self.code);

        let (instruments, assets) = (self.get_from_exchange)("foo")?;

        for asset in assets {
            if let Some(asset_mapping) = &self.asset_mapping {
                if let Some(am) = asset_mapping.get(&asset) {
                    fa.insert(asset.clone(), *db_asset.get(am).unwrap_or(&0));
                    println!("replacing {asset} by {am}");

                    continue;
                }
            }

            if db_asset.get(&asset).is_none() {
                not_found_asset.insert(asset.to_owned());
                println!("asset not found for: {}", &asset);
            }

            fa.insert(asset.to_owned(), *db_asset.get(&asset).unwrap_or(&0));
        }

        // Used for slack purposes it seeems ?
        //let mut exists = 0;
        let mut insts = Vec::new();
        for inst in instruments {
            if inst.exist(&db_insts, &self.instrument_mapping, &self.prefix) {
                // exists += 1;
            }

            let bas = inst.has_same_fa(&fa, opts.auto_map, &inst.base);
            let qas = inst.has_same_fa(&fa, opts.auto_map, &inst.quote);

            let normalized_symbol = if let Some(normalizer) = &self.normalizer {
                normalizer.normalize(&inst.symbol)
            } else {
                inst.symbol.clone()
            };

            let db_inst = DBInstrument {
                symbol: Some(inst.symbol),
                base_id: Some(bas),
                quote_id: Some(qas),
                class: inst.class,
                ..Default::default()
            };

            insts.push((db_inst, normalized_symbol));
        }

        Ok(insts)
    }
}

// TODO see if we can use async_trait here...
impl RestSource {
    pub async fn create_bulk(
        &self,
        sources: Vec<(DBInstrument, String)>,
        handler: &Handler,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let instruments_bulk: Vec<_> = sources
            .into_iter()
            .map(|(d, n)| d.insert_instrument(handler, &self.code, n))
            .collect();

        let res = future::join_all(instruments_bulk).await;
        for item in res {
            if let Err(err) = item {
                println!("Unable to push instrument due to {}", err.to_string());
            }
        }

        Ok(())
    }
}
