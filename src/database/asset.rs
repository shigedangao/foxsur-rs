use super::Handler;
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, sqlx::FromRow)]
pub struct Assets {
    #[sqlx(rename = "Id")]
    id: i32,
    #[sqlx(rename = "Code")]
    code: String,
}

impl Assets {
    /// Return the list of assets in the database
    ///
    /// # Arguments
    ///
    /// * `handler` - &Handler
    pub async fn get_assets(handler: &Handler) -> Result<HashMap<String, i32>> {
        let assets = sqlx::query_as::<_, Assets>(r#"SELECT "Id", "Code" FROM "Assets""#)
            .fetch_all(&handler.pool)
            .await?;

        let assets_map: HashMap<String, i32> = assets.into_iter().map(|a| (a.code, a.id)).collect();

        Ok(assets_map)
    }
}
