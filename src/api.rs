use crate::aggregator::get_transaction_by_id;
// use core::time;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use warp::Filter;

pub fn get_routes(
    transactions: Arc<Mutex<Vec<Value>>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let get_all_transactions = warp::path("transactions")
        .and(warp::get())
        .and(with_transactions(transactions))
        .map(|transactions: Arc<Mutex<Vec<Value>>>| {
            let txs = transactions.lock().unwrap();
            let parsed_transactions: Vec<_> = txs
                .iter()
                .filter_map(|tx| extract_transaction_info(tx))
                .collect();
            let html_table = generate_html_table(&parsed_transactions);
            warp::reply::html(format!(
                "<!DOCTYPE html>
                <html>
                <head><title>Transactions From Current Epoch</title>
                <style>
                </style>
                </head>
                <body>
                <h1>Refresh the Page to see the updated transactions</h1>
                {}</body>
                </html>
                ",
                html_table
            ))
            // warp::reply::json(&parsed_transactions)
        });

    let get_transactions_by_id_route = warp::path("transactionbyid")
        .and(warp::query::<std::collections::HashMap<String, String>>())
        .and_then(handle_get_transaction_by_id);

    get_all_transactions
        .or(get_transactions_by_id_route)
        .boxed()
}

fn with_transactions(
    transactions: Arc<Mutex<Vec<Value>>>,
) -> impl Filter<Extract = (Arc<Mutex<Vec<Value>>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || transactions.clone())
}

pub fn extract_transaction_info(transaction: &Value) -> Option<Value> {
    // println!("{}", transaction);
    let sender = transaction["transaction"]["message"]["accountKeys"][0]
        .as_str()?
        .to_string();
    let receiver = transaction["transaction"]["message"]["accountKeys"][1]
        .as_str()?
        .to_string();
    let amount = transaction["meta"]["preBalances"][0].as_u64().unwrap_or(0);
    let timestamp = transaction["blockTime"].as_u64().unwrap_or(0);

    Some(serde_json::json!({
        "sender": sender,
        "receiver": receiver,
        "amount": amount,
        "timestamp": timestamp
    }))
}

fn generate_html_table(json_array: &Vec<Value>) -> String {
    let mut table = String::from(
        "
    <style>
        table {
            border-collapse: collapse;
            max-width: 100vw;
            margin: 20px 0;
            font-size: 1.1em;
            font-family: Arial, sans-serif;
            background-color: #f4f4f9;
            text-align: left;
        }
        th, td {
            border: 1px solid #dddddd;
            padding: 12px 15px;
        }
        th {
            background-color: #009879;
            color: white;
        }
        tr:nth-child(even) {
            background-color: #f2f2f2;
        }
    </style>
    <table border=\"1\"><tr><th>No</th>",
    );

    if let Some(first_object) = json_array.get(0) {
        if let Value::Object(map) = first_object {
            for key in map.keys() {
                table.push_str(&format!("<th>{}</th>", key));
            }
            table.push_str("</tr>");
        }
    }

    let mut table_index: u16 = 0;
    for obj in json_array {
        table_index += 1;
        if let Value::Object(map) = obj {
            table.push_str(&format!("<tr><td>{}</td>", table_index));
            for value in map.values() {
                table.push_str(&format!("<td>{}</td>", value));
            }
            table.push_str("</tr>");
        }
    }
    table.push_str("</table>");
    table
}

async fn handle_get_transaction_by_id(
    params: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(id) = params.get("id") {
        match get_transaction_by_id(id).await {
            Ok(transaction) => Ok(warp::reply::json(&transaction)),
            Err(error_message) => Ok(warp::reply::json(
                &serde_json::json!({ "error": error_message }),
            )),
        }
    } else {
        Ok(warp::reply::json(
            &serde_json::json!({ "error": "Missing `id` parameter" }),
        ))
    }
}
