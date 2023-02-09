use lightning::chain::chainmonitor::ChainMonitor;
use lightning::chain::keysinterface::InMemorySigner;
use lightning::chain::{self, BestBlock, Filter};
use lightning::ln::channelmanager::{ChainParameters, ChannelManager};
use lightning::util::config::UserConfig;
use lightning_block_sync::init::synchronize_listeners;
use lightning_block_sync::UnboundedCache;
use lightning_persister::FilesystemPersister;
use rlnnode::bitcoin_client::BitcoindClient;
use rlnnode::keys_manager::get_keys_manager;
use rlnnode::logger::RLNLogger;

#[tokio::main]
async fn main() {
    let ln_dir = "./node_1";
    let bitcoind_client = BitcoindClient::new();
    let logger = RLNLogger;
    let filter: Option<Box<dyn Filter>> = None;

    // Use sample LDK chain persistor
    let persister = FilesystemPersister::new("".to_owned());
    let chain_monitor: ChainMonitor<InMemorySigner, Box<_>, _, _, _, _> = ChainMonitor::new(
        filter,
        &bitcoind_client,
        &logger,
        &bitcoind_client,
        &persister,
    );

    // Initialize key manager
    let keys_manager = get_keys_manager(ln_dir);
    let _channel_monitors = persister.read_channelmonitors(&keys_manager).unwrap();

    // Create channel manager
    let (channel_manager_blockhash, channel_manager) = {
        let best_blockhash = bitcoind_client.get_best_blockhash();
        let height = bitcoind_client.get_block_height(&best_blockhash);

        let chain_params = ChainParameters {
            network: bitcoincore_rpc::bitcoin::Network::Regtest,
            best_block: BestBlock::new(best_blockhash, height as u32),
        };

        (
            best_blockhash,
            ChannelManager::new(
                &bitcoind_client,
                &chain_monitor,
                &bitcoind_client,
                &logger,
                &keys_manager,
                UserConfig::default(),
                chain_params,
            ),
        )
    };

    // Chain tip
    let mut cache = UnboundedCache::new();
    let chain_listeners = vec![(
        channel_manager_blockhash,
        &channel_manager as &dyn chain::Listen,
    )];
    let mut _chain_tip = Some(
        synchronize_listeners(
            &bitcoind_client,
            bitcoincore_rpc::bitcoin::Network::Regtest,
            &mut cache,
            chain_listeners,
        )
        .await
        .unwrap(),
    );
}
