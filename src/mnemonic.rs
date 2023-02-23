use std::time::Instant;
use bip39::{Mnemonic, MnemonicType, Language};

pub fn create_mnemonic() -> String {
    let now = Instant::now();
    let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
    let phrase: &str = mnemonic.phrase();
    let elapsed_time = now.elapsed();
    let time = elapsed_time.as_micros() as f64 / 1000.0;
    println!("[time]: {} ms, [mnemonic]: {}", time, phrase);
    return String::from(phrase)
}