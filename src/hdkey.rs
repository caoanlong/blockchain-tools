use std::ops::Deref;
use std::str::FromStr;
use bip32::{ExtendedPrivateKey, PrivateKey, XPrv, DerivationPath as DPath};
use bip32::secp256k1::ecdsa::SigningKey;
use bip39::{Seed, Mnemonic, Language};
use derivation_path::DerivationPath;
use sha3::{Digest, Keccak256};

pub enum ChainType {
    BTC = 0,
    LTC = 2,
    ETH = 60,
    TRX = 195,
}

pub struct HDKey {
    pub seed: Seed,
    pub mnemonic: Mnemonic,
    pub root_xprv: ExtendedPrivateKey<SigningKey>
}

impl HDKey {
    // pub fn from_seed(seed: Seed) -> Self {
    //     let root_xprv = XPrv::new(&seed)?;
    //     assert_eq!(root_xprv, XPrv::derive_from_path(&seed, &"m".parse()?)?);
    //
    //     HDKey {
    //         seed,
    //         mnemonic: NULL,
    //         root_xprv
    //     }
    // }

    pub fn from_mnemonic(m: String) -> Self {
        let result = Mnemonic::from_phrase(m.as_str(), Language::English);
        let mnemonic = result.unwrap();
        let seed = Seed::new(&mnemonic, "");
        let root_xprv = XPrv::new(&seed).unwrap();
        HDKey {
            seed,
            mnemonic,
            root_xprv
        }
    }

    pub fn generate(&self, chain_type: ChainType, index: u32) -> &[u8] {
        let path = DerivationPath::bip44(44, chain_type as u32, 0, index).unwrap();
        let p = DPath::from_str(path.to_string().as_str()).unwrap();
        let child_xprv = XPrv::derive_from_path(&self.seed, &p).unwrap();
        let private_key = child_xprv.private_key();
        let compressed_point = private_key.public_key().to_bytes();
        let pub_key_bytes = compressed_point.clone();
        // let pub_key = hex::encode(pub_key_bytes);
        // println!("[len]:{}", pub_key.len());
        &pub_key_bytes.as_slice()
    }

    // pub fn gen_trx(&self, pub_key: &[u8]) -> &[u8] {
    //     let mut hasher = Keccak256::new();
    //     hasher.update(pub_key);
    //     let result = hasher.finalize();
    //     let r = &result[..];
    //     return r
    // }
}
