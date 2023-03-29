use postgres::Client;
use anyhow::Result;
use log::info;
use std::collections::HashMap;
use postgres::row::Row;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default)]
pub struct Instrument {
    pub id: Option<i32>,
    pub base_id: Option<i32>,
    pub quote_id: Option<i32>,
    pub symbol: Option<String>,
    pub raw_symbol: Option<String>,
    pub class: Option<String>,
}

impl TryFrom<Row> for Instrument {
    type Error = anyhow::Error;

    fn try_from(value: Row) -> Result<Self> {
        Ok(Instrument {
            id: value.try_get("InstrumentId")?,
            base_id: value.try_get("BaseAssetId")?,
            quote_id: value.try_get("QuoteAssetId")?,
            symbol: value.try_get("KaikoLegacySymbol")?,
            raw_symbol: value.try_get("ExchangePairCode")?,
            class: value.try_get("Class")?,
        })
    }
}

impl Instrument {
    /// Get a list of instruments from the database based on the slug
    ///
    /// # Arguments
    ///
    /// * `handler` - &Handler
    /// * `slug` - &str
    pub fn get_instruments(
        handler: Arc<Mutex<Client>>,
        slug: &str,
    ) -> Result<HashMap<String, Instrument>> {
        let mut client = handler.lock()
            .map_err(|err| anyhow::anyhow!("Unable to acquire lock {}", err.to_string()))?;
        
        let rows = client.query(
            r#"
            SELECT
                "InstrumentId",
                "BaseAssetId",
                "QuoteAssetId",
                "KaikoLegacySymbol",
                "ExchangePairCode",
                "Class"
            FROM
                "Instruments"
            WHERE
                "ExchangeCode" = $1
        "#,
        &[&slug]
        )?;

        let mut processed_instruments = HashMap::new();
        for row in rows {
            let instrument = Instrument::try_from(row)?;
            let raw_symbol = instrument.clone().raw_symbol.unwrap_or_default();
            
            processed_instruments.insert(raw_symbol, instrument);
        }

        Ok(processed_instruments)
    }

    /// Insert a new instrument
    pub fn insert_instrument(
        self,
        handler: Arc<Mutex<Client>>,
        exch_code: String,
        normalized_symbol: String,
    ) -> Result<()> {
        // Increase the reference counting by copying the Arc
        let mut client = handler.lock()
            .map_err(|err| anyhow::anyhow!("Unable to acquire lock {}", err.to_string()))?;
        
        let id = client.execute(
            r#"
            INSERT INTO "Instruments"
            (
                "ExchangeCode",
                "ExchangePairCode",
                "KaikoLegacySymbol",
                "BaseAssetId",
                "QuoteAssetId",
                "Class",
                "TradeCount",
                "TradeCompressedSize"
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            returning "InstrumentId"
        "#,
        &[
            &exch_code,
            &self.symbol,
            &normalized_symbol,
            &self.base_id,
            &self.quote_id,
            &self.class,
            &0_i64,
            &0_i64
        ]
        )?;

        drop(client);

        info!(
            "pushed for id {:?} and symbol {:?} and base_id {:?} and quote_id {:?}",
            id, self.symbol, self.base_id, self.quote_id
        );

        Ok(())
    }
}
