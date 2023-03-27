use super::Handler;
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, sqlx::FromRow, Clone, Default)]
pub struct Instrument {
    #[sqlx(rename = "InstrumentId")]
    pub id: Option<i32>,
    #[sqlx(rename = "BaseAssetId")]
    pub base_id: Option<i32>,
    #[sqlx(rename = "QuoteAssetId")]
    pub quote_id: Option<i32>,
    #[sqlx(rename = "KaikoLegacySymbol")]
    pub symbol: Option<String>,
    #[sqlx(rename = "ExchangePairCode")]
    pub raw_symbol: Option<String>,
    #[sqlx(rename = "Class")]
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
        let instruments = sqlx::query_as::<_, Instrument>(
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
        )
        .bind(slug)
        .fetch_all(&handler.pool)
        .await?;

        let processed_instruments: HashMap<String, Instrument> = instruments
            .into_iter()
            .map(|i| {
                let raw_symbol = i.clone().raw_symbol.unwrap_or_default();
                (raw_symbol, i)
            })
            .collect();

        Ok(processed_instruments)
    }

    /// Insert a new instrument
    pub async fn insert_instrument(
        self,
        handler: &Handler,
        exch_code: &str,
        normalized_symbol: String,
    ) -> Result<()> {
        let id: (i32,) = sqlx::query_as(
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

        println!("pushed for {:?}", id);

        Ok(())
    }
}
