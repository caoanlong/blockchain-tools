extern crate core;

use crate::hdkey::{ChainType, HDKey};

mod mnemonic;
mod hdkey;

fn main() {
    // let mnemonic_phrase = mnemonic::create_mnemonic();
    let mnemonic_phrase = String::from("liar torch useful breeze vacuum release kit purpose wave carpet weekend bonus");
    let root_key = HDKey::from_mnemonic(mnemonic_phrase);
    let pub_key = root_key.generate(ChainType::TRX, 0);
    println!("[pub_key]: {}", hex::encode(pub_key.clone()));
    let address = root_key.gen_trx(pub_key.clone());
    println!("[address]: {}", address);
}
