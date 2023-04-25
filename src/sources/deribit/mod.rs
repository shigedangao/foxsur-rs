use super::rest_source::RestSource;
use super::Src;
use crate::instruments::deribit::DeribitHandler;
use crate::instruments::GetInstrument;

// Constant
const CODE: &str = "drbt";
pub const NAME: &str = "deribit";

pub struct Deribit;

impl<'a> Src<RestSource<'a>> for Deribit {
    fn get_source() -> RestSource<'a> {
        RestSource {
            code: CODE,
            get_from_exchange: DeribitHandler::get_instrument,
            name: NAME,
            normalizer: |s, re| {
                if let Some(r) = re {
                    r.replace_all(s, "").to_lowercase()
                } else {
                    s.to_lowercase()
                }
            },
            regex: Some(r#"/[-_]"#),
            ..Default::default()
        }
    }
}
