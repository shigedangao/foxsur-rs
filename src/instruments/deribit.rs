use super::{GetInstrument, Instrument};
use anyhow::Result;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashSet;
use std::thread;

// Constant
const DERIBIT_URL: &str = "https://www.deribit.com/api/v2/public";

pub struct DeribitHandler;

#[derive(Debug, Clone, Deserialize)]
pub struct DeribitInstrument {
    instrument_name: String,
    base_currency: String,
    counter_currency: String,
    kind: String,
}

impl DeribitInstrument {
    fn get_instrument_class(&self) -> String {
        if self.kind.ends_with("PERPETUAL") {
            return "perpetual-future".to_string();
        }

        self.kind.to_string()
    }
}

impl GetInstrument for DeribitHandler {
    fn get_instrument() -> Result<(Vec<Instrument>, HashSet<String>)> {
        let mut set = HashSet::new();
        let mut insts = Vec::new();

        let cresp = reqwest::blocking::get(format!("{}/get_currencies", DERIBIT_URL))?
            .json::<Value>()?;

        let Some(results) = cresp.get("result").and_then(|o| o.as_array()) else {
            return Err(anyhow::anyhow!("No result found"));
        };

        let currencies = results
            .iter()
            .filter_map(|e| e.get("currency").and_then(|c| c.as_str()))
            .collect::<Vec<_>>();

        // Call deribit endpoints from the vector of currencies
        let drbt_instruments = get_deribit_instruments_for_currencies(currencies)?;

        for inst in drbt_instruments {
            set.insert(inst.base_currency.clone());
            set.insert(inst.counter_currency.clone());

            let class = inst.get_instrument_class();
            insts.push(Instrument {
                symbol: inst.instrument_name,
                base: inst.base_currency,
                quote: inst.counter_currency,
                class: Some(class),
            });
        }

        Ok((insts, set))
    }
}

fn get_deribit_instruments_for_currencies(
    currencies: Vec<&str>,
) -> Result<Vec<DeribitInstrument>> {
    let mut handlers = Vec::new();
    let mut drbt_inst = Vec::new();

    for currency in currencies {
        let endpoint = format!("{DERIBIT_URL}/get_instruments?currency={currency}");

        let handler = thread::spawn(move || {
            let res = reqwest::blocking::get(endpoint)?.json::<Value>();
            let mut inner_drbt_inst = Vec::new();

            let Ok(result) = res else {
                return Err(anyhow::anyhow!("An error happened while fetching deribit instrument"));
            };

            let Some(result) = result.get("result").and_then(|o| o.as_array()) else {
                return Err(anyhow::anyhow!("No result found"));
            };

            for r in result.iter().cloned() {
                let inst: DeribitInstrument = serde_json::from_value(r)?;
                inner_drbt_inst.push(inst);
            }

            Ok(inner_drbt_inst)
        });

        handlers.push(handler);
    }

    for handler in handlers {
        let res = handler
            .join()
            .map_err(|err| anyhow::anyhow!("An error happened while fetching deribit instrument"))?;

        let inst = res?;
        drbt_inst.extend(inst);
    }

    Ok(drbt_inst)
}
