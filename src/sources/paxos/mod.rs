use super::rest_source::RestSource;
use super::Src;
use crate::instruments::paxos::PaxosHandler;
use crate::instruments::GetInstrument;
use std::collections::HashMap;

// Constant
const CODE: &str = "itbi";
pub const NAME: &str = "paxos";

pub struct Paxos;

impl<'a> Src<RestSource<'a>> for Paxos {
    fn get_source() -> RestSource<'a> {
        RestSource {
            instrument_mapping: HashMap::from([
                ("BTCEUR".to_string(), "XBTEUR".to_string()),
                ("BTCSGD".to_string(), "XBTSGD".to_string()),
                ("BTCUSD".to_string(), "XBTUSD".to_string()),
            ]),
            code: CODE,
            get_from_exchange: PaxosHandler::get_instrument,
            name: NAME,
            normalizer: |s, _| s.to_lowercase(),
            ..Default::default()
        }
    }
}
