// C.R.E.A.M. Get the money https://www.youtube.com/watch?v=or5C2jV1qRc
//
// Its time to load our wallet with some BTC.
// The way to get BTC in regtest environment is to mine blocks.
//
//  pub trait BitcoinClient {
//      fn get_dough_if_broke(&self);
//  }
//
// The idea is to generate blocks and pass the block rewards to an address
// in our loaded wallet. Make sure to:
//  - check balance (don't want to be generating blocking everytime)
//  - if no balance, generate blocks
//
//
// RESOURCES:
//
//  - https://developer.bitcoin.org/reference/rpc/index.html#generating-rpcs
//    Block generation reference
//
//  - https://developer.bitcoin.org/reference/rpc/getnewaddress.html
//    Might be needed to generate a new address
//
// While working on this, I noticed that there was an issue with generating
// blocks. My call always returned an Err variant the first time round I was
// trying to generate blocks.
//
// But luckily, the blocks get generated in the chain. So running the tests
// again works fine!!!

use bitcoincore_rpc::{Client, RpcApi};
use bitcoin_basics::BitcoinClient;

fn main() {
    let client = Client::setup();
    let wallet_name = "test_wallet";
    let wallet_info = client.load_wallet_in_node(wallet_name);

    assert_eq!(wallet_name, wallet_info.wallet_name);

    client.get_dough_if_broke();
    let balance = client.get_balance(None, None).unwrap();
    assert!(balance.to_btc() > 0f64);
}
