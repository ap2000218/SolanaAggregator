use mockall::*;
use serde_json::Value;
use std::sync::{Arc, Mutex};

#[automock]
pub trait Aggregation {
    fn get_current_epoch(&self) -> Result<u64, Box<dyn std::error::Error>>;
    fn fetch_epoch_transactions(
        &self,
        epoch: u64,
        transactions: Arc<Mutex<Vec<Value>>>,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

pub async fn start_aggregating(
    transactions: Arc<Mutex<Vec<Value>>>,
    aggregator: Arc<dyn Aggregation>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing data aggregator...");

    let current_epoch = aggregator.get_current_epoch()?; // Use the trait method
    println!("Starting aggregation from epoch: {}", current_epoch);

    aggregator.fetch_epoch_transactions(current_epoch, transactions)?;

    Ok(())
}

#[tokio::test]
async fn test_start_aggregating() {
    let transactions = Arc::new(Mutex::new(Vec::new()));
    let mut mock_aggregator = MockAggregation::new();

    // Mock `get_current_epoch`
    mock_aggregator
        .expect_get_current_epoch()
        .returning(|| Ok(42));

    // Mock `fetch_epoch_transactions`
    mock_aggregator
        .expect_fetch_epoch_transactions()
        .returning(|_, transactions| {
            let mut tx = transactions.lock().unwrap();
            tx.push(serde_json::json!({ "epoch": 42, "data": "test" }));
            Ok(())
        });

    // Call the function
    let result = start_aggregating(transactions.clone(), Arc::new(mock_aggregator)).await;

    // Assert success
    assert!(result.is_ok());

    // Assert that transactions were updated
    let tx = transactions.lock().unwrap();
    assert_eq!(tx.len(), 1);
    assert_eq!(tx[0]["epoch"], 42);
}
