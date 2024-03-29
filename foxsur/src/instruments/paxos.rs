use super::{GetInstrument, Instrument};
use anyhow::Result;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashSet;

// Constant
const PAXOS_URL: &str = "https://api.paxos.com/v2/markets";

pub struct PaxosHandler;

#[derive(Debug, Clone, Deserialize)]
struct PaxosInstrument {
    market: String,
    base_asset: String,
    quote_asset: String,
}

impl GetInstrument for PaxosHandler {
    fn get_instrument() -> Result<(Vec<Instrument>, HashSet<String>)> {
        let resp = reqwest::blocking::get(PAXOS_URL)?.json::<Value>()?;
        let Some(markets) = resp.get("markets") else {
            return Err(anyhow::anyhow!("No markets found"));
        };

        let mut instruments = Vec::new();
        let mut set = HashSet::new();

        if let Some(markets_vec) = markets.as_array() {
            for value in markets_vec.iter().cloned() {
                let inst: PaxosInstrument = serde_json::from_value(value)?;

                set.insert(inst.base_asset.clone());
                set.insert(inst.quote_asset.clone());

                instruments.push(Instrument {
                    symbol: inst.market,
                    base: inst.base_asset,
                    quote: inst.quote_asset,
                    class: None,
                    ..Default::default()
                });
            }
        }

        Ok((instruments, set))
    }
}
