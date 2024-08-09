// src/core/services/utxo_service.rs
use crate::models::{TweakRequest, UTXO};
use crate::storage::UtxoStore;
use crate::Result;
use std::sync::Arc;

pub struct UtxoService<S: UtxoStore + Send + Sync> {
    store: Arc<S>,
}

impl<S: UtxoStore + Send + Sync> UtxoService<S> {
    pub fn new(store: Arc<S>) -> Self {
        Self { store }
    }

    pub async fn add_utxo(&self, block_height: u64, utxo: UTXO) -> Result<()> {
        self.store.add_utxo(block_height, utxo).await
    }

    pub async fn query_utxos(&self, block_height: u64) -> Result<Vec<UTXO>> {
        self.store.query_utxos(block_height).await
    }

    pub async fn query_utxos_range(&self, request: &TweakRequest) -> Result<Vec<UTXO>> {
        let mut all_utxos = Vec::new();
        for height in request.start_height..=request.end_height {
            let utxos = self.store.query_utxos(height).await?;
            all_utxos.extend(utxos);
        }
        Ok(all_utxos)
    }
}
