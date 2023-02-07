use lightning::chain::chainmonitor::ChainMonitor;
use lightning::chain::keysinterface::InMemorySigner;
use lightning::chain::Filter;
use lightning_persister::FilesystemPersister;
use rlnnode::bitcoin_client::BitcoindClient;
use rlnnode::logger::RLNLogger;

fn main() {
    let bitcoind_client = BitcoindClient::new();
    let logger = RLNLogger;
    let filter: Option<Box<dyn Filter>> = None;

    let persister = FilesystemPersister::new("".to_owned());
    let _chain_monitor: ChainMonitor<InMemorySigner, Box<_>, _, _, _, _> = ChainMonitor::new(
        filter,
        &bitcoind_client,
        &logger,
        &bitcoind_client,
        &persister,
    );
}
