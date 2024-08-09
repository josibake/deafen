use crate::models::{ClientData, RegistrationRequest, RegistrationResponse};
use crate::storage::ClientStore;
use crate::Result;
use silentpayments::receiving::{Label, Receiver};
use silentpayments::secp256k1::{PublicKey, SecretKey};
use silentpayments::utils::Network;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

pub struct ClientService<S: ClientStore + Send + Sync> {
    store: Arc<S>,
}

impl<S: ClientStore + Send + Sync> ClientService<S> {
    pub fn new(store: Arc<S>) -> Self {
        Self { store }
    }

    pub async fn register_client(&self, req: RegistrationRequest) -> Result<RegistrationResponse> {
        let scan_pubkey = PublicKey::from_str(&req.scan_pubkey)?;
        let spend_pubkey = PublicKey::from_str(&req.spend_pubkey)?;
        let change_label = Label::try_from(req.change_label)?;
        let network: Network = match req.network.as_str() {
            "mainnet" => Network::Mainnet,
            _ => Network::Regtest,
        };
        let b_scan = SecretKey::from_str(&req.b_scan)?;

        let receiver = Receiver::new(
            req.version,
            scan_pubkey,
            spend_pubkey,
            change_label,
            network,
        )?;

        let client_id = Uuid::new_v4().to_string();

        let client_data = ClientData {
            receiver: receiver.clone(),
            b_scan: b_scan.secret_bytes(),
        };

        self.store
            .store_client_data(&client_id, client_data)
            .await?;

        let receiving_address = receiver.get_receiving_address();

        Ok(RegistrationResponse {
            client_id,
            receiving_address,
        })
    }

    pub async fn get_client_data(&self, client_id: &str) -> Result<ClientData> {
        self.store.get_client_data(client_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::MemoryStore;

    #[tokio::test]
    async fn test_register_client() {
        let store = Arc::new(MemoryStore::new());
        let service = ClientService::new(store);

        let request = RegistrationRequest {
            version: 0,
            scan_pubkey: "03bbc63f12745d3b9e9d24c6cd7a1efebad0a7f469232fbecf31fba7b4f7ddeda8"
                .to_string(),
            spend_pubkey: "0381eb9a9a9ec739d527c1631b31b421566f5c2a47b4ab5b1f6a686dfb68eab716"
                .to_string(),
            change_label: "3e9fce73d4e77a4809908e3c3a2e54ee147b9312dc5044a193d1fc85de46e3c1"
                .to_string(),
            network: "mainnet".to_string(),
            b_scan: "04b2a411635c097759aacd0f005a4c82c8c92862c6fc284b80b8efebc20c3d17".to_string(),
        };

        let result = service.register_client(request).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(!response.client_id.is_empty());
        assert!(!response.receiving_address.is_empty());
    }
}
