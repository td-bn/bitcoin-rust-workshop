use bitcoincore_rpc::{Client, RpcApi};
use miniscript_workshop::MiniscriptClient;

fn main() {
    let client = Client::configure_client();

    // Assert
    let bal = client.get_balance(Some(1), None).unwrap();
    assert!(bal.to_btc() > 0f64);
}
