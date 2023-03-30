use super::{GetInstrument, Instrument};
use anyhow::Result;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashSet;

// Constant
const KRAKEN_URL: &str = "https://api.kraken.com/0/public/AssetPairs";

pub struct KrakenHandler;

#[derive(Debug, Clone, Deserialize)]
struct KrakenInstrument {
    wsname: String,
    altname: String,
}

impl GetInstrument for KrakenHandler {
    fn get_instrument() -> Result<(Vec<Instrument>, HashSet<String>)> {
        let resp = reqwest::blocking::get(KRAKEN_URL)?.json::<Value>()?;
        let Some(res) = resp.get("result").and_then(|r| r.as_object()).cloned() else {
            return Err(anyhow::anyhow!("No result found"));
        };

        let mut instruments = Vec::new();
        let mut set = HashSet::new();

        for (_, v) in res {
            let inst: KrakenInstrument = serde_json::from_value(v)?;

            let ws = inst.wsname.clone();

            let aa: Vec<&str> = ws.split('/').collect();
            let base_asset = aa.first().unwrap_or(&"");
            let quote_asset = aa.get(1).unwrap_or(&"");

            set.insert(base_asset.to_string());
            set.insert(quote_asset.to_string());

            instruments.push(Instrument {
                symbol: inst.altname,
                base: base_asset.to_string(),
                quote: quote_asset.to_string(),
                raw_symbol: Some(inst.wsname),
                class: Some("spot".to_string()),
            });
        }

        Ok((instruments, set))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_to_load_kraken_instruments() {
        let res = KrakenHandler::get_instrument();

        assert!(res.is_ok());
    }
}
