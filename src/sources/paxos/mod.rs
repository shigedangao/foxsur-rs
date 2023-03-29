use super::rest_source::RestSource;
use super::Src;
use crate::instruments::paxos::PaxosHandler;
use crate::instruments::GetInstrument;
use std::collections::HashMap;

// Constant
const CODE: &str = "itbi";
pub const NAME: &str = "paxos";

pub struct Paxos;

impl Src<RestSource> for Paxos {
    fn get_source() -> RestSource {
        RestSource {
            instrument_mapping: HashMap::from([
                ("BTCEUR".to_string(), "XBTEUR".to_string()),
                ("BTCSGD".to_string(), "XBTSGD".to_string()),
                ("BTCUSD".to_string(), "XBTUSD".to_string()),
            ]),
            code: CODE.to_string(),
            get_from_exchange: PaxosHandler::get_instrument,
            name: NAME.to_string(),
            normalizer: |s, _| s.to_lowercase(),
            ..Default::default()
        }
    }
}
