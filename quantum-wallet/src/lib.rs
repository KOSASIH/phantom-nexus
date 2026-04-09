//! # Quantum Wallet
//!
//! Hardware/software wallet with biometric + brainwave authentication.
//! Quantum-resistant key storage and transaction signing.

use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WalletError {
    #[error("authentication failed")]
    AuthFailed,
    #[error("insufficient balance: have {balance}, need {required}")]
    InsufficientBalance { balance: u128, required: u128 },
    #[error("wallet locked")]
    Locked,
    #[error("invalid biometric data")]
    InvalidBiometric,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AuthMethod {
    Biometric { fingerprint_hash: [u8; 32] },
    Brainwave { pattern_hash: [u8; 32] },
    MultiSig { threshold: u8, signers: Vec<String> },
    Combined { methods: Vec<AuthMethod> },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WalletAccount {
    pub address: String,
    pub balance: u128,
    pub nonce: u64,
    pub auth_methods: Vec<AuthMethod>,
    pub locked: bool,
}

/// Quantum Wallet
pub struct QuantumWallet {
    pub accounts: HashMap<String, WalletAccount>,
    pub active_account: Option<String>,
    pub locked: bool,
}

impl QuantumWallet {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            active_account: None,
            locked: true,
        }
    }

    /// Create a new wallet account
    pub fn create_account(&mut self, auth_methods: Vec<AuthMethod>) -> String {
        let mut hasher = Sha3_256::new();
        hasher.update(format!("{:?}", auth_methods).as_bytes());
        hasher.update(&rand::random::<[u8; 32]>());
        let hash = hasher.finalize();
        let address = format!("0x{}", hex::encode(&hash));

        let account = WalletAccount {
            address: address.clone(),
            balance: 0,
            nonce: 0,
            auth_methods,
            locked: false,
        };

        self.accounts.insert(address.clone(), account);
        address
    }

    /// Unlock wallet with biometric authentication
    pub fn unlock_biometric(&mut self, fingerprint: &[u8]) -> Result<(), WalletError> {
        let mut hasher = Sha3_256::new();
        hasher.update(fingerprint);
        let hash = hasher.finalize();
        let mut fingerprint_hash = [0u8; 32];
        fingerprint_hash.copy_from_slice(&hash);

        // Verify against stored biometric
        if let Some(addr) = &self.active_account {
            if let Some(account) = self.accounts.get(addr) {
                for method in &account.auth_methods {
                    if let AuthMethod::Biometric { fingerprint_hash: stored } = method {
                        if *stored == fingerprint_hash {
                            self.locked = false;
                            return Ok(());
                        }
                    }
                }
            }
        }

        Err(WalletError::AuthFailed)
    }

    /// Get balance of active account
    pub fn balance(&self) -> Result<u128, WalletError> {
        if self.locked {
            return Err(WalletError::Locked);
        }
        let addr = self.active_account.as_ref().ok_or(WalletError::Locked)?;
        let account = self.accounts.get(addr).ok_or(WalletError::Locked)?;
        Ok(account.balance)
    }
}
