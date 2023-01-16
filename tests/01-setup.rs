use bitcoincore_rpc::RpcApi;
use rust_bitcoin_workshop::BitcoinClient;

fn main() {
    let client = BitcoinClient::new();

    let version = client.version();
    assert!(version.is_ok());
}

