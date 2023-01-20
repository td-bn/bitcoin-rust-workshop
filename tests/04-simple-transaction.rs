// Send a simple payment to an address in our own wallet 
//
// We add the following behaviour to our client.
//
//  pub trait BitcoinClient {
//      fn transfer(&self, address: &Address, amount: f64) -> Txid;
//  }
//
// We want this function to do the following things:
//  - check to see if the Amount is okay
//  - create a tx to send BTC to the given address 
//  - mine a block to include this tx 
//  - returns Txid of the transaction(might come in handy later)
// 
// RESOURCES:
//
//  1.  https://developer.bitcoin.org/reference/rpc/sendtoaddress.html
//      Send to address RPC API
//
//  2.  https://docs.rs/bitcoin/latest/bitcoin/util/amount/struct.Amount.html
//      Docs for the Amount struct 
//
// Potential issues:
//
//      - one of the issues might be that the client is not able to estimate the fee
//      for the tx, this is because the mempool is empty
//
//      - https://bitcoin.stackexchange.com/questions/102508
//

use bitcoincore_rpc::{RpcApi, Client};
use rust_bitcoin_workshop::*;

fn main() {
    let client = Client::setup();
    let wallet_name = "test_wallet_5";
    client.load_wallet_in_node(wallet_name);
    client.get_dough_if_broke();
    let address = client.get_new_address(None, None).unwrap();
    client.transfer(&address, 1f64);

    let amount = client.get_received_by_address(&address, None).unwrap();
    assert_eq!(amount.to_btc(), 1f64);
}

