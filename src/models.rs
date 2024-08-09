use serde::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes};
use silentpayments::receiving::Receiver;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClientData {
    pub receiver: Receiver,
    pub b_scan: [u8; 32],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistrationRequest {
    pub version: u32,
    pub scan_pubkey: String,
    pub spend_pubkey: String,
    pub change_label: String,
    pub network: String,
    pub b_scan: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistrationResponse {
    pub client_id: String,
    pub receiving_address: String,
}

#[serde_as]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UTXO {
    #[serde_as(as = "Bytes")]
    pub txid: [u8; 32],
    pub vout: u32,
    pub amount: u64,
    pub script_pubkey: [u8; 32],
    #[serde_as(as = "Bytes")]
    pub input_tweak: [u8; 33],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanRequest {
    pub block_height: u64,
    pub client_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TweakRequest {
    pub start_height: u64,
    pub end_height: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use silentpayments::receiving::Label;
    use silentpayments::secp256k1::PublicKey;
    use silentpayments::utils::Network;
    use std::str::FromStr;

    #[test]
    fn test_utxo_serialization() {
        let utxo = UTXO {
            txid: [0; 32],
            vout: 1,
            amount: 100000,
            script_pubkey: [1; 32],
            input_tweak: [2; 33],
        };

        let serialized = serde_json::to_string(&utxo).unwrap();
        let deserialized: UTXO = serde_json::from_str(&serialized).unwrap();

        assert_eq!(utxo, deserialized);
    }

    #[test]
    fn test_client_data_serialization() {
        let receiver = Receiver::new(
            0,
            PublicKey::from_str(
                "03bbc63f12745d3b9e9d24c6cd7a1efebad0a7f469232fbecf31fba7b4f7ddeda8",
            )
            .unwrap(),
            PublicKey::from_str(
                "0381eb9a9a9ec739d527c1631b31b421566f5c2a47b4ab5b1f6a686dfb68eab716",
            )
            .unwrap(),
            Label::try_from(
                "3e9fce73d4e77a4809908e3c3a2e54ee147b9312dc5044a193d1fc85de46e3c1".to_string(),
            )
            .unwrap(),
            Network::Mainnet,
        )
        .unwrap();

        let client_data = ClientData {
            receiver,
            b_scan: [0; 32],
        };

        let serialized = serde_json::to_string(&client_data).unwrap();
        let deserialized: ClientData = serde_json::from_str(&serialized).unwrap();

        assert_eq!(client_data.b_scan, deserialized.b_scan);
        assert_eq!(
            client_data.receiver.get_receiving_address(),
            deserialized.receiver.get_receiving_address()
        );
    }
}
