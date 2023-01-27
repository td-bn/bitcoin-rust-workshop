// Now that we have a client set up with a wallet and some BTC,
// lets use descriptors to send a P2PKH transaction.
//
// For this exercise we'll write a functions that do the following things:
//  - Creates a partially signed bitcoin transaction(PSBT)
//  - Adds inputs and outputs to the PSBT
//  - signs, finalizes the PSBT, and creates a raw tx
//  - mines a block
//
//
//  impl MiniscriptClient {
//     // This function sends a raw tx and mines a block
//     fn sendrawtx(&self, tx: Transaction);
//
//
//     // The function will take a TxOut that will contain the output of the raw
//     // transaction.
//     fn pkh(
//         &self,
//         txid: Txid,
//         desc: Descriptor<DefiniteDescriptorKey>,
//         txout: TxOut,
//         secret_key: secp256k1::SecretKey,
//         pub_key: secp256k1::PublicKey
//     );
//      Use of args:
//       txid: get deatils for UTXO
//       desc: for using with PSBT extentions provided by miniscript 
//       txout: txout for the raw tx
//       secret_key: for signing
//       pub_key: for adding partial sig
//  }
//
// It might make sense to create some helper functions that will come in handy later:
//
//  // Creates a new PSBT
//  - fn psbt_new() -> PartiallySignedTransaction {..}
//
//  // Updates PSBT with spending info
//  - fn update_psbt(
//      psbt: &mut PartiallySignedTransaction,
//      outp: OutPoint, // OutPoint for tx input
//      output: TxOut,  // Txout to create
//      desc: &Descriptor<DefiniteDescriptorKey>,
//      txout: TxOut // Txout to spend  
//    ) {..}
//
//  // Signs and finializes the PBST with the given signers
//  - fn sign_and_finalize(
//        psbt: &mut PartiallySignedTransaction,
//        secrets: &[SecretKey],
//        pubkeys: &[PublicKey],
//    ) -> Transaction {..}
//
// RESOURCES:
//
//  - https://github.com/bitcoin/bitcoin/blob/master/doc/descriptors.md
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
use miniscript::DefiniteDescriptorKey;
use miniscript_workshop::MiniscriptClient;
use secp256k1::{rand, Secp256k1};
use std::str::FromStr;

fn main() {
    let client = Client::configure_client();

    // Generate Keypair
    let secp = Secp256k1::new();
    let (secret_key, pub_key) = secp.generate_keypair(&mut rand::thread_rng());

    // Create Descriptor
    let s = format!("pkh({})", pub_key);
    let desc = miniscript::Descriptor::<DefiniteDescriptorKey>::from_str(&s).unwrap();
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
    client.spend_pkh(txid, desc, txout, secret_key, pub_key);

    // Assert
    let bal = client.get_received_by_address(&recv_addr, Some(1)).unwrap();
    assert_eq!(bal, Amount::from_sat(value));
}
