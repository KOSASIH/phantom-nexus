//! Phantom Vault Contract
//!
//! DeFi 2.0 vault with AI-optimized yield strategies.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VaultConfig {
    pub name: String,
    pub deposit_token: String,
    pub min_deposit: u128,
    pub max_deposit: u128,
    pub lock_period_seconds: u64,
    pub performance_fee: f64,   // e.g., 0.02 = 2%
    pub management_fee: f64,    // e.g., 0.005 = 0.5%
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserDeposit {
    pub amount: u128,
    pub shares: u128,
    pub deposit_time: u64,
    pub rewards_claimed: u128,
}

/// Phantom Vault Contract
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PhantomVaultContract {
    pub config: VaultConfig,
    pub total_shares: u128,
    pub total_assets: u128,
    pub deposits: HashMap<String, UserDeposit>,
    pub price_per_share: f64,
    pub cumulative_yield: f64,
    pub ai_strategy_active: bool,
}

impl PhantomVaultContract {
    pub fn new(config: VaultConfig) -> Self {
        Self {
            config,
            total_shares: 0,
            total_assets: 0,
            deposits: HashMap::new(),
            price_per_share: 1.0,
            cumulative_yield: 0.0,
            ai_strategy_active: true,
        }
    }

    /// Deposit tokens and receive shares
    pub fn deposit(&mut self, user: &str, amount: u128) -> Result<u128, String> {
        if amount < self.config.min_deposit {
            return Err(format!("Below minimum deposit: {}", self.config.min_deposit));
        }
        if amount > self.config.max_deposit {
            return Err(format!("Exceeds maximum deposit: {}", self.config.max_deposit));
        }

        let shares = if self.total_shares == 0 {
            amount
        } else {
            (amount as f64 / self.price_per_share) as u128
        };

        let deposit = self.deposits.entry(user.to_string()).or_insert(UserDeposit {
            amount: 0,
            shares: 0,
            deposit_time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            rewards_claimed: 0,
        });

        deposit.amount += amount;
        deposit.shares += shares;
        self.total_shares += shares;
        self.total_assets += amount;

        Ok(shares)
    }

    /// Withdraw by burning shares
    pub fn withdraw(&mut self, user: &str, shares: u128) -> Result<u128, String> {
        let deposit = self.deposits.get_mut(user)
            .ok_or("No deposit found")?;

        if deposit.shares < shares {
            return Err(format!("Insufficient shares: {} < {}", deposit.shares, shares));
        }

        let amount = (shares as f64 * self.price_per_share) as u128;

        // Apply performance fee on profits
        let original_value = (shares as f64 / deposit.shares as f64 * deposit.amount as f64) as u128;
        let profit = amount.saturating_sub(original_value);
        let fee = (profit as f64 * self.config.performance_fee) as u128;
        let net_amount = amount - fee;

        deposit.shares -= shares;
        deposit.amount = (deposit.amount as f64 * (deposit.shares as f64 / (deposit.shares + shares) as f64)) as u128;
        self.total_shares -= shares;
        self.total_assets -= net_amount;

        Ok(net_amount)
    }

    /// AI strategy harvests yield and updates share price
    pub fn harvest_yield(&mut self, yield_amount: u128) {
        self.total_assets += yield_amount;
        if self.total_shares > 0 {
            self.price_per_share = self.total_assets as f64 / self.total_shares as f64;
        }
        self.cumulative_yield += yield_amount as f64;
    }

    /// Get current APY estimate
    pub fn estimated_apy(&self) -> f64 {
        if self.total_assets == 0 {
            return 0.0;
        }
        (self.cumulative_yield / self.total_assets as f64) * 365.25 * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> VaultConfig {
        VaultConfig {
            name: "Phantom Yield V1".to_string(),
            deposit_token: "PHNX".to_string(),
            min_deposit: 100,
            max_deposit: 1_000_000_000,
            lock_period_seconds: 86400,
            performance_fee: 0.02,
            management_fee: 0.005,
        }
    }

    #[test]
    fn test_deposit_and_withdraw() {
        let mut vault = PhantomVaultContract::new(test_config());
        let shares = vault.deposit("alice", 10000).unwrap();
        assert_eq!(shares, 10000);
        assert_eq!(vault.total_assets, 10000);

        let withdrawn = vault.withdraw("alice", 5000).unwrap();
        assert!(withdrawn > 0);
    }

    #[test]
    fn test_yield_harvest() {
        let mut vault = PhantomVaultContract::new(test_config());
        vault.deposit("alice", 10000).unwrap();
        vault.harvest_yield(500);
        assert!(vault.price_per_share > 1.0);
    }
}
