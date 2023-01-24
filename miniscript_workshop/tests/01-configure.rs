// Before we get started with descriptors and miniscript magic
// it would be a good idea to make sure we write a method to 
// the client. 
//
// We want the client to connect to the node, have a loaded 
// wallet as well as have some premined BTC. See exercises 1-3 
// of bitcoin basics.
// 
//  impl MiniscriptClient {
//      fn configure_client() -> Client;
//  }
//
//  Returns a RPC client

use bitcoincore_rpc::{Client, RpcApi};
use miniscript_workshop::MiniscriptClient;

fn main() {
    let client = Client::configure_client();

    // Assert
    let bal = client.get_balance(Some(1), None).unwrap();
    assert!(bal.to_btc() > 0f64);
}

