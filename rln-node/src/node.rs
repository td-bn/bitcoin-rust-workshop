use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

use crate::bitcoin_client::BitcoindClient;
use crate::channel_manager_utils::get_channel_manager;
use crate::event_handler::RLNEventHandler;
use crate::keys_manager::get_keys_manager;
use crate::logger::RLNLogger;
use bitcoincore_rpc::bitcoin::blockdata::constants::genesis_block;
use bitcoincore_rpc::bitcoin::secp256k1::rand::{thread_rng, RngCore};
use bitcoincore_rpc::bitcoin::Network;
use lightning::chain::keysinterface::{InMemorySigner, KeysInterface, KeysManager, Recipient};
use lightning::chain::{self, Filter};
use lightning::chain::{chainmonitor, ChannelMonitorUpdateStatus, Watch};
use lightning::ln::channelmanager::SimpleArcChannelManager;
use lightning::ln::peer_handler::{IgnoringMessageHandler, MessageHandler, SimpleArcPeerManager};
use lightning::onion_message::SimpleArcOnionMessenger;
use lightning::routing::gossip::{self, P2PGossipSync};
use lightning::routing::router::DefaultRouter;
use lightning::routing::scoring::{ProbabilisticScorer, ProbabilisticScoringParameters};
use lightning_background_processor::{BackgroundProcessor, GossipSync};
use lightning_block_sync::init::synchronize_listeners;
use lightning_block_sync::{poll, SpvClient, UnboundedCache};
use lightning_invoice::payment;
use lightning_net_tokio::SocketDescriptor;
use lightning_persister::FilesystemPersister;

pub(crate) type ChainMonitor = chainmonitor::ChainMonitor<
    InMemorySigner,
    Arc<dyn Filter + Send + Sync>,
    Arc<BitcoindClient>,
    Arc<BitcoindClient>,
    Arc<RLNLogger>,
    Arc<FilesystemPersister>,
>;

pub(crate) type ChannelManager =
    SimpleArcChannelManager<ChainMonitor, BitcoindClient, BitcoindClient, RLNLogger>;

pub(crate) type PeerManager = SimpleArcPeerManager<
    SocketDescriptor,
    ChainMonitor,
    BitcoindClient,
    BitcoindClient,
    dyn chain::Access + Send + Sync,
    RLNLogger,
>;

pub(crate) type NetworkGraph = gossip::NetworkGraph<Arc<RLNLogger>>;
pub(crate) type OnionMessenger = SimpleArcOnionMessenger<RLNLogger>;
pub(crate) type InvoicePayer =
    payment::InvoicePayer<Arc<ChannelManager>, Router, Arc<RLNLogger>, RLNEventHandler>;

pub(crate) type Router = DefaultRouter<
    Arc<NetworkGraph>,
    Arc<RLNLogger>,
    Arc<Mutex<ProbabilisticScorer<Arc<NetworkGraph>, Arc<RLNLogger>>>>,
>;

pub struct Node {
    invoice_payer: Arc<InvoicePayer>,
    peer_manager: Arc<PeerManager>,
    channel_manager: Arc<ChannelManager>,
    keys_manager: Arc<KeysManager>,
    net_graph: Arc<NetworkGraph>,
    onion_messenger: Arc<OnionMessenger>,
    ln_dir: String,
    logger: Arc<RLNLogger>,
    bg_processor: BackgroundProcessor,
}

