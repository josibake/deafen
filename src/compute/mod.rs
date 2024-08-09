mod local;

pub use local::LocalCompute;

use crate::models::UTXO;
use crate::Result;
use async_trait::async_trait;
use silentpayments::receiving::Receiver;
use silentpayments::secp256k1::SecretKey;

#[async_trait]
pub trait Compute: Send + Sync {
    async fn perform_ecdh(
        &self,
        utxos: &[UTXO],
        receiver: &Receiver,
        b_scan: &SecretKey,
    ) -> Result<Vec<UTXO>>;
}
