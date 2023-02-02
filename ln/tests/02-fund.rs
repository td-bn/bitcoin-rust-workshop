use ln_workshop::LNClient;
use clightningrpc::LightningRPC;

fn main() {
    let node = "ln-nodes/node-1/regtest/lightning-rpc";
    let client = LightningRPC::get_client(node);
    client.add_funds();
    let funds_list = client.listfunds();
    assert!(funds_list.is_ok());
    assert!(funds_list.unwrap().outputs.len() > 0);
}

