use deafen::models::{RegistrationRequest, RegistrationResponse, ScanRequest, UTXO};
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let base_url = "http://localhost:3030";

    // Register a new client
    // spk: 596b20b0f02f9b085a801ee276ce9f21470c0d30b633372617c564a2a2fda171
    // tweak: 020d8ec185ece237b30d2064da3700aaf42519d60ddcb0a76695b3eada2d23b319
    let registration_request = RegistrationRequest {
        version: 0,
        scan_pubkey: "03bbc63f12745d3b9e9d24c6cd7a1efebad0a7f469232fbecf31fba7b4f7ddeda8"
            .to_string(),
        spend_pubkey: "0315bb61abed8d5b7b91eee3b4837fe6300d72dfa0a5a0a7d979ac87b81454ae4e"
            .to_string(),
        change_label: "3e9fce73d4e77a4809908e3c3a2e54ee147b9312dc5044a193d1fc85de46e3c1"
            .to_string(),
        network: "testnet".to_string(),
        b_scan: "04b2a411635c097759aacd0f005a4c82c8c92862c6fc284b80b8efebc20c3d17".to_string(),
    };

    let registration_response: RegistrationResponse = client
        .post(&format!("{}/register", base_url))
        .json(&registration_request)
        .send()
        .await?
        .json()
        .await?;

    println!("Registration successful:");
    println!("Client ID: {}", registration_response.client_id);
    println!(
        "Receiving Address: {}",
        registration_response.receiving_address
    );

    // Query UTXOs
    let query_request = ScanRequest {
        block_height: 1,
        client_id: registration_response.client_id,
    };

    let utxos: Vec<UTXO> = client
        .post(&format!("{}/query", base_url))
        .json(&query_request)
        .send()
        .await?
        .json()
        .await?;

    println!("\nQuery results:");
    if utxos.is_empty() {
        println!("No UTXOs found.");
    } else {
        for (i, utxo) in utxos.iter().enumerate() {
            println!("UTXO {}:", i + 1);
            println!("  TXID: {:?}", utxo.txid);
            println!("  VOUT: {}", utxo.vout);
            println!("  Amount: {}", utxo.amount);
            println!("  Script Pubkey: {:?}", utxo.script_pubkey);
            println!("  Input Tweak: {:?}", utxo.input_tweak);
        }
    }

    Ok(())
}
