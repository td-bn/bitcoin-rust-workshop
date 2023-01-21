// It took me a while to get this one working.
//
// Our aim is to create a transaction that creates a UTXO
// that can only be unlocked using multisig. Then we'll
// spend that UTXO by signing with the threshold amount of
// keys.
//
//  pub trait BitcoinClient {
//
//      fn multi_sig_tx(
//          &self,
//          n: usize,
//          pubkeys: &Vec<String>,
//      ) -> (u64, u64, Txid, AddMultiSigAddressResult);
//
//      fn spend_multisig(
//          &self,
//          txid: Txid,
//          vout: u64,
//          to: &Address,
//          amount: Amount,
//          res: AddMultiSigAddressResult,
//          secret_keys: &[SecretKey],
//      );
//  }
//
//  The function `multi_sig_tx`:
//      creates a multi sig address,
//      sends some bitcoin to the created address,
//      returns the relevant details needed to spend listed below.
//
//  The function takes:
//      n, signatures threshold, and a vector of pubkeys.
//
//  It returns (vout, value, Txid, AddMultiSigAddressResult)
//      vout: index of UTXO in the tx outputs
//      value: value locked in UTXO
//      txid: txid of transaction that includes this UTXO
//      AddMultiSigAddressResult: Data related to multisig(see docs)
//
//
//  The function `spend_multisig` takes above info and spends it by creating
//  a raw transaction and signing it with the secret keys.
//
//
// RESOURCES:
//
//  - https://developer.bitcoin.org/examples/transactions.html#p2sh-multisig
//
//  - https://docs.rs/bitcoincore-rpc-json/0.16.0/bitcoincore_rpc_json/struct.AddMultiSigAddressResult.html
//    Capture result of create multi sig call
//
//  - https://docs.rs/bitcoincore-rpc/0.16.0/bitcoincore_rpc/trait.RpcApi.html#tymethod.call
//    Note the rust rpc client does not have a function that makes this call, you can implement it
//    for the client of use `call` method.
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
// 
// Extra Reading:
// - https://github.com/bitcoinbook/bitcoinbook/blob/develop/ch07.asciidoc#multisignature
// - https://github.com/bitcoinbook/bitcoinbook/blob/develop/ch06.asciidoc
//

use std::ops::Sub;

use bitcoincore_rpc::{bitcoin::Amount, Client, RpcApi};
use bitcoin_basics::BitcoinClient;
use secp256k1::{rand, Secp256k1};

fn main() {
    let client = Client::setup();
    let wallet_name = "test_wallet";
    client.load_wallet_in_node(wallet_name);

    let secp = Secp256k1::new();

    // Generate 3 keypairs
    let mut pub_keys = vec![];
    let mut secrets = vec![];
    for _ in 1..=3 {
        let (secret_key, pk) = secp.generate_keypair(&mut rand::thread_rng());
        pub_keys.push(pk.to_string());
        secrets.push(secret_key);
    }

    // 2 of 3 signers
    let signers = &[secrets[0], secrets[2]];

    // Create 2/3 multi sig address and sent a transaction to it
    let (vout, value, txid, res) = client.multi_sig_tx(2, &pub_keys);

    let to = client.get_new_address(None, None).unwrap();
    let amount = Amount::from_sat(value).sub(Amount::from_sat(100_000));

    // Spend the from multi sig address
    client.spend_multisig(txid, vout, &to, amount, res, signers);

    let bal = client.get_received_by_address(&to, None).unwrap();
    assert_eq!(amount, bal);
}
