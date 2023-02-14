use rlnnode::node::start_node;

#[tokio::main]
async fn main() {
    start_node("./node_1").await;
}