pub async fn start_node(ln_dir: &str) -> Node
{
    let bitcoind_client = Arc::new(BitcoindClient::new());
    let logger = Arc::new(RLNLogger);

    // Use sample LDK chain persistor
    let persister = Arc::new(FilesystemPersister::new(ln_dir.to_owned()));
    let chain_monitor: Arc<ChainMonitor> = Arc::new(chainmonitor::ChainMonitor::new(
        None,
        bitcoind_client.clone(),
        logger.clone(),
        bitcoind_client.clone(),
        persister.clone(),
    ));

    // Initialize key manager
    let keys_manager = Arc::new(get_keys_manager(ln_dir));
    let mut channel_monitors = persister
        .read_channelmonitors(keys_manager.clone())
        .unwrap();

    // Create channel manager
    let (channel_manager_blockhash, channel_manager) = get_channel_manager(
        bitcoind_client.clone(),
        chain_monitor.clone(),
        logger.clone(),
        keys_manager.clone(),
        ln_dir,
        &mut channel_monitors,
    );

    // Chain tip
    let mut cache = UnboundedCache::new();
    let chain_listeners = vec![(
        channel_manager_blockhash,
        &channel_manager as &dyn chain::Listen,
    )];
    let chain_tip = Some(
        synchronize_listeners(
            bitcoind_client.clone(),
            bitcoincore_rpc::bitcoin::Network::Regtest,
            &mut cache,
            chain_listeners,
        )
        .await
        .unwrap(),
    );

    // Channel monitors to chain monitor
    for (_, monitor) in channel_monitors.drain(..) {
        let outpoint = monitor.get_funding_txo().0;
        let status = chain_monitor.watch_channel(outpoint, monitor);
        assert_eq!(status, ChannelMonitorUpdateStatus::Completed);
    }

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

    let peer_manager: Arc<PeerManager> = Arc::new(PeerManager::new(
        ln_message_handler,
        keys_manager.get_node_secret(Recipient::Node).unwrap(),
        current_time.try_into().unwrap(),
        &epheremal_bytes,
        logger.clone(),
        IgnoringMessageHandler {},
    ));

    // Initialize network
    let listen_port = 9735;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", listen_port))
        .await
        .unwrap();
    let peer_manager_connection = peer_manager.clone();
    tokio::spawn(async move {
        loop {
            let tcp_stream = listener.accept().await.unwrap().0;
            let peer_manager_connection = peer_manager_connection.clone();
            tokio::spawn(async move {
                lightning_net_tokio::setup_inbound(
                    peer_manager_connection,
                    tcp_stream.into_std().unwrap(),
                )
                .await
            });
        }
    });

    // Keeping LKD up to date
    let channel_manager_spv = channel_manager.clone();
    let chain_monitor_spv = chain_monitor.clone();

    tokio::spawn(async move {
        let chain_poller = poll::ChainPoller::new(bitcoind_client.clone(), Network::Regtest);
        let chain_listener = (chain_monitor_spv, channel_manager_spv);
        let mut spv_client = SpvClient::new(
            chain_tip.unwrap(),
            chain_poller,
            &mut cache,
            &chain_listener,
        );
        loop {
            spv_client.poll_best_tip().await.unwrap();
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });

    // LDK event handler
    let event_handler = RLNEventHandler;

    // Prob. scorer
    let scorer_params = ProbabilisticScoringParameters::default();
    let scorer = Arc::new(Mutex::new(ProbabilisticScorer::new(
        scorer_params,
        network_graph.clone(),
        logger.clone(),
    )));

    // 	InvoicePayer
    let router = DefaultRouter::new(
        network_graph.clone(),
        logger.clone(),
        keys_manager.get_secure_random_bytes(),
        scorer.clone(),
    );
    let invoice_payer = Arc::new(InvoicePayer::new(
        channel_manager.clone(),
        router,
        logger.clone(),
        event_handler,
        payment::Retry::Timeout(Duration::from_secs(5)),
    ));

    // Background process
    let bg_process = BackgroundProcessor::start(
        persister,
        invoice_payer.clone(),
        chain_monitor.clone(),
        channel_manager.clone(),
        GossipSync::p2p(gossip_sync.clone()),
        peer_manager.clone(),
        logger.clone(),
        Some(scorer.clone()),
    );

    Node {
        invoice_payer: invoice_payer.clone(),
        peer_manager: peer_manager.clone(),
        channel_manager: channel_manager.clone(),
        keys_manager: keys_manager.clone(),
        net_graph: network_graph.clone(),
        onion_messenger: onion_messenger.clone(),
        ln_dir: ln_dir.to_owned(),
        logger: logger.clone(),
        bg_processor: bg_process,
    }
}
