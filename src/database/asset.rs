use super::Handler;
use std::collections::HashMap;

#[derive(Debug, sqlx::FromRow)]
pub struct Assets {
    id: i64,
    code: String,
}

impl Assets {
    /// Return the list of assets in the database
    ///
    /// # Arguments
    ///
    /// * `handler` - &Handler
    pub async fn get_assets(
        handler: &Handler,
    ) -> Result<HashMap<String, i64>, Box<dyn std::error::Error>> {
        let assets = sqlx::query_as::<_, Assets>("SELECT Id, Code FROM Assets")
            .fetch_all(&handler.pool)
            .await?;

        let assets_map: HashMap<String, i64> = assets.into_iter().map(|a| (a.code, a.id)).collect();

        Ok(assets_map)
    }
}
