extern crate abci;
extern crate byteorder;
extern crate env_logger;
#[macro_use]
extern crate log;

use abci::*;
use byteorder::{BigEndian, ByteOrder};
use env_logger::Env;

// Simple counter application.  Its only state is a u64 count
// We use BigEndian to serialize the data across transactions calls
struct ValueApp {
    value: String,
}

impl ValueApp {
    fn new() -> ValueApp {
        ValueApp {
            value: String::new(),
        }
    }
}

// Convert incoming tx data to the proper BigEndian size. txs.len() > 8 will return 0
fn convert_tx(bytes: &[u8]) -> String {
    String::from_utf8(bytes.to_vec()).expect("Found invalid UTF-8")
}

impl abci::Application for ValueApp {
    // Validate transactions.  Rule:  Transactions must be incremental: 1,2,3,4...
    fn check_tx(&mut self, req: &RequestCheckTx) -> ResponseCheckTx {
        // Get the Tx [u8] and convert to u64
        let c = convert_tx(req.get_tx());
        let mut resp = ResponseCheckTx::new();

        info!("Received {:?}", c);
        if c != "abcd" {
            resp.set_log(String::from("Can only send abcd"));
            return resp;
        } else {
            resp.set_code(0_u32);
            resp.set_log(String::from("Added to log"));
            resp.set_data(c.as_bytes().to_owned());
        }

        // Update state to keep state correct for next check_tx call
        self.value = c;
        resp
    }

    fn query(&mut self, _req: &RequestQuery) -> ResponseQuery {
        println!("Processing query!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
        let mut resp = ResponseQuery::new();
        resp.set_code(0);
        println!("Code is {}", resp.get_code());
        resp.set_log(String::from("Exists"));
        resp.set_index(-1);
        resp.set_key(self.value.clone().into_bytes().to_vec());
        resp.set_value(self.value.clone().into_bytes().to_vec());
        resp.set_height(1_i64);
        resp.set_codespace(String::from("Bla"));
        resp
    }

    fn deliver_tx(&mut self, req: &RequestDeliverTx) -> ResponseDeliverTx {
        // Get the Tx [u8]
        let c = convert_tx(req.get_tx());
        // Update state
        self.value = c;
        // Return default code 0 == bueno
        ResponseDeliverTx::new()
    }

    fn commit(&mut self, _req: &RequestCommit) -> ResponseCommit {
        // Create the response
        let mut resp = ResponseCommit::new();
        // Convert count to bits
        let buf = self.value.clone().into_bytes();
        // Set data so last state is included in the block
        resp.set_data(buf.to_vec());
        info!("Returning commit {:?}", resp);
        resp
    }
}

fn main() {
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    abci::run_local(ValueApp::new());
}
