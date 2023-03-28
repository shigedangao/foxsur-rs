use super::{GetInstrument, Instrument};
use anyhow::Result;
use futures::future;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashSet;
use tokio::runtime::Handle;

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

        let cresp = tokio::task::block_in_place(|| {
            Handle::current().block_on(async {
                reqwest::get(format!("{}/get_currencies", DERIBIT_URL))
                    .await?
                    .json::<Value>()
                    .await
            })
        })?;

        let Some(results) = cresp.get("result").and_then(|o| o.as_array()) else {
            return Err(anyhow::anyhow!("No result found"));
        };

        let currencies = results
            .iter()
            .filter_map(|e| e.get("currency").and_then(|c| c.as_str()))
            .collect::<Vec<_>>();

        // Call deribit endpoints from the vector of currencies
        let drbt_instruments = tokio::task::block_in_place(move || {
            Handle::current()
                .block_on(async { get_deribit_instruments_for_currencies(currencies).await })
        })?;

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

async fn get_deribit_instruments_for_currencies(
    currencies: Vec<&str>,
) -> Result<Vec<DeribitInstrument>> {
    let mut req_tasks = Vec::new();
    let mut drbt_inst = Vec::new();

    for currency in currencies {
        let endpoint = format!("{DERIBIT_URL}/get_instruments?currency={currency}");
        let task = reqwest::get(endpoint);

        req_tasks.push(task);
    }

    let req_tasks_fut = future::join_all(req_tasks).await;
    for req_res in req_tasks_fut {
        let resp = req_res?;
        let result = resp.json::<Value>().await?;

        let Some(result) = result.get("result").and_then(|o| o.as_array()) else {
            return Err(anyhow::anyhow!("No result found"));
        };

        for r in result.iter().cloned() {
            let inst: DeribitInstrument = serde_json::from_value(r)?;
            drbt_inst.push(inst);
        }
    }

    Ok(drbt_inst)
}
