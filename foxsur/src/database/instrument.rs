use anyhow::Result;
use log::info;
use postgres::row::Row;
use postgres::Client;
use psql::PostgresType;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default, PostgresType)]
pub struct DBInstrument {
    pub instrument_id: Option<i32>,
    pub base_asset_id: Option<i32>,
    pub quote_asset_id: Option<i32>,
    pub legacy_symbol: Option<String>,
    pub exchange_pair_code: Option<String>,
    pub class: Option<String>,
}

impl DBInstrument {
    /// Get a list of instruments from the database based on the slug
    ///
    /// # Arguments
    ///
    /// * `handler` - &Handler
    /// * `slug` - &str
    pub fn get_instruments(
        handler: Arc<Mutex<Client>>,
        slug: &str,
    ) -> Result<HashMap<String, DBInstrument>> {
        let mut client = handler
            .lock()
            .map_err(|err| anyhow::anyhow!("Unable to acquire lock {}", err.to_string()))?;

        let rows = client.query(
            r#"
            SELECT
                "InstrumentId" AS instrument_id,
                "BaseAssetId" AS base_asset_id,
                "QuoteAssetId" AS quote_asset_id,
                "LegacySymbol" AS legacy_symbol,
                "ExchangePairCode" AS exchange_pair_code,
                "Class"
            FROM
                "Instruments"
            WHERE
                "ExchangeCode" = $1
        "#,
            &[&slug],
        )?;

        let mut processed_instruments = HashMap::new();
        for row in rows {
            let instrument = DBInstrument::try_from(row)?;
            let raw_symbol = instrument.exchange_pair_code.to_owned().unwrap_or_default();

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
        let mut client = handler
            .lock()
            .map_err(|err| anyhow::anyhow!("Unable to acquire lock {}", err.to_string()))?;

        let id = client.execute(
            r#"
            INSERT INTO "Instruments"
            (
                "ExchangeCode",
                "ExchangePairCode",
                "LegacySymbol",
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
                &self.exchange_pair_code,
                &normalized_symbol,
                &self.base_asset_id,
                &self.quote_asset_id,
                &self.class,
                &0_i64,
                &0_i64,
            ],
        )?;

        drop(client);

        info!(
            "pushed for id {:?} and symbol {:?} and base_id {:?} and quote_id {:?}",
            id,
            self.legacy_symbol.unwrap_or_default(),
            self.base_asset_id.unwrap_or_default(),
            self.quote_asset_id.unwrap_or_default()
        );

        Ok(())
    }
}
