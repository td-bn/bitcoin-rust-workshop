use ln_workshop::LNClient;
use clightningrpc::LightningRPC;

fn main() {
    let client = LightningRPC::get_client();
    let info = client.getinfo();
    assert!(info.is_ok());
}

