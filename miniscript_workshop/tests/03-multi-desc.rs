// Creating a descriptor.
//
// Create a multi sig descriptor to pass the test.
//
//  impl MiniscriptClient for Client {
//     fn multisig_desc(&self, n: usize, pubkeys: &[PublicKey]) -> Descriptor<DefiniteDescriptorKey>;
//  }
//
//  Takes in n, the number of confirmations and a list of pubkeys and returns a 
//  descriptor describing the n of m multisig.
//
// RESOURCES:
//
//  - https://github.com/bitcoin/bitcoin/blob/master/doc/descriptors.md
//  - https://docs.rs/miniscript/latest/miniscript/descriptor/struct.DefiniteDescriptorKey.html
//

use bitcoincore_rpc::{Client, bitcoin};
use miniscript_workshop::MiniscriptClient;
use secp256k1::{rand, Secp256k1};

fn main() {
    let client = Client::configure_client();

    // Generate keypairs
    let secp = Secp256k1::new();
    let mut pubkeys = vec![];
    for _ in 0..3 {
        let (_, pub_key) = secp.generate_keypair(&mut rand::thread_rng());
        pubkeys.push(pub_key);
    }

    // Create descriptors
    let desc1 = client.multisig_desc(2, &pubkeys);
    pubkeys.swap(0,2);
    let desc2 = client.multisig_desc(2, &pubkeys);

    // Assert
    let add1 = desc1.address(bitcoin::Network::Regtest).unwrap();
    let add2 = desc2.address(bitcoin::Network::Regtest).unwrap();

    assert_eq!(add1, add2);
}
