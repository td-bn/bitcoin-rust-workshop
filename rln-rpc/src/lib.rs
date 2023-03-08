use jsonrpsee::{server::ServerBuilder, RpcModule};
use std::net::SocketAddr;

pub async fn start_server() -> anyhow::Result<SocketAddr> {
    let server = ServerBuilder::default()
        .build("127.0.0.1:0".parse::<SocketAddr>()?)
        .await?;
    let mut module = RpcModule::new(());
    module.register_method("say_hello", |_, _| Ok("'lo"))?;

    let addr = server.local_addr()?;
    let handle = server.start(module)?;

    tokio::spawn(handle.stopped());

    println!("Hello from server");

    Ok(addr)
}

