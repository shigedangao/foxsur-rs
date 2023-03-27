use crate::instruments::paxos::PaxosHandler;
use crate::instruments::GetInstrument;
use std::collections::HashMap;

use super::rest_source::RestSource;

// Constant
const CODE: &str = "itbi";
const NAME: &str = "paxos";

pub struct Paxox;

impl Paxox {
    pub fn new() -> RestSource {
        RestSource {
            asset_mapping: None,
            instrument_mapping: HashMap::from([
                ("BTCEUR".to_string(), "XBTEUR".to_string()),
                ("BTCSGD".to_string(), "XBTSGD".to_string()),
                ("BTCUSD".to_string(), "XBTUSD".to_string()),
            ]),
            code: CODE.to_string(),
            get_from_exchange: |_| PaxosHandler::get_instrument(),
            name: NAME.to_string(),
            normalizer: |s| s.to_string(),
            prefix: None,
        }
    }
}
