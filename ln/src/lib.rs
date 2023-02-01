use std::path::Path;
use clightningrpc::LightningRPC;

pub trait LNClient {
    fn get_client() -> LightningRPC;
}

impl LNClient for LightningRPC {
    fn get_client() -> LightningRPC {
        unimplemented!()
    }
}

