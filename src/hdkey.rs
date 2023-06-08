use std::io::Read;
use std::ops::Deref;
use std::ptr::null;
use std::str::FromStr;
use bip32::{ExtendedPrivateKey, PrivateKey, PublicKey, XPrv, DerivationPath as DPath};
use bip32::secp256k1::ecdsa::{SigningKey, VerifyingKey};
use bip32::secp256k1::elliptic_curve::sec1::{CompressedPoint, ToEncodedPoint};
use bip32::secp256k1::sha2::Sha256;
use bip39::{Mnemonic, Seed, Language};
use derivation_path::DerivationPath;
use sha3::{Digest, Keccak256};
use ripemd::{Ripemd160};

pub enum ChainType {
    BTC = 0,
    LTC = 2,
    ETH = 60,
    TRX = 195,
}

impl ToString for ChainType {
    fn to_string(&self) -> String {
        match self {
            ChainType::BTC => String::from("0"),
            ChainType::LTC => String::from("2"),
            ChainType::ETH => String::from("60"),
            ChainType::TRX => String::from("195"),
        }
    }
}

pub struct HDKey {
    pub seed: Seed,
    pub mnemonic: Mnemonic,
    pub root_xprv: ExtendedPrivateKey<SigningKey>
}

impl HDKey {

    pub fn from_mnemonic(m: String) -> Self {
        let result = Mnemonic::from_phrase(m.as_str(), Language::English);
        let mnemonic = result.unwrap();
        let seed = Seed::new(&mnemonic, "");
        let root_xprv = XPrv::new(seed.clone()).unwrap();
        HDKey {
            seed,
            mnemonic,
            root_xprv
        }
    }

    pub fn generate(&self, chain_type: ChainType, index: u32) -> Vec<u8> {
        // let child_path = "m/44'/195'/0'/0/0";
        let path = DerivationPath::bip44(chain_type as u32, 0, 0, index).unwrap();
        println!("[path]:{}", path.to_string());
        let p = DPath::from_str(&path.to_string()).unwrap();
        let child_xprv = XPrv::derive_from_path(&self.seed, &p).unwrap();
        let private_key = child_xprv.private_key();
        println!("[priv_key]:{}", hex::encode(private_key.to_bytes().clone()));
        // 97c2ff0e159ff11d70164c9d210b60a1e23fcedb37cb8e0206aa817bd932692c
        let compressed_point = private_key.clone().public_key().to_bytes();
        let compressed_pub_key_bytes = compressed_point.clone();
        println!("[compressed_pub_key]:{}", hex::encode(compressed_pub_key_bytes));
        let public_key = private_key.public_key();
        let point = public_key.to_encoded_point(false);
        let mut pub_key_bytes = point.as_bytes();
        pub_key_bytes = &pub_key_bytes[1..];
        pub_key_bytes.to_vec()
    }

    pub fn gen_trx(&self, pub_key: Vec<u8>) -> String {
        let mut hasher = Keccak256::new();
        hasher.update(pub_key.clone());
        let _hash = hasher.finalize();
        let hash = &_hash[12..];
        let mut init_address: Vec<u8>= vec![0x41];
        init_address.extend(hash);
        let hash2 = Sha256::digest(&Sha256::digest(&init_address).to_vec()).to_vec();
        let check_sum = &hash2[0..4];
        init_address.extend(check_sum);
        return bs58::encode(init_address).into_string();
    }
}
