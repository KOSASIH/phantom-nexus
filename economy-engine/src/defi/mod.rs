//! DeFi 2.0 Phantom Vaults
//!
//! AI-optimized yield strategies with zero impermanent loss.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Vault strategy type
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum VaultStrategy {
    YieldFarming { pools: Vec<String> },
    Arbitrage { pairs: Vec<(String, String)> },
    LiquidityProvision { dex: String },
    StablecoinYield,
    DeltaNeutral,
}

/// Phantom Vault
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PhantomVault {
    pub id: String,
    pub name: String,
    pub strategy: VaultStrategy,
    pub total_value_locked: u128,
    pub apy: f64,
    pub risk_score: f64,  // 0.0 (safe) to 1.0 (risky)
    pub depositors: HashMap<String, u128>,
    pub ai_managed: bool,
    pub auto_compound: bool,
}

impl PhantomVault {
    pub fn new(id: &str, name: &str, strategy: VaultStrategy) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            strategy,
            total_value_locked: 0,
            apy: 0.0,
            risk_score: 0.5,
            depositors: HashMap::new(),
            ai_managed: true,
            auto_compound: true,
        }
    }

    /// Deposit tokens into vault
    pub fn deposit(&mut self, user: &str, amount: u128) {
        *self.depositors.entry(user.to_string()).or_insert(0) += amount;
        self.total_value_locked += amount;
    }

    /// Withdraw tokens from vault
    pub fn withdraw(&mut self, user: &str, amount: u128) -> Result<u128, String> {
        let balance = self.depositors.get(user).copied().unwrap_or(0);
        if balance < amount {
            return Err(format!("Insufficient balance: {} < {}", balance, amount));
        }
        *self.depositors.get_mut(user).unwrap() -= amount;
        self.total_value_locked -= amount;
        Ok(amount)
    }

    /// Get user's share of the vault
    pub fn user_share(&self, user: &str) -> f64 {
        if self.total_value_locked == 0 {
            return 0.0;
        }
        let balance = self.depositors.get(user).copied().unwrap_or(0);
        balance as f64 / self.total_value_locked as f64
    }
}

/// Vault manager
pub struct VaultManager {
    pub vaults: HashMap<String, PhantomVault>,
}

impl VaultManager {
    pub fn new() -> Self {
        Self {
            vaults: HashMap::new(),
        }
    }

    pub fn create_vault(&mut self, vault: PhantomVault) {
        self.vaults.insert(vault.id.clone(), vault);
    }

    pub fn total_tvl(&self) -> u128 {
        self.vaults.values().map(|v| v.total_value_locked).sum()
    }
}
