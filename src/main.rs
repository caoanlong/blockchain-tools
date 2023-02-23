extern crate core;

use crate::hdkey::{ChainType, HDKey};

mod mnemonic;
mod hdkey;

fn main() {
    let mnemonic_phrase = mnemonic::create_mnemonic();
    let root_key = HDKey::from_mnemonic(mnemonic_phrase);
    let pub_key = root_key.generate(ChainType::ETH, 0);
    println!("[pub_key]: {}", hex::encode(pub_key));
    // let address = root_key.gen_trx(pub_key);
    // println!("[address]: {}", address.len());
}
