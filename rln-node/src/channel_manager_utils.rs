use std::{fs, sync::Arc};

use bitcoincore_rpc::bitcoin::BlockHash;
use lightning::chain::chainmonitor;
use lightning::chain::channelmonitor::ChannelMonitor;
use lightning::util::ser::ReadableArgs;
use lightning::{
    chain::{
        keysinterface::{InMemorySigner, KeysManager},
        BestBlock, Filter,
    },
    ln::channelmanager::{ChainParameters, ChannelManagerReadArgs, SimpleArcChannelManager},
    util::config::UserConfig,
};
use lightning_persister::FilesystemPersister;

use crate::{bitcoin_client::BitcoindClient, logger::RLNLogger};

type ChainMonitor = chainmonitor::ChainMonitor<
    InMemorySigner,
    Arc<dyn Filter + Send + Sync>,
    Arc<BitcoindClient>,
    Arc<BitcoindClient>,
    Arc<RLNLogger>,
    Arc<FilesystemPersister>,
>;

type ChannelManager =
    SimpleArcChannelManager<ChainMonitor, BitcoindClient, BitcoindClient, RLNLogger>;

pub fn get_channel_manager(
    bitcoind_client: Arc<BitcoindClient>,
    chain_monitor: Arc<ChainMonitor>,
    logger: Arc<RLNLogger>,
    keys_manager: Arc<KeysManager>,
    ldk_data_dir: &str,
    channelmonitors: &mut Vec<(BlockHash, ChannelMonitor<InMemorySigner>)>,
) -> (BlockHash, ChannelManager) {
    // Restarting
    if let Ok(mut f) = fs::File::open(format!("{}/manager", ldk_data_dir)) {
        let mut channel_monitor_mut_references = Vec::new();
        for (_, channel_monitor) in channelmonitors.iter_mut() {
            channel_monitor_mut_references.push(channel_monitor);
        }
        let read_args = ChannelManagerReadArgs::new(
            keys_manager.clone(),
            bitcoind_client.clone(),
            chain_monitor.clone(),
            bitcoind_client.clone(),
            logger.clone(),
            UserConfig::default(),
            channel_monitor_mut_references,
        );
        <(BlockHash, ChannelManager)>::read(&mut f, read_args).unwrap()
    } else {

        // Create channel manager
        let best_blockhash = bitcoind_client.get_best_blockhash();
        let height = bitcoind_client.get_block_height(&best_blockhash);

        let chain_params = ChainParameters {
            network: bitcoincore_rpc::bitcoin::Network::Regtest,
            best_block: BestBlock::new(best_blockhash, height as u32),
        };

        (
            best_blockhash,
            ChannelManager::new(
                bitcoind_client.clone(),
                chain_monitor.clone(),
                bitcoind_client.clone(),
                logger.clone(),
                keys_manager.clone(),
                UserConfig::default(),
                chain_params,
            ),
        )
    }
}
