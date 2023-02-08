use bitcoin_basics::BitcoinClient;
use bitcoincore_rpc::{bitcoin::BlockHash, Client, RpcApi};
use lightning::chain::chaininterface::{BroadcasterInterface, FeeEstimator};

pub struct BitcoindClient {
    client: Client,
}

impl BitcoindClient {
    pub fn new() -> Self {
        let client = Client::setup();
        client.load_wallet_in_node("test_wallet");
        client.get_dough_if_broke();
        Self { client }
    }

    pub fn get_best_blockhash(&self) -> BlockHash {
        self.client
            .get_best_block_hash()
            .expect("Failed to get latest blockhash")
    }

    pub fn get_block_height(&self, blockhash: &BlockHash) -> usize {
        self.client
            .get_block_info(&blockhash)
            .expect("Failed to get height of blockhash")
            .height
    }
}

impl FeeEstimator for BitcoindClient {
    fn get_est_sat_per_1000_weight(
        &self,
        _confirmation_target: lightning::chain::chaininterface::ConfirmationTarget,
    ) -> u32 {
        // TODO: more sophisticated
        1000
    }
}

impl BroadcasterInterface for BitcoindClient {
    fn broadcast_transaction(&self, tx: &bitcoincore_rpc::bitcoin::Transaction) {
        self.client
            .send_raw_transaction(tx)
            .expect("Failed to send raw tx");
    }
}
