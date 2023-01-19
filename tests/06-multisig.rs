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
