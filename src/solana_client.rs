use reqwest::Client;
use serde_json::Value;
use std::env;
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};

pub async fn get_current_epoch() -> Result<u64, Box<dyn std::error::Error>> {
    let solana_rpc_url =
        env::var("SOLANA_RPC_URL").expect("Solana Rpc URL is not set in .env file");
    let client = Client::new();
    let request_body = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getEpochInfo"
    });

    let res = client
        .post(solana_rpc_url)
        .json(&request_body)
        .send()
        .await?
        .json::<Value>()
        .await?;

    Ok(res["result"]["epoch"].as_u64().unwrap())
}

pub async fn fetch_epoch_transactions(
    epoch: u64,
    transactions: Arc<Mutex<Vec<Value>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let solana_rpc_url =
        env::var("SOLANA_RPC_URL").expect("Solana Rpc URL is not set in .env file");
    loop {
        println!("Fetching transactions for epoch: {}", epoch);
        let request_body = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getSignaturesForAddress",
            "params": ["Vote111111111111111111111111111111111111111", { "limit": 10 }]
        });

        let res = client
            .post(&solana_rpc_url)
            .json(&request_body)
            .send()
            .await?
            .json::<Value>()
            .await?;

        if let Some(signatures) = res["result"].as_array() {
            for signature_entry in signatures {
                if let Some(signature) = signature_entry["signature"].as_str() {
                    // Step 2: Fetch transaction details using the signature
                    let transaction_details = fetch_transaction_details(signature).await?;

                    // Step 3: Store the transaction details in shared memory
                    let mut txs = transactions.lock().unwrap();
                    txs.push(transaction_details);
                }
            }
        }

        // Wait before the next fetch
        sleep(Duration::from_secs(10)).await;
    }
}

async fn fetch_transaction_details(signature: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let client = Client::new();
    let solana_rpc_url =
        env::var("SOLANA_RPC_URL").expect("Solana Rpc URL is not set in .env file");
    let request_body = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getTransaction",
        "params": [signature, { "encoding": "json" }]
    });

    let response = client
        .post(solana_rpc_url)
        .json(&request_body)
        .send()
        .await?
        .json::<Value>()
        .await?;

    Ok(response["result"].clone())
}
