use std::{fs, time::SystemTime};

use bitcoincore_rpc::bitcoin::secp256k1::rand::{thread_rng, RngCore};
use lightning::{chain::keysinterface::KeysManager, util::ser::Writer};

pub fn get_keys_manager(ln_dir: &str) -> KeysManager {
    let seed_path = format!("{}/keys_seed", ln_dir);
    let keys_seed = if let Ok(seed) = fs::read(seed_path.clone()) {
        assert_eq!(seed.len(), 32);
        let mut key = [0; 32];
        key.copy_from_slice(&seed);
        key
    } else {
        let mut key = [0; 32];
        thread_rng().fill_bytes(&mut key);

        match fs::File::create(seed_path.clone()) {
            Ok(mut f) => {
                f.write_all(&key)
                    .expect("Failed to write node keys to disk");
                f.sync_all().expect("Failed to sync node keys to disk");
            }
            Err(e) => {
                panic!("Failed to create seed path file: {}\n{}", seed_path, e);
            }
        }
        key
    };
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    KeysManager::new(
        &keys_seed,
        current_time.as_secs(),
        current_time.subsec_nanos(),
    )
}
