// Spending from a multi sig script
//
// Spend from a multi sig script to pass the test.
//
//  impl MiniscriptClient for Client {
//
//      // We could do without this, but adding this would remove duplication.
//      //
//      // This method takes in a script_pubkey and a txid
//      // It checks the outputs in the Tx and returns details for the first match
//      // found for a given script_pubkey
//      //
//      // Used to get UTXO details to be used as an input to the raw tx we are constructing
//      fn utxo_details(&self, script_pubkey: Script, txid: Txid) -> Result<(OutPoint, TxOut), ()>;
//
//      // Spends from a given multi sig output desc
//      fn spend_multi(
//          &self,
//          txid: Txid,
//          desc: Descriptor<DefiniteDescriptorKey>,
//          out_txout: TxOut,
//          secrets: &[SecretKey],
//          pubkeys: &[PublicKey],
//      );
//  }
//  
// RESOURCES:
//
//  - https://github.com/bitcoin/bitcoin/blob/master/doc/descriptors.md
// 
//  - https://docs.rs/bitcoincore-rpc/0.16.0/bitcoincore_rpc/trait.RpcApi.html#method.get_transaction
//
//  - https://docs.rs/miniscript/latest/miniscript/index.html
//
//  - https://docs.rs/miniscript/latest/miniscript/psbt/trait.PsbtExt.html#tymethod.finalize_mut
//
//  - https://docs.rs/bitcoin/latest/bitcoin/util/psbt/struct.PartiallySignedTransaction.html
//
//  - https://docs.rs/miniscript/latest/miniscript/psbt/trait.PsbtInputExt.html#tymethod.update_with_descriptor_unchecked
//
//  - https://github.com/rust-bitcoin/rust-miniscript/blob/master/tests/test_desc.rs
//

use bitcoincore_rpc::{
    bitcoin::{self as bitcoin, Amount, TxOut},
    Client, RpcApi,
};
use miniscript_workshop::MiniscriptClient;
use secp256k1::{rand, Secp256k1};

fn main() {
    let client = Client::configure_client();

    // Generate keypairs
    let secp = Secp256k1::new();
    let mut pubkeys = vec![];
    let mut secrets = vec![];
    for _ in 0..3 {
        let (secret, pub_key) = secp.generate_keypair(&mut rand::thread_rng());
        pubkeys.push(pub_key);
        secrets.push(secret);
    }

    // Create descriptors
    let desc = client.multisig_desc(2, &pubkeys);
    let address = desc.address(bitcoin::Network::Regtest).unwrap();

    // Send some bitcoin to descriptor
    let txid = client
        .send_to_address(
            &address,
            bitcoin::Amount::ONE_BTC,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();

    // Mine tx
    client
        .generate_to_address(1, &client.get_new_address(None, None).unwrap())
        .unwrap();

    // Create receiving address to assert against
    let recv_addr = client.get_new_address(None, None).unwrap();
    let value = 99_980_000;
    let txout = TxOut {
        value,
        script_pubkey: recv_addr.script_pubkey(),
    };

    // Act
    client.spend_multi(txid, desc, txout, &secrets, &pubkeys);

    // Assert
    let bal = client.get_received_by_address(&recv_addr, Some(1)).unwrap();
    assert_eq!(bal, Amount::from_sat(value));
}
