use super::{GetInstrument, Instrument};
use anyhow::Result;
use serde::Deserialize;
use std::collections::HashSet;

pub struct PaxosHandler;

#[derive(Debug, Clone, Deserialize)]
struct PaxosInstrument {
    market: String,
    base_asset: String,
    quote_asset: String,
}

impl GetInstrument for PaxosHandler {
    fn get_instrument() -> Result<(Vec<Instrument>, HashSet<String>)> {
        let data = r#"
        [
            {
                "market": "BTCUSD",
                "base_asset": "BTC",
                "quote_asset": "USD"
            },
            {
                "market": "BTCEUR",
                "base_asset": "BTC",
                "quote_asset": "EUR"
            },
            {
                "market": "BTCSGD",
                "base_asset": "BTC",
                "quote_asset": "SGD"
            }
        ]
        "#;

        let paxos_instruments: Vec<PaxosInstrument> = serde_json::from_str(data)?;
        let mut set = HashSet::new();
        let mut instruments = Vec::new();

        for inst in &paxos_instruments {
            set.insert(inst.base_asset.to_lowercase());
            set.insert(inst.quote_asset.to_lowercase());

            instruments.push(Instrument {
                symbol: inst.market.to_lowercase(),
                base: inst.base_asset.to_lowercase(),
                quote: inst.quote_asset.to_lowercase(),
                class: None,
            });
        }

        Ok((instruments, set))
    }
}
