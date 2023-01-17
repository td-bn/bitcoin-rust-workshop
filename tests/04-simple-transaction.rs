// 

use bitcoincore_rpc::RpcApi;
use rust_bitcoin_workshop::*;

fn main() {
    let client = BitcoinClient::new();
    let wallet_name = "test_wallet_5";
    client.load_wallet(wallet_name);
    client.get_dough_if_broke();
    let address = client.get_new_address(None, None).unwrap();
    client.transfer(&address, 1f64);

    let amount = client.get_received_by_address(&address, None).unwrap();
    assert_eq!(amount.to_btc(), 1f64);
}

