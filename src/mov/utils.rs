use base64_light::*;
use bip39::{Language, Mnemonic};
use std::error::Error;

pub fn is_valid_base64_key(key: &str) -> bool {
    if key.len() == 0 {
        return false;
    }

    let decoded = base64_decode(key);
    if decoded.len() != 33 {
        return false;
    }

    if decoded[0] != 0x00 {
        return false;
    }

    true
}

pub fn is_valid_mnemonic(mne: &str) -> bool {
    if mne.len() == 0 {
        return false;
    }
    match Mnemonic::from_phrase(mne, Language::English) {
        Ok(_) => true,
        Err(_) => false,
    }
}
