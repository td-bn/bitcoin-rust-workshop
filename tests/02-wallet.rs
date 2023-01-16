use rust_bitcoin_workshop::*;

fn main() {
    let client = BitcoinClient::new();
    let wallet_name = "test_wallet_5";
    let wallet_info = client.load_wallet(wallet_name);

    assert_eq!(wallet_name, wallet_info.wallet_name);
}

