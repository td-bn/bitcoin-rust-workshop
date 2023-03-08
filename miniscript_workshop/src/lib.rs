use std::{collections::BTreeMap, str::FromStr};

use bitcoincore_rpc::{
    bitcoin::{
        self as bitcoin,
        util::{
            psbt::{self, PartiallySignedTransaction},
            sighash::SighashCache,
        },
        OutPoint, PackedLockTime, Script, Sequence, Transaction, TxIn, TxOut, Txid,
    },
    Client, RpcApi,
};

use bitcoin_basics::BitcoinClient;

use miniscript::{
    psbt::{PsbtExt, PsbtInputExt},
    DefiniteDescriptorKey, Descriptor,
};
use secp256k1::{PublicKey, Secp256k1, SecretKey};

pub trait MiniscriptClient {
    fn configure_client() -> Client;
    fn sendrawtx(&self, tx: Transaction);
    fn spend_pkh(
        &self,
        txid: Txid,
        desc: Descriptor<DefiniteDescriptorKey>,
        txout: TxOut,
        secret_key: secp256k1::SecretKey,
        pub_key: secp256k1::PublicKey,
    );
    fn multisig_desc(&self, n: usize, pubkeys: &[PublicKey]) -> Descriptor<DefiniteDescriptorKey>;
    fn spend_multi(
        &self,
        txid: Txid,
        desc: Descriptor<DefiniteDescriptorKey>,
        out_txout: TxOut,
        secrets: &[SecretKey],
        pubkeys: &[PublicKey],
    );
    fn utxo_details(&self, script_pubkey: Script, txid: Txid) -> Result<(OutPoint, TxOut), ()>;
}

impl MiniscriptClient for Client {
    fn configure_client() -> Self {
        // Load client with wallet and some
        let client = Client::setup();
        client.load_wallet_in_node("test_wallet");
        client.get_dough_if_broke();
        client
    }

    fn sendrawtx(&self, tx: Transaction) {
        let _txid = self
            .send_raw_transaction(&tx)
            .expect(&format!("send raw tx failed \nTx: {:#?}", tx));

        let _blocks = self
            .generate_to_address(1, &self.get_new_address(None, None).unwrap())
            .unwrap();
    }

    fn spend_pkh(
        &self,
        txid: Txid,
        desc: Descriptor<DefiniteDescriptorKey>,
        out_txout: TxOut,
        secret_key: secp256k1::SecretKey,
        pub_key: secp256k1::PublicKey,
    ) {
        let mut psbt = psbt_new();

        let (outp, txout) = self
            .utxo_details(desc.script_pubkey(), txid)
            .expect("Failed to get UTXO deatils");

        update_psbt(&mut psbt, outp, out_txout, &desc, txout);

        let tx = sign_and_finalize(&mut psbt, &[secret_key], &[pub_key]);
        self.sendrawtx(tx);
    }

    fn multisig_desc(&self, n: usize, pubkeys: &[PublicKey]) -> Descriptor<DefiniteDescriptorKey> {
        let str_pks: Vec<_> = pubkeys.into_iter().map(|pk| pk.to_string()).collect();
        let s = format!("wsh(sortedmulti({},{}))", n, str_pks.join(","));
        Descriptor::<DefiniteDescriptorKey>::from_str(&s).unwrap()
    }

    fn spend_multi(
        &self,
        txid: Txid,
        desc: Descriptor<DefiniteDescriptorKey>,
        out_txout: TxOut,
        secrets: &[SecretKey],
        pubkeys: &[PublicKey],
    ) {
        let mut psbt = psbt_new();

        let (outp, txout) = self
            .utxo_details(desc.script_pubkey(), txid)
            .expect("Failed to get UTXO deatils");

        update_psbt(&mut psbt, outp, out_txout, &desc, txout);

        let tx = sign_and_finalize(&mut psbt, secrets, pubkeys);
        self.sendrawtx(tx);
    }

    fn utxo_details(&self, script_pubkey: Script, txid: Txid) -> Result<(OutPoint, TxOut), ()> {
        let tx = self
            .get_transaction(&txid, None)
            .unwrap()
            .transaction()
            .unwrap();
        for (i, txo) in tx.output.into_iter().enumerate() {
            if script_pubkey == txo.script_pubkey {
                return Ok((OutPoint::new(txid, i as u32), txo));
            }
        }
        Err(())
    }
}

fn psbt_new() -> PartiallySignedTransaction {
    PartiallySignedTransaction {
        unsigned_tx: Transaction {
            version: 2,
            lock_time: PackedLockTime::ZERO,
            input: vec![],
            output: vec![],
        },
        version: 0,
        xpub: BTreeMap::new(),
        proprietary: BTreeMap::new(),
        unknown: BTreeMap::new(),
        inputs: vec![],
        outputs: vec![],
    }
}

fn update_psbt(
    psbt: &mut PartiallySignedTransaction,
    outp: OutPoint,
    output: TxOut,
    desc: &Descriptor<DefiniteDescriptorKey>,
    txout: TxOut,
) {
    let mut txin = TxIn::default();
    txin.previous_output = outp;
    txin.sequence = Sequence::MAX;
    psbt.unsigned_tx.input.push(txin);

    // TxOut from args
    psbt.unsigned_tx.output.push(output);

    let mut input = psbt::Input::default();
    input.update_with_descriptor_unchecked(&desc).unwrap();
    input.witness_utxo = Some(txout);
    psbt.inputs.push(input);

    psbt.outputs.push(psbt::Output::default());
}

fn sign_and_finalize(
    psbt: &mut PartiallySignedTransaction,
    secrets: &[SecretKey],
    pubkeys: &[PublicKey],
) -> Transaction {
    let secp = Secp256k1::new();
    let mut sighash_cache = SighashCache::new(&psbt.unsigned_tx);
    let msg = psbt
        .sighash_msg(0, &mut sighash_cache, None)
        .unwrap()
        .to_secp_msg();
    for i in 0..secrets.len() {
        let sig = secp.sign_ecdsa(&msg, &secrets[i]);
        psbt.inputs[0].partial_sigs.insert(
            bitcoin::PublicKey::from_str(&pubkeys[i].to_string()).unwrap(),
            bitcoin::EcdsaSig {
                sig,
                hash_ty: bitcoin::EcdsaSighashType::All,
            },
        );
    }

    psbt.finalize_mut(&secp).expect("Failed to finialize tx");

    psbt.extract(&secp).expect("Extraction error")
}
