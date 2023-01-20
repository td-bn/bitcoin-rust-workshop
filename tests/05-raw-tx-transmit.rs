// In this exercise we'll create a raw transaction.
// They can be useful in creating a transaction that you want to send to the
// network at a later time.
//
// We'll implement the behaviour
//
//  pub trait BitcoinClient {
//      fn transmit_raw_transaction(&self, utxo: &ListUnspentResultEntry, address: &Address);
//  }
//
// This function takes a UTXO and an address, and creates and signs a raw tx.
// It then sends that raw tx to the network to be mined. Note we'll also have to
// mine a block.
//
// RESOURCES:
//
//  - https://developer.bitcoin.org/reference/rpc/createrawtransaction.html
//  
//  - https://developer.bitcoin.org/reference/rpc/signrawtransactionwithwallet.html
//
//  - https://developer.bitcoin.org/reference/rpc/sendrawtransaction.html
//
//  - https://docs.rs/bitcoincore-rpc/0.16.0/bitcoincore_rpc/struct.Client.html#impl-RpcApi-for-Client
//
//  - https://docs.rs/bitcoin/latest/bitcoin/util/amount/struct.Amount.html
// 
// NOTE: The assertion expects that you'll provide a fee of 100_000 sats in the
// tx that you create.
//

use std::ops::Sub;

use bitcoincore_rpc::{bitcoin::Amount, RpcApi, Client};
use rust_bitcoin_workshop::*;

fn main() {
    let client = Client::setup();
    let wallet_name = "test_wallet_5";
    client.load_wallet_in_node(wallet_name);

    let utxos = client
        .list_unspent(Some(1), None, None, None, None)
        .unwrap();
    assert!(utxos.len() > 0);

    let address = client.get_new_address(None, None).unwrap();

    // TODO pass amount here so that tests is independent of implementation
    client.transmit_raw_transaction(utxos.first().unwrap(), &address);

    let bal = client.get_received_by_address(&address, None).unwrap();
    assert_eq!(
        bal,
        utxos.first().unwrap().amount.sub(Amount::from_sat(100_000))
    );
}
