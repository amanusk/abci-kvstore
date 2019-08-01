use tendermint::rpc::Client;

fn main() {
    better_panic::Settings::debug()
        .most_recent_first(false)
        .lineno_suffix(true)
        .install();

    let client = Client::new(&"tcp://127.0.0.1:26657".parse().unwrap()).unwrap();

    // let block = client.block(1u64).unwrap();
    // println!("{:?}", block);

    // let query = client.abci_query(None, r#"0x00"#, None, false);
    // println!("{:?}", query);

    let tx = tendermint::abci::transaction::Transaction::new("abcd".as_bytes().to_owned());
    let query = client.broadcast_tx_sync(tx).unwrap();
    println!("{:?}", query);
}
