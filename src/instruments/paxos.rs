use super::{GetInstrument, Instrument};
use anyhow::Result;
use async_trait::async_trait;
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

#[async_trait]
impl GetInstrument for PaxosHandler {
    async fn get_instrument() -> Result<(Vec<Instrument>, HashSet<String>)> {
        // Use blocking for now as Rust does not support async fn pointer...
        // thought do we really need to use async there ? as we're not going to do anything
        // while we wait for the response so it kinda makes sense to use blocking here.
        // /!\ Note as we run the program in an async context we need to use the block_in_place or it'd panic.
        let resp = reqwest::get(PAXOS_URL).await?.json::<Value>().await?;
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
                });
            }
        }

        Ok((instruments, set))
    }
}
