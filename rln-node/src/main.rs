use std::sync::Arc;
use std::time::SystemTime;

use bitcoincore_rpc::bitcoin::blockdata::constants::genesis_block;
use bitcoincore_rpc::bitcoin::secp256k1::rand::{thread_rng, RngCore};
use bitcoincore_rpc::bitcoin::Network;
use lightning::chain::chainmonitor;
use lightning::chain::keysinterface::{InMemorySigner, KeysInterface, Recipient};
use lightning::chain::{self, BestBlock, Filter};
use lightning::ln::channelmanager::{ChainParameters, SimpleArcChannelManager};
use lightning::ln::peer_handler::{IgnoringMessageHandler, MessageHandler, SimpleArcPeerManager};
use lightning::onion_message::OnionMessenger;
use lightning::routing::gossip::{NetworkGraph, P2PGossipSync};
use lightning::util::config::UserConfig;
use lightning_block_sync::init::synchronize_listeners;
use lightning_block_sync::UnboundedCache;
use lightning_net_tokio::SocketDescriptor;
use lightning_persister::FilesystemPersister;
use rlnnode::bitcoin_client::BitcoindClient;
use rlnnode::keys_manager::get_keys_manager;
use rlnnode::logger::RLNLogger;

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

type PeerManager = SimpleArcPeerManager<
    SocketDescriptor,
    ChainMonitor,
    BitcoindClient,
    BitcoindClient,
    dyn chain::Access + Send + Sync,
    RLNLogger,
>;

#[tokio::main]
async fn main() {
    let ln_dir = "./node_1";
    let bitcoind_client = Arc::new(BitcoindClient::new());
    let logger = Arc::new(RLNLogger);

    // Use sample LDK chain persistor
    let persister = Arc::new(FilesystemPersister::new("".to_owned()));
    let chain_monitor: Arc<ChainMonitor> = Arc::new(chainmonitor::ChainMonitor::new(
        None,
        bitcoind_client.clone(),
        logger.clone(),
        bitcoind_client.clone(),
        persister,
    ));

    // Initialize key manager
    let keys_manager = Arc::new(get_keys_manager(ln_dir));
    // let _channel_monitors = persister.read_channelmonitors(&keys_manager).unwrap();

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
                bitcoind_client.clone(),
                chain_monitor.clone(),
                bitcoind_client.clone(),
                logger.clone(),
                keys_manager.clone(),
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
            bitcoind_client,
            bitcoincore_rpc::bitcoin::Network::Regtest,
            &mut cache,
            chain_listeners,
        )
        .await
        .unwrap(),
    );

    // NetGraphMsgHandler
    let genesis_hash = genesis_block(Network::Regtest).header.block_hash();
    let network_graph = Arc::new(NetworkGraph::new(genesis_hash, logger.clone()));
    let gossip_sync = Arc::new(P2PGossipSync::new(
        Arc::clone(&network_graph),
        None::<Arc<dyn chain::Access + Send + Sync>>,
        logger.clone(),
    ));

    // Onion messenger
    let channel_manager = Arc::new(channel_manager);
    let onion_messenger = Arc::new(OnionMessenger::new(
        keys_manager.clone(),
        logger.clone(),
        IgnoringMessageHandler {},
    ));

    // Peer manager
    let mut epheremal_bytes = [0; 32];
    thread_rng().fill_bytes(&mut epheremal_bytes);
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let ln_message_handler = MessageHandler {
        chan_handler: channel_manager.clone(),
        route_handler: gossip_sync.clone(),
        onion_message_handler: onion_messenger.clone(),
    };

    let _peer_manager: SimpleArcPeerManager<
        SocketDescriptor,
        ChainMonitor,
        BitcoindClient,
        BitcoindClient,
        dyn chain::Access + Send + Sync,
        RLNLogger,
    > = PeerManager::new(
        ln_message_handler,
        keys_manager.get_node_secret(Recipient::Node).unwrap(),
        current_time.try_into().unwrap(),
        &epheremal_bytes,
        logger.clone(),
        IgnoringMessageHandler {},
    );
}
