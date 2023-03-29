use super::rest_source::RestSource;
use super::Src;
use crate::instruments::deribit::DeribitHandler;
use crate::instruments::GetInstrument;

// Constant
const CODE: &str = "drbt";
pub const NAME: &str = "deribit";

pub struct Deribit;

impl Src<RestSource> for Deribit {
    fn get_source() -> RestSource {
        RestSource {
            code: CODE.to_string(),
            get_from_exchange: || DeribitHandler::get_instrument(),
            name: NAME.to_string(),
            normalizer: |s, re| {
                if let Some(r) = re {
                    r.replace_all(s, "").to_lowercase()
                } else {
                    s.to_lowercase()
                }
            },
            regex: Some(r#"/[-_]"#.to_owned()),
            ..Default::default()
        }
    }
}
