use std::sync::{Arc, Mutex};
use postgres::Client;
use anyhow::Result;
use postgres::Row;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct Assets {
    id: i32,
    code: String,
}

impl TryFrom<Row> for Assets {
    type Error = anyhow::Error;

    fn try_from(value: Row) -> Result<Self> {
        Ok(Assets {
            id: value.try_get("Id")?,
            code: value.try_get("Code")?,
        })
    }
}

impl Assets {
    /// Return the list of assets in the database
    ///
    /// # Arguments
    ///
    /// * `handler` - &Handler
    pub fn get_assets(handler: Arc<Mutex<Client>>) -> Result<HashMap<String, i32>> {
        let mut client = handler.lock()
            .map_err(|err| anyhow::anyhow!("Unable to acquire lock {}", err.to_string()))?;
        
        let rows = client.query(r#"SELECT "Id", "Code" FROM "Assets""#, &[])?;
        
        let mut assets = Vec::new();
        for row in rows {
            let asset: Assets = row.try_into()?;
            assets.push(asset);
        }

        let assets_map: HashMap<String, i32> = assets.into_iter().map(|a| (a.code, a.id)).collect();

        Ok(assets_map)
    }
}
