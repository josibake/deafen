use super::Compute;
use crate::models::UTXO;
use crate::Result;
use async_trait::async_trait;
use rayon::prelude::*;
use silentpayments::receiving::Receiver;
use silentpayments::secp256k1::{PublicKey, Secp256k1, SecretKey, XOnlyPublicKey};
use silentpayments::utils::receiving::calculate_ecdh_shared_secret;

pub struct LocalCompute {
    secp: Secp256k1<silentpayments::secp256k1::All>,
}

impl LocalCompute {
    pub fn new() -> Self {
        LocalCompute {
            secp: Secp256k1::new(),
        }
    }
}

#[async_trait]
impl Compute for LocalCompute {
    async fn perform_ecdh(
        &self,
        utxos: &[UTXO],
        receiver: &Receiver,
        b_scan: &SecretKey,
    ) -> Result<Vec<UTXO>> {
        Ok(utxos
            .par_iter()
            .filter_map(|utxo| {
                let tweak_pubkey = PublicKey::from_slice(&utxo.input_tweak).ok()?;
                let ecdh_shared_secret = calculate_ecdh_shared_secret(&tweak_pubkey, b_scan);
                let pubkey = XOnlyPublicKey::from_slice(&utxo.script_pubkey).ok()?;
                let scan_result = receiver
                    .scan_transaction(&self.secp, &ecdh_shared_secret, vec![pubkey])
                    .ok()?;
                if !scan_result.is_empty() {
                    Some(utxo.clone())
                } else {
                    None
                }
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use silentpayments::secp256k1::{PublicKey, SecretKey};
    use silentpayments::utils::Network;
    use std::str::FromStr;

    #[tokio::test]
    async fn test_local_compute() {
        let compute = LocalCompute::new();
        let b_scan =
            SecretKey::from_str("04b2a411635c097759aacd0f005a4c82c8c92862c6fc284b80b8efebc20c3d17")
                .unwrap();
        let receiver = Receiver::new(
            0,
            PublicKey::from_str(
                "03bbc63f12745d3b9e9d24c6cd7a1efebad0a7f469232fbecf31fba7b4f7ddeda8",
            )
            .expect("Bad hex string"),
            PublicKey::from_str(
                "0381eb9a9a9ec739d527c1631b31b421566f5c2a47b4ab5b1f6a686dfb68eab716",
            )
            .expect("Bad hex string"),
            "3e9fce73d4e77a4809908e3c3a2e54ee147b9312dc5044a193d1fc85de46e3c1"
                .to_string()
                .try_into()
                .expect("bad label"),
            Network::Mainnet,
        )
        .expect("Cannot create receiver");
        let utxo = UTXO {
            txid: [0; 32],
            vout: 0,
            amount: 100000,
            script_pubkey: hex::decode(
                "596b20b0f02f9b085a801ee276ce9f21470c0d30b633372617c564a2a2fda171",
            )
            .unwrap()
            .try_into()
            .unwrap(),
            input_tweak: hex::decode(
                "020d8ec185ece237b30d2064da3700aaf42519d60ddcb0a76695b3eada2d23b319",
            )
            .unwrap()
            .try_into()
            .unwrap(),
        };

        let utxos = vec![utxo];
        let result = compute
            .perform_ecdh(&utxos, &receiver, &b_scan)
            .await
            .unwrap();

        assert_eq!(result.len(), 1);
    }
}
