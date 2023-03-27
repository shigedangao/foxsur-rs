use super::{BulkOps, SourceOps};
use crate::database::instrument::Instrument as DBInstrument;
use crate::database::Handler;
use crate::instruments::Instrument;
use crate::options::Opts;
use anyhow::Result;
use async_trait::async_trait;
use futures::future;
use std::collections::{HashMap, HashSet};

// Based on the existing restsource.go
#[derive(Clone)]
pub struct RestSource {
    pub asset_mapping: Option<HashMap<String, String>>,
    pub code: String,
    pub get_from_exchange: fn(&str) -> Result<(Vec<Instrument>, HashSet<String>)>,
    pub instrument_mapping: HashMap<String, String>,
    pub name: String,
    pub normalizer: fn(&str) -> String,
    pub prefix: Option<String>,
    pub created: usize,
    pub exists: i64,
    pub not_found_asset_count: usize
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
            created: 0,
            exists: 0,
            not_found_asset_count: 0
        }
    }
}

impl SourceOps for RestSource {
    fn fetch(
        &mut self,
        db_asset: HashMap<String, i32>,
        db_insts: HashMap<String, DBInstrument>,
        opts: &Opts,
    ) -> Result<Vec<(DBInstrument, String)>> {
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
        let mut insts = Vec::new();
        for inst in instruments {
            if inst.exist(&db_insts, &self.instrument_mapping, &self.prefix) {
                self.exists += 1;
            }

            let bas = inst.has_same_fa(&fa, opts.auto_map, &inst.base);
            let qas = inst.has_same_fa(&fa, opts.auto_map, &inst.quote);

            let normalized_symbol = (self.normalizer)(&inst.symbol);

            let db_inst = DBInstrument {
                symbol: Some(inst.symbol),
                base_id: Some(bas),
                quote_id: Some(qas),
                class: inst.class,
                ..Default::default()
            };

            insts.push((db_inst, normalized_symbol));
        }

        self.not_found_asset_count = not_found_asset.len();

        Ok(insts)
    }

    fn build_message(&self) -> String {
        format!(r#"
            Foxsur report for {}
            • {} instruments created
            • {} instruments already existing
            • {} unknown assets
            "#,
            self.name,
            0,
            self.exists,
            self.not_found_asset_count
        )
    }
}

#[async_trait]
impl BulkOps for RestSource {
    async fn create_bulk(
        &mut self,
        sources: Vec<(DBInstrument, String)>,
        handler: &Handler,
    ) -> Result<Vec<Result<(), anyhow::Error>>> {
        let instruments_bulk: Vec<_> = sources
            .into_iter()
            .map(|(d, n)| d.insert_instrument(handler, &self.code, n))
            .collect();

        let bulk_length = instruments_bulk.len();

        let res = future::join_all(instruments_bulk).await;
        let errs = res
            .into_iter()
            .filter(|r| r.is_err())
            .collect::<Vec<_>>();

        if errs.len() == 0 {
            self.created = bulk_length;
        }

        Ok(errs)
    }
}
