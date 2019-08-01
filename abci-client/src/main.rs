use serde::{Deserialize, Serialize};
use std::io::{stdout, Write};

use curl::easy::Easy;

// Print a web page onto stdout
fn main() {
    let mut easy = Easy::new();
    //easy.url(&format!(
    //    "localhost:26657/broadcast_tx_commit?tx={}",
    //    "\"hello\""
    //))
    //.unwrap();
    easy.url(&format!(
        "localhost:26657/abci_query?path=\"{}\"&data=\"{}\"&prove={}",
        "", "abcd", "false"
    ))
    .unwrap();
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    })
    .unwrap();
    easy.perform().unwrap();

    println!("{}", easy.response_code().unwrap());
}
