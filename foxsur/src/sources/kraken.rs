use super::{rest_source::RestSource, Src};
use crate::instruments::{kraken::KrakenHandler, GetInstrument};

// Constant
const CODE: &str = "krkn";
pub const NAME: &str = "kraken";

pub struct Kraken;

impl Src<RestSource> for Kraken {
    fn get_source() -> RestSource {
        RestSource {
            code: CODE.to_string(),
            get_from_exchange: KrakenHandler::get_instrument,
            name: NAME.to_string(),
            normalizer: |s, _| s.to_lowercase(),
            regex: None,
            ..Default::default()
        }
    }
}
