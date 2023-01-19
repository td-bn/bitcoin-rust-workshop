// Whew! Finally multi sig.
// It took me a while to get this one working myself.
//
// Our aim is to create a transaction that creates a UTXO
// that can only be unlocked using multisig. Then we'll
// spend that UTXO by signing with the threshold amount of 
// keys.
//
//  struct BitcoinClient {
//
//      This function creates a multi sig address, and sends 
//      some bitcoin to the created address. It returns the relevant
//      details needed to spend listed below.
//
//      The function take: n, signatures threshold, and a vector
//      of pubkeys.
//
//      It returns (vout, value, Txid, AddMultiSigAddressResult)
//          vout: index of UTXO in the tx outputs
//          value: value locked in UTXO
//          txid: txid of transaction that includes this UTXO
//          AddMultiSigAddressResult: Data related to multisig(see docs) 
//      pub fn multi_sig_tx(
//          &self,
//          n: usize,
//          pubkeys: &Vec<String>,
//      ) -> (u64, u64, Txid, AddMultiSigAddressResult) {..}
//
//
//      The next function takes this info and spends in by creating 
//      a raw transaction and signing it with the secret keys.
//
//      pub fn spend_multisig(
//          &self,
//          txid: Txid,
//          vout: u64,
//          to: &Address,
//          amount: Amount,
//          res: AddMultiSigAddressResult,
//          secret_keys: &[SecretKey],
//      ) {..}
//  }
//
// RESOURCES:
//
//  - https://developer.bitcoin.org/examples/transactions.html#p2sh-multisig
//
//  - https://docs.rs/bitcoincore-rpc-json/0.16.0/bitcoincore_rpc_json/struct.AddMultiSigAddressResult.html
//    Capture result of create multi sig call 
//  
//  - https://docs.rs/bitcoincore-rpc/0.16.0/bitcoincore_rpc/trait.RpcApi.html#tymethod.call
//    Call method for the client for RPCs not covered by the Rust library
//
//  - https://docs.rs/bitcoincore-rpc-json/0.16.0/bitcoincore_rpc_json/struct.SignRawTransactionInput.html
//    For creating input required for signing
//  
//  - https://developer.bitcoin.org/reference/rpc/createmultisig.html
//  - https://developer.bitcoin.org/reference/rpc/getrawtransaction.html
//  - https://developer.bitcoin.org/reference/rpc/signrawtransactionwithkey.html
//  - https://developer.bitcoin.org/reference/rpc/createrawtransaction.html
//  - https://developer.bitcoin.org/reference/rpc/sendrawtransaction.html
//  - https://docs.rs/bitcoin/latest/bitcoin/util/amount/struct.Amount.html
//
use std::ops::Sub;

use bitcoincore_rpc::{bitcoin::Amount, RpcApi};
use secp256k1::{rand, KeyPair, Secp256k1};
use rust_bitcoin_workshop::*;

fn main() {
    let client = BitcoinClient::new();
    let wallet_name = "test_wallet_5";
    client.load_wallet(wallet_name);

    let secp = Secp256k1::new();

    let keypairs: Vec<_> = (1..=3)
        .into_iter()
        .map(|_| {
            let (secret_key, _) = secp.generate_keypair(&mut rand::thread_rng());
            KeyPair::from_secret_key(&secp, &secret_key)
        })
        .collect();
    let pub_keys = keypairs
        .iter()
        .map(|k| k.public_key().to_string())
        .collect();

    let secret_keys = &[keypairs[0].secret_key(), keypairs[2].secret_key()];
    let (vout, value, txid, res) = client.multi_sig_tx(2, &pub_keys);

    let to = client.get_new_address(None, None).unwrap();
    let amount = Amount::from_sat(value).sub(Amount::from_sat(100_000));
    client.spend_multisig(txid, vout, &to, amount, res, secret_keys);

    let bal = client.get_received_by_address(&to, None).unwrap();
    assert_eq!(amount, bal);
}
