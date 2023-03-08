use std::collections::HashMap;

use bitcoincore_rpc::{
    bitcoin::{secp256k1::SecretKey, Address, Amount, PrivateKey, Txid},
    bitcoincore_rpc_json::{
        AddMultiSigAddressResult, CreateRawTransactionInput, GetWalletInfoResult,
        ListUnspentResultEntry, SignRawTransactionInput,
    },
    jsonrpc::{error::Error::Rpc, serde_json::Value},
    Auth, Client,
    Error::JsonRpc,
    RpcApi,
};

pub trait BitcoinClient {
    pub fn setup() -> Client;
    fn with_custom_path(path: &str) -> Client;
    pub fn load_wallet_in_node(&self, wallet_name: &str) -> GetWalletInfoResult;
    pub fn get_dough_if_broke(&self);
    fn transfer(&self, address: &Address, amount: f64) -> Txid;
    fn transmit_raw_transaction(
        &self,
        utxo: &ListUnspentResultEntry,
        address: &Address,
        amount: Amount,
    );
    fn multi_sig_tx(
        &self,
        n: usize,
        pubkeys: &Vec<String>,
    ) -> (u64, u64, Txid, AddMultiSigAddressResult);
    fn spend_multisig(
        &self,
        txid: Txid,
        vout: u64,
        to: &Address,
        amount: Amount,
        res: AddMultiSigAddressResult,
        secret_keys: &[SecretKey],
    );
}

impl BitcoinClient for Client {
    fn setup() -> Self {
        let auth = Auth::UserPass("user".to_string(), "userBTCNode@123".to_string());
        Client::new("http://localhost:18443", auth).unwrap()
    }

    fn with_custom_path(path: &str) -> Self {
        let auth = Auth::UserPass("user".to_string(), "userBTCNode@123".to_string());
        Client::new(&format!("http://localhost:18443/{}", path), auth).unwrap()
    }

    fn load_wallet_in_node(&self, wallet_name: &str) -> GetWalletInfoResult {
        match self.load_wallet(wallet_name) {
            Ok(_) => self.get_wallet_info().unwrap(),
            Err(e) => {
                match e {
                    JsonRpc(je) => match je {
                        Rpc(rpc_error) => {
                            match rpc_error.code {
                                -4 => {
                                    // Wallet already exists
                                    Self::with_custom_path(&format!("wallet/{}", wallet_name))
                                        .get_wallet_info()
                                        .unwrap()
                                }
                                -18 => {
                                    // Does not exist
                                    let _ = self.create_wallet(wallet_name, None, None, None, None);
                                    Self::with_custom_path(&format!("wallet/{}", wallet_name))
                                        .get_wallet_info()
                                        .unwrap()
                                }
                                _ => unimplemented!(),
                            }
                        }
                        _ => unimplemented!(),
                    },
                    _ => unimplemented!(),
                }
            }
        }
    }

    fn get_dough_if_broke(&self) {
        match self.get_balance(None, None) {
            Ok(bal) if bal.to_btc() == 0f64 => {
                let address = self.get_new_address(None, None).unwrap();
                match self.generate_to_address(101, &address) {
                    Ok(_) => (),

                    // TODO fix: Mining 101 blocks fails with error after mining blocks, not sure why!!?
                    // "JsonRpc(Transport(SocketError(Os { code: 35, kind: WouldBlock, message: "Resource
                    // temporarily unavailable" })))"
                    //
                    // To reproduce: delete the regtest chain (delete regtest dir in bitcoin data) and run
                    // `cargo test`
                    Err(e) => {
                        eprintln!("Error occurred while mining 101 blocks:\n{:?}", e);
                    }
                }
            }
            Err(e) => panic!(
                "Failed to get balance for the loaded wallet with error: \n{:?}",
                e
            ),
            _ => (),
        }
    }

    fn transfer(&self, address: &Address, amount: f64) -> Txid {
        let amount = match Amount::from_btc(amount) {
            Ok(a) => a,
            _ => panic!("Incorrect amount, please check again"),
        };
        let txid = match self.send_to_address(address, amount, None, None, None, None, None, None) {
            Ok(txid) => (txid),
            Err(e) => panic!(
                "Failed to send transfer to {} with error:\n {:?}",
                address, e
            ),
        };
        // Generate a block
        self.generate_to_address(1, &address).unwrap();
        txid
    }

