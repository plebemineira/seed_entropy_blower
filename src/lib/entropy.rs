use super::audio::blow_entropy;
use bip39::Mnemonic;

pub struct SeedEntropyBlower {
    entropy: Vec<u8>,
    bip39_seed_phrase: Mnemonic,
}

impl SeedEntropyBlower {
    pub fn new() -> Self {
        let entropy = blow_entropy();
        Self {
            entropy: entropy.clone(),
            bip39_seed_phrase: bip39::Mnemonic::from_entropy(&entropy).expect("Failed to create BIP39 seed phrase from entropy")
        }
    }

    #[allow(dead_code)]
    pub fn get_entropy(self) -> Vec<u8> {
        self.entropy
    }

    pub fn get_bip39_seed_phrase(&self) -> &Mnemonic {
        &self.bip39_seed_phrase
    }
}