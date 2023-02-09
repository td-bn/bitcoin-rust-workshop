use bitcoin_basics::BitcoinClient;
use bitcoincore_rpc::{
    bitcoin::{util::uint::Uint256, BlockHash},
    Client, RpcApi,
};
use lightning::chain::chaininterface::{BroadcasterInterface, FeeEstimator};
use lightning_block_sync::{BlockData, BlockHeaderData, BlockSource};

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

impl BlockSource for BitcoindClient {
    fn get_block<'a>(
        &'a self,
        header_hash: &'a BlockHash,
    ) -> lightning_block_sync::AsyncBlockSourceResult<'a, lightning_block_sync::BlockData> {
        let block = self
            .client
            .get_block(header_hash)
            .expect("Failed to get block");
        Box::pin(async move { Ok(BlockData::FullBlock(block)) })
    }

    fn get_header<'a>(
        &'a self,
        header_hash: &'a BlockHash,
        _height_hint: Option<u32>,
    ) -> lightning_block_sync::AsyncBlockSourceResult<'a, lightning_block_sync::BlockHeaderData>
    {
        let header = self
            .client
            .get_block_header(header_hash)
            .expect("Failed to get header");
        let info = self
            .client
            .get_block_header_info(header_hash)
            .expect("Failed to get header info");
        let mut chainw = [0 as u8; 32];
        for (i, v) in info.chainwork .iter() .enumerate() {
            chainw[i] = *v;
        }
        Box::pin(async move {
            Ok(BlockHeaderData {
                header,
                height: info.height as u32,
                chainwork: Uint256::from_be_bytes(chainw),
            })
        })
    }

    fn get_best_block<'a>(
        &'a self,
    ) -> lightning_block_sync::AsyncBlockSourceResult<(BlockHash, Option<u32>)> {
        let hash = self.get_best_blockhash();
        let height = self.get_block_height(&hash) as u32;
        Box::pin(async move {
            Ok((hash, Some(height)))
        })
    }
}
