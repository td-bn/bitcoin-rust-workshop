use bitcoincore_rpc::RpcApi;
use rust_bitcoin_workshop::setup;

fn main() {
    let rpc = setup();

    let version = rpc.version();
    assert!(version.is_ok());
}

