use crate::solana_client::{fetch_epoch_transactions, get_current_epoch};
use reqwest::Client;
use serde_json::Value;
use std::sync::{Arc, Mutex};

const SOLANA_RPC_URL: &str =
    "https://mainnet.helius-rpc.com/?api-key=b4a8d3b2-c967-40de-badc-af6a2051d52e";

pub async fn start_aggregating(
    transactions: Arc<Mutex<Vec<Value>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing data aggregator...");

    // Fetch the current epoch
    let current_epoch = get_current_epoch().await?;
    println!("Starting aggregation from epoch: {}", current_epoch);

    // Fetch and store transactions continuously
    fetch_epoch_transactions(current_epoch, transactions).await?;

    Ok(())
}

pub async fn get_transaction_by_id(id: &str) -> Result<Value, String> {
    let client = Client::new();
    println!("Fetching transaction with ID: {}", id);

    // request body here
    let request_body = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getTransaction",
        "params": [id]
    });

    // Send the request and handle errors
    let res = match client.post(SOLANA_RPC_URL).json(&request_body).send().await {
        Ok(response) => response,
        Err(err) => return Err(format!("Failed to send request: {}", err)),
    };

    // Parse the response and handle errors
    let transaction: Value = match res.json::<Value>().await {
        Ok(json) => json,
        Err(err) => return Err(format!("Failed to parse response: {}", err)),
    };

    // Return the transaction details
    Ok(transaction)
}