    fn transmit_raw_transaction(
        &self,
        utxo: &ListUnspentResultEntry,
        address: &Address,
        amount: Amount,
    ) {
        let tx_input = CreateRawTransactionInput {
            txid: utxo.txid,
            vout: utxo.vout,
            sequence: None,
        };
        let mut outs = HashMap::new();
        outs.insert(address.to_string(), amount);
        let hex_string = match self.create_raw_transaction_hex(&[tx_input], &outs, None, None) {
            Ok(res) => res,
            Err(r) => panic!("Failed to create raw transaction with error: {:#?}", r),
        };

        let raw_tx_res = match self.sign_raw_transaction_with_wallet(hex_string, None, None) {
            Ok(res) => res,
            Err(r) => panic!("Failed to sign raw transaction with error: {:#?}", r),
        };

        let _txid = match self.send_raw_transaction(&raw_tx_res.hex) {
            Ok(res) => res,
            Err(e) => panic!("Failed to sign raw transaction with error: {:#?}", e),
        };
        self.generate_to_address(1, &address).unwrap();
    }

    fn spend_multisig(
        &self,
        txid: Txid,
        vout: u64,
        to: &Address,
        amount: Amount,
        res: AddMultiSigAddressResult,
        secret_keys: &[SecretKey],
    ) {
        let tx_input = CreateRawTransactionInput {
            txid: txid.clone(),
            vout: vout as u32,
            sequence: None,
        };
        let mut outs = HashMap::new();
        outs.insert(to.to_string(), amount);
        let hex_string = match self.create_raw_transaction_hex(&[tx_input], &outs, None, None) {
            Ok(res) => res,
            Err(r) => panic!("Failed to create raw transaction with error: {:#?}", r),
        };

        let s_tx_input = SignRawTransactionInput {
            txid,
            vout: vout as u32,
            script_pub_key: res.address.script_pubkey(),
            redeem_script: Some(res.redeem_script),
            amount: Some(amount),
        };

        let priv_keys: Vec<_> = secret_keys
            .iter()
            .map(|sk| PrivateKey::new(sk.to_owned(), bitcoincore_rpc::bitcoin::Network::Regtest))
            .collect();

        let raw_tx_res = match self.sign_raw_transaction_with_key(
            hex_string,
            &priv_keys,
            Some(&[s_tx_input]),
            None,
        ) {
            Ok(res) => res,
            Err(r) => panic!("Failed to sign raw transaction with error: {:#?}", r),
        };

        let _txid = match self.send_raw_transaction(&raw_tx_res.hex) {
            Ok(res) => res,
            Err(e) => panic!("Failed to sign raw transaction with error: {:#?}", e),
        };
        self.generate_to_address(1, &to).unwrap();
    }

    fn multi_sig_tx(
        &self,
        n: usize,
        pubkeys: &Vec<String>,
    ) -> (u64, u64, Txid, AddMultiSigAddressResult) {
        let multi_sig_addrs: Vec<_> = pubkeys.iter().map(|p| Value::String(p.clone())).collect();

        let res: AddMultiSigAddressResult = match self.call(
            &"createmultisig",
            &[Value::Number(n.into()), Value::Array(multi_sig_addrs)],
        ) {
            Ok(res) => res,
            Err(e) => panic!("Failed to create multi sig with error: {:#?}", e),
        };

        // Pay to script
        let txid = self.transfer(&res.address, 10.0);

        let latest_blockhash = self.get_best_block_hash().unwrap();

        let utxos: Vec<_> = self
            .get_raw_transaction(&txid, Some(&latest_blockhash))
            .unwrap()
            .output;
        let filtered: Vec<_> = utxos
            .iter()
            .enumerate()
            .filter(|(_, o)| o.script_pubkey.eq(&res.address.script_pubkey()))
            .collect();

        assert!(filtered.len() > 0);
        let utxo = filtered.first().unwrap().to_owned();
        (utxo.0 as u64, utxo.1.value, txid, res)
    }
}
