// `bitcoincore_rpc` has a `Client` struct that allows us to interact with the 
// RPC interface exposed by the underlying client.
//
// For this test to pass, we need to write the following API.
//  struct BitcoinClient {
//      client: <rust RPC client>
//  }
//  
//  impl BitcoinClient {
//      pub fn new() -> Self {..}
//  }
//
// Since we'll be using this struct extensively, it might make sense to write 
// Deref for the struct that derfs into the underlying RPC client, so that we 
// can use the `.` operator to call methods on the struct, as if we were 
// calling methods on the underlying client.
//
// In addition, we need to start a Bitcoin node in `regtest` environment
//
// RESOURCES:
//  1.  https://developer.bitcoin.org/examples/testing.html#regtest-mode
//      Bitcoin guide to start a node in the regtest environment
//
//  2.  https://github.com/rust-bitcoin/rust-bitcoincore-rpc
//      Repository for the Rust client library. See docs for the crate for 
//      more information.
//
//  3.  https://doc.rust-lang.org/std/ops/trait.Deref.html
//      Documentation for the Deref trait.
//

use bitcoincore_rpc::RpcApi;
use rust_bitcoin_workshop::BitcoinClient;

fn main() {
    let client = BitcoinClient::new();

    let version = client.version();
    assert!(version.is_ok());
}

