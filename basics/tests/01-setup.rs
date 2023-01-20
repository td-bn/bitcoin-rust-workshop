// `bitcoincore_rpc` has a `Client` struct that allows us to interact with the
// RPC interface exposed by the underlying client.
//
// For this test to pass, we need to write an implementation of the following
// trait for the Client stuct.
//
//  pub trait BitcoinClient {
//      fn setup() -> Client;
//  }
//
//  impl BitcoinClient for Client {
//      fn setup() -> Client {
//          ...
//      }
//  }
//
// We need to start a Bitcoin node in `regtest` environment, before we can
// make progress.
//
// RESOURCES:
//
//  1.  https://developer.bitcoin.org/examples/testing.html#regtest-mode
//      Bitcoin guide to start a node in the regtest environment
//
//  2.  https://github.com/rust-bitcoin/rust-bitcoincore-rpc
//      Repository for the Rust client library. See docs for the crate for
//      more information.
//

use bitcoincore_rpc::{Client, RpcApi};
use bitcoin_basics::BitcoinClient;

fn main() {
    let client = Client::setup();

    let version = client.version();
    assert!(version.is_ok());
}
