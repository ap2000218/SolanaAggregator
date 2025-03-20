use std::{
    env,
    sync::{Arc, Mutex},
};
mod aggregator;
mod api;
mod models;
mod solana_client;

#[tokio::main]
async fn main() {
    env_logger::init();
    let port_string = env::var("PORT").expect("PORT is not set in .env file");
    let port: u16 = port_string.parse::<u16>().unwrap();

    // in-memory storage -> transactions
    let transactions = Arc::new(Mutex::new(Vec::new()));

    println!("Starting Solana Data Aggregator");
    // Start the data aggregator in a separate async task
    let aggregator_transactions = transactions.clone();
    tokio::spawn(async move {
        if let Err(e) = aggregator::start_aggregating(aggregator_transactions).await {
            log::error!("Data aggregation error: {}", e);
        }
    });

    println!("API Server running on: {}", port);
    let api_routes = api::get_routes(transactions.clone());
    warp::serve(api_routes).run(([127, 0, 0, 1], port)).await;
}
