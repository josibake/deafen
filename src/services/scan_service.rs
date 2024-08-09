// src/core/services/scan_service.rs
use crate::compute::Compute;
use crate::models::{ScanRequest, TweakRequest, UTXO};
use crate::services::{ClientService, UtxoService};
use crate::storage::{ClientStore, UtxoStore};
use crate::Result;
use silentpayments::secp256k1::SecretKey;
use std::sync::Arc;

pub struct ScanService<S: UtxoStore + ClientStore + Send + Sync, C: Compute> {
    utxo_service: Arc<UtxoService<S>>,
    client_service: Arc<ClientService<S>>,
    compute_service: Arc<C>,
}

impl<S: UtxoStore + ClientStore + Send + Sync, C: Compute> ScanService<S, C> {
    pub fn new(
        utxo_service: Arc<UtxoService<S>>,
        client_service: Arc<ClientService<S>>,
        compute_service: Arc<C>,
    ) -> Self {
        Self {
            utxo_service,
            client_service,
            compute_service,
        }
    }

    pub async fn scan_utxos(&self, request: ScanRequest) -> Result<Vec<UTXO>> {
        let utxos = self.utxo_service.query_utxos(request.block_height).await?;
        let client_data = self
            .client_service
            .get_client_data(&request.client_id)
            .await?;
        let b_scan = SecretKey::from_slice(&client_data.b_scan)?;

        self.compute_service
            .perform_ecdh(&utxos, &client_data.receiver, &b_scan)
            .await
    }

    pub async fn get_tweaks(&self, request: TweakRequest) -> Result<Vec<UTXO>> {
        self.utxo_service.query_utxos_range(&request).await
    }
}
