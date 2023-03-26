use super::Handler;
use futures_util::TryStreamExt;
use std::collections::HashMap;

#[derive(Debug, sqlx::FromRow, Clone, Default)]
pub struct Instrument {
    pub id: Option<i64>,
    pub base_id: Option<i64>,
    pub quote_id: Option<i64>,
    pub symbol: Option<String>,
    pub raw_symbol: Option<String>,
    pub class: Option<String>,
}

impl Instrument {
    /// Get a list of instruments from the database based on the slug
    ///
    /// # Arguments
    ///
    /// * `handler` - &Handler
    /// * `slug` - &str
    pub async fn get_instruments(
        handler: &Handler,
        slug: &str,
    ) -> Result<HashMap<String, Instrument>, Box<dyn std::error::Error>> {
        let mut query = sqlx::query_as::<_, Instrument>(
            r#"
            SELECT
                InstrumentId,
                BaseAssetId,
                QuoteAssetId,
                KaikoLegacySymbol,
                ExchangePairCode
            FROM
                Instruments
            WHERE
                ExchangeCode = ?
        "#,
        )
        .bind(slug)
        .fetch(&handler.pool);

        let mut instruments = HashMap::new();
        while let Ok(instrument) = query.try_next().await {
            if let Some(inst) = instrument {
                let raw_symbol = inst.to_owned().raw_symbol.unwrap_or_default();
                instruments.insert(raw_symbol, inst);
            }
        }

        Ok(instruments)
    }

    /// Insert a new instrument
    pub async fn insert_instrument(
        self,
        handler: &Handler,
        exch_code: &str,
        normalized_symbol: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // @TODO very dirty to be improve for the final presentation, just want to have the same code as the foxsur go for test purposes...
        let _: (i64,) = sqlx::query_as(
            r#"
            INSERT INTO Instruments
            (
                ExchangeCode,
                ExchangePairCode,
                KaikoLegacySymbol,
                BaseAssetId,
                QuoteAssetId,
                Class,
                TradeCount,
                TradeCompressedSize
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            returning id
        "#,
        )
        .bind(exch_code)
        .bind(self.symbol.clone().unwrap_or_default())
        .bind(normalized_symbol)
        .bind(self.base_id.unwrap_or_default())
        .bind(self.quote_id.unwrap_or_default())
        .bind(self.class.clone().unwrap_or_default())
        .bind(0)
        .bind(0)
        .fetch_one(&handler.pool)
        .await?;

        Ok(())
    }
}
