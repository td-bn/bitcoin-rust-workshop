use clightningrpc::responses::NetworkAddress;
use clightningrpc::LightningRPC;
use ln_workshop::LNClient;

fn main() {
    // RPC clients
    let node1 = "ln-nodes/node-1/regtest/lightning-rpc";
    let node2 = "ln-nodes/node-2/regtest/lightning-rpc";
    let client1 = LightningRPC::get_client(node1);
    let client2 = LightningRPC::get_client(node2);

    // Add funds
    client1.add_funds();

    // Get connection id
    let info = client2.getinfo().expect("Failed to get info");
    let ipv4 = info.binding.first().expect("Failed to get first item");
    let (address, port) = match ipv4 {
        NetworkAddress::Ipv4 { address, port } => (address, port),
        _ => unimplemented!(),
    };
    let id = format!("{}@{}:{}", info.id, address, port);

    // Create channel
    let channel_id = client1.create_channel(&id);
}
