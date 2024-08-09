use super::{ClientStore, UtxoStore};
use crate::models::{ClientData, UTXO};
use crate::{Error, Result};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct MemoryStore {
    utxos: Arc<RwLock<HashMap<u64, Vec<UTXO>>>>,
    clients: Arc<RwLock<HashMap<String, ClientData>>>,
}

impl MemoryStore {
    pub fn new() -> Self {
        Self {
            utxos: Arc::new(RwLock::new(HashMap::new())),
            clients: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl UtxoStore for MemoryStore {
    async fn add_utxo(&self, block_height: u64, utxo: UTXO) -> Result<()> {
        let mut utxos = self.utxos.write().await;
        utxos
            .entry(block_height)
            .or_insert_with(Vec::new)
            .push(utxo);
        Ok(())
    }

    async fn query_utxos(&self, block_height: u64) -> Result<Vec<UTXO>> {
        let utxos = self.utxos.read().await;
        Ok(utxos.get(&block_height).cloned().unwrap_or_default())
    }
}

#[async_trait]
impl ClientStore for MemoryStore {
    async fn store_client_data(&self, client_id: &str, client_data: ClientData) -> Result<()> {
        let mut clients = self.clients.write().await;
        clients.insert(client_id.to_string(), client_data);
        Ok(())
    }

    async fn get_client_data(&self, client_id: &str) -> Result<ClientData> {
        let clients = self.clients.read().await;
        clients.get(client_id).cloned().ok_or(Error::ClientNotFound)
    }
}
