use rlnrpc::start_server;

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let addr = start_server().await?;

    Ok(())
}
