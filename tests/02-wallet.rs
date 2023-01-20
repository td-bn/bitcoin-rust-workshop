// We now have a client that we can use to make RPCs. In this exercise we'll 
// create a new wallet and load it. Or load one if it already exists.
//
// The aim is add the following behaviour to our client.
//
//  pub trait BitcoinClient {
//
//      // Returns a client with a custom RPC path
//      fn with_custom_path(path: &str) -> Self {..}
//
//      // Loads a wallet in the node
//      fn load_wallet_in_node(&self, wallet_name: &str) -> GetWalletInfoResult {..}     
//  }
//
// The `with_custom_path` method takes a string and returns an RPC client 
// with the given path appended. See RESOURCES#2 for why it might be
// useful. If you can get away without doing this. Raise a PR, and help 
// improve.
//
// The `load_wallet_in_node` method should at least take care of
//  - creating a wallet if it doesn't exist 
//  - loading a wallet if it does exist
//  - returning wallet info using the getwalletinfo RPC
//  - handle relevant errors
//
// RESOURCES:
//
//  1.  https://docs.rs/bitcoincore-rpc/0.16.0/bitcoincore_rpc/trait.RpcApi.html
//      RPC API trait that is implemented for the client. See RPC methods that 
//      might help to write the function.
//
//  2.  https://stackoverflow.com/questions/64324893
//      For loading a wallet with a given name.
//

use bitcoincore_rpc::Client;
use rust_bitcoin_workshop::*;

fn main() {
    let client = Client::setup();
    let wallet_name = "test_wallet_5";
    let wallet_info = client.load_wallet_in_node(wallet_name);

    assert_eq!(wallet_name, wallet_info.wallet_name);
}

