use bitcoincore_rpc::Client;

pub trait BitcoinClient {
    fn setup() -> Client;
}

impl BitcoinClient for Client {
    fn setup() -> Self {
        unimplemented!()
    }
}
