use anyhow::Result;
use postgres::Client;
use postgres::Row;
use psql::PostgresType;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default, PostgresType)]
pub struct Assets {
    id: i32,
    code: String,
}

impl Assets {
    /// Return the list of assets in the database
    ///
    /// # Arguments
    ///
    /// * `handler` - &Handler
    pub fn get_assets(handler: Arc<Mutex<Client>>) -> Result<HashMap<String, i32>> {
        let mut client = handler
            .lock()
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
