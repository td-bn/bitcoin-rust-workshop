use bitcoincore_rpc::{Client, bitcoin};
use miniscript_workshop::MiniscriptClient;
use secp256k1::{rand, Secp256k1};

fn main() {
    let client = Client::configure_client();

    // Generate keypairs
    let secp = Secp256k1::new();
    let mut pubkeys = vec![];
    let mut secrets = vec![];
    for _ in 0..3 {
        let (secret_key, pub_key) = secp.generate_keypair(&mut rand::thread_rng());
        pubkeys.push(pub_key.to_string());
        secrets.push(secret_key);
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
