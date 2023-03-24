use std::collections::HashSet;
use serde::Deserialize;
use super::{GetInstrument, Instrument};

pub struct PaxosHandler;

#[derive(Debug, Clone, Deserialize)]
struct PaxosInstrument {
    market: String,
    base_asset: String,
    quote_asset: String
}

impl GetInstrument for PaxosHandler {
    fn get_instrument() -> Result<(Vec<Instrument>, HashSet<String>), Box<dyn std::error::Error>> {
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
            set.insert(inst.base_asset.to_string());
            set.insert(inst.quote_asset.to_string());

            instruments.push(Instrument {
                symbol: inst.market.to_string(),
                base: inst.base_asset.to_string(),
                quote: inst.quote_asset.to_string()
            });
        }

        Ok((instruments, set))
    }
}

