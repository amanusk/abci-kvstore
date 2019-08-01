use base64::decode;
use curl::easy::Easy;
use serde::{Deserialize, Serialize};
use serde_json::{Error, Map, Value};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct AbciQueryResult {
    key: String,
    value: String,
    log: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AbciQuery {
    jsonrpc: String,
    id: String,
    result: HashMap<String, AbciQueryResult>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AbciCheckTxResult {
    data: String,
    log: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AbciCheckTx {
    jsonrpc: String,
    id: String,
    result: HashMap<String, AbciCheckTxResult>,
}

fn json_to_rust() -> Result<(), Error> {
    let mut json = Vec::new();

    let mut easy = Easy::new();
    easy.url(&format!(
        "localhost:26657/abci_query?path=\"{}\"&data=\"{}\"&prove={}",
        "", "abcd", "false"
    ))
    .unwrap();
    {
        let mut transfer = easy.transfer();
        transfer
            .write_function(|data| {
                json.extend_from_slice(data);
                Ok(data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    }

    assert_eq!(200, easy.response_code().unwrap());

    println!("{:?}", json);
    let result: AbciQuery = serde_json::from_slice(&json).unwrap();
    println!("{:?}", result);

    println!(
        "Query Result is {:?}",
        result.result.get("response").unwrap()
    );

    println!(
        "Query Value is {:?}",
        result.result.get("response").unwrap().value
    );

    println!(
        "Query Value as string {:?}",
        String::from_utf8(base64::decode(&result.result.get("response").unwrap().value).unwrap())
            .unwrap()
    );

    Ok(())
}

fn main() {
    json_to_rust();
}
