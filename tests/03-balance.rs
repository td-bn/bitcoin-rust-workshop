// C.R.E.A.M. Get the money https://www.youtube.com/watch?v=or5C2jV1qRc
// 
// Its time to load our wallet with some BTC.
// The way to get BTC in regtest environment is to mine blocks.
//
//  impl BitcoinClient {
//      pub fn get_dough_if_broke(&self) {..}
//  }
//
// The idea is to generate blocks and pass the block rewards to an address
// in our loaded wallet. Make sure:
//  - check balance 
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
// blocks. My call always returned an Err variant. Not sure how to fix that. 
// But luckily, the blocks get generated in the chain. So running the tests 
// again works fine!

use bitcoincore_rpc::RpcApi;
use rust_bitcoin_workshop::*;

fn main() {
    let client = BitcoinClient::new();
    let wallet_name = "test_wallet_5";
    let wallet_info = client.load_wallet(wallet_name);

    assert_eq!(wallet_name, wallet_info.wallet_name);

    client.get_dough_if_broke();
    let balance = client.get_balance(None, None).unwrap();
    println!("Balance: {}", balance);
    assert!(balance.to_btc() > 0f64);
}

