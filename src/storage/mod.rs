mod mdbx;
mod memory;

pub use mdbx::MdbxDatabase;
pub use memory::MemoryStore;

use crate::models::{ClientData, UTXO};
use crate::Result;
use async_trait::async_trait;

#[async_trait]
pub trait UtxoStore: Send + Sync {
    async fn add_utxo(&self, block_height: u64, utxo: UTXO) -> Result<()>;
    async fn query_utxos(&self, block_height: u64) -> Result<Vec<UTXO>>;
}

#[async_trait]
pub trait ClientStore: Send + Sync {
    async fn store_client_data(&self, client_id: &str, client_data: ClientData) -> Result<()>;
    async fn get_client_data(&self, client_id: &str) -> Result<ClientData>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use silentpayments::receiving::Receiver;
    use silentpayments::secp256k1::PublicKey;
    use silentpayments::utils::Network;
    use std::str::FromStr;
    use tempfile::tempdir;

    // Define a trait that both storage backends implement
    trait TestStorage: ClientStore + UtxoStore {
        fn new_for_test() -> Self;
    }

    impl TestStorage for MemoryStore {
        fn new_for_test() -> Self {
            Self::new()
        }
    }

    impl TestStorage for MdbxDatabase {
        fn new_for_test() -> Self {
            let temp_dir = tempdir().unwrap();
            Self::new(temp_dir.path().to_path_buf()).unwrap()
        }
    }

    async fn test_storage_implementation<S: TestStorage + ClientStore + UtxoStore>() {
        let store = S::new_for_test();

        // Test UTXO storage
        let utxo = UTXO {
            txid: [0; 32],
            vout: 1,
            amount: 100000,
            script_pubkey: [1; 32],
            input_tweak: [2; 33],
        };
        store.add_utxo(1, utxo.clone()).await.unwrap();
        let retrieved_utxos = store.query_utxos(1).await.unwrap();
        assert_eq!(retrieved_utxos.len(), 1);
        assert_eq!(retrieved_utxos[0], utxo);

        // Test client data storage
        let client_data = ClientData {
            receiver: Receiver::new(
                0,
                PublicKey::from_str(
                    "03bbc63f12745d3b9e9d24c6cd7a1efebad0a7f469232fbecf31fba7b4f7ddeda8",
                )
                .expect("Bad hex string"),
                PublicKey::from_str(
                    "0315bb61abed8d5b7b91eee3b4837fe6300d72dfa0a5a0a7d979ac87b81454ae4e",
                )
                .expect("Bad hex string"),
                "3e9fce73d4e77a4809908e3c3a2e54ee147b9312dc5044a193d1fc85de46e3c1"
                    .to_string()
                    .try_into()
                    .expect("bad label"),
                Network::Mainnet,
            )
            .expect("Cannot create receiver"),
            b_scan: [0; 32],
        };
        store
            .store_client_data("test_client", client_data.clone())
            .await
            .unwrap();
        let retrieved_client_data = store.get_client_data("test_client").await.unwrap();
        assert_eq!(retrieved_client_data.b_scan, client_data.b_scan);
    }

    #[tokio::test]
    async fn test_memory_store() {
        test_storage_implementation::<MemoryStore>().await;
    }

    #[tokio::test]
    async fn test_mdbx_database() {
        test_storage_implementation::<MdbxDatabase>().await;
    }
}
