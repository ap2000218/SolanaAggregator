# Solana Data Aggregator

This project is a Rust-based application that collects and processes data from the Solana blockchain. It is capable of retrieving transaction and account data for the current epoch, processing the data, and exposing it through a RESTful API.

## Features

- Retrieve transactions and account data from the Solana blockchain
- Process and structure data for analysis
- Expose data through a RESTful API
- Query transaction details by ID

## Architecture

- **API Layer**: Built using `warp` to handle RESTful requests
- **Data Aggregation**: Retrieves and processes blockchain data
- **Solana Client**: Interacts with Solana's blockchain API

## Requirements

- Install Rust
- Set up a Solana node on Devnet or Testnet via [Helius]("https://mainnet.helius-rpc.com/?<YOUR_API_KEY_HERE>";
) or other providers
(You can change this url according to your API key in .env file)

## Usage

1. Run the application:
    cargo run

2. Access the API:
Open your browser and hit the following url to 
Get whole transactions sine the command executed: 
http://127.0.0.1:3030/transactions
(For now you should refresh the page to see the updated result of transactions)

Get transaction by ID: http://localhost:3030/transactionbyid?id=<transaction_id>
Get the test transaction id from .env file.
(For now you can only see the Json-structured Data of a transaction)


## Further Develop

1. Scalability:
    We can use caching systems like Redis or Memcached to reduce the load on the database and speed up response times
    ( For now there is only in-memory Data structure, db integration for future developing)

2. Security:
    We can add security features to our API server in main.rs using TLS encryption (secure sockets) implementing tokio-rustls crate 
