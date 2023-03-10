use ln_workshop::LNClient;
use clightningrpc::LightningRPC;

fn main() {
    let node = "ln-nodes/node-1/regtest/lightning-rpc";
    let client = LightningRPC::get_client(node);
    let info = client.getinfo();
    assert!(info.is_ok());
}

