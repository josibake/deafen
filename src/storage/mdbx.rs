use crate::models::{ClientData, UTXO};
use crate::storage::{ClientStore, UtxoStore};
use crate::{Error, Result};
use async_trait::async_trait;
use libmdbx::orm::{
    table, table_info, Database as OrmDatabase, DatabaseChart, Decodable, Encodable,
};
use once_cell::sync::Lazy;
use std::path::PathBuf;
use std::sync::Arc;

impl Encodable for UTXO {
    type Encoded = Vec<u8>;

    fn encode(self) -> Self::Encoded {
        bincode::serialize(&self).unwrap()
    }
}

impl Decodable for UTXO {
    fn decode(v: &[u8]) -> std::result::Result<Self, anyhow::Error> {
        Ok(bincode::deserialize(v)?)
    }
}

impl Encodable for ClientData {
    type Encoded = Vec<u8>;

    fn encode(self) -> Self::Encoded {
        bincode::serialize(&self).unwrap()
    }
}

impl Decodable for ClientData {
    fn decode(v: &[u8]) -> std::result::Result<Self, anyhow::Error> {
        Ok(bincode::deserialize(v)?)
    }
}

table!(
    /// Table for UTXOs.
    ( UTXOs ) u64 => UTXO
);

table!(
    /// Table for Client keys.
    ( Clients ) String => ClientData
);

static TABLES: Lazy<DatabaseChart> = Lazy::new(|| {
    [table_info!(UTXOs), table_info!(Clients)]
        .into_iter()
        .collect()
});

pub struct MdbxDatabase {
    db: Arc<OrmDatabase>,
}

impl MdbxDatabase {
    pub fn new(path: PathBuf) -> Result<Self> {
        let db = Arc::new(OrmDatabase::create(Some(path), &TABLES)?);
        Ok(MdbxDatabase { db })
    }
}

#[async_trait]
impl UtxoStore for MdbxDatabase {
    async fn add_utxo(&self, block_height: u64, utxo: UTXO) -> Result<()> {
        let tx = self.db.begin_readwrite()?;
        tx.upsert::<UTXOs>(block_height, utxo)?;
        tx.commit()?;
        Ok(())
    }

    async fn query_utxos(&self, block_height: u64) -> Result<Vec<UTXO>> {
        let tx = self.db.begin_read()?;
        let cursor = tx.cursor::<UTXOs>()?;
        Ok(cursor
            .walk(Some(block_height))
            .map(|result| Ok(result?.1))
            .collect::<Result<Vec<UTXO>>>()?)
    }
}

#[async_trait]
impl ClientStore for MdbxDatabase {
    async fn store_client_data(&self, client_id: &str, client_data: ClientData) -> Result<()> {
        let tx = self.db.begin_readwrite()?;
        let mut cursor = tx.cursor::<Clients>()?;
        cursor.upsert(client_id.to_string(), client_data)?;
        tx.commit()?;
        Ok(())
    }

    async fn get_client_data(&self, client_id: &str) -> Result<ClientData> {
        let tx = self.db.begin_read()?;
        let client_data: ClientData = tx
            .get::<Clients>(client_id.to_string())?
            .ok_or(Error::ClientNotFound)?;
        Ok(client_data)
    }
}
