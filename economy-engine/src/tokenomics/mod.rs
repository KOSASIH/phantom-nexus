//! Predictive Tokenomics Engine
//!
//! AI-driven supply management with dynamic burn/mint mechanisms.

use serde::{Deserialize, Serialize};
use log::info;

/// Token supply state
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenSupply {
    pub total_supply: u128,
    pub circulating_supply: u128,
    pub staked_supply: u128,
    pub burned_total: u128,
    pub minted_total: u128,
    pub target_staking_ratio: f64,
}

/// Market condition detected by AI
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MarketCondition {
    BullMarket { intensity: f64 },
    BearMarket { intensity: f64 },
    Neutral,
    BlackSwan { severity: f64 },
}

/// Tokenomics engine
pub struct TokenomicsEngine {
    pub supply: TokenSupply,
    pub burn_rate: f64,
    pub emission_rate: f64,
    pub epoch: u64,
    pub halving_interval: u64,
}

impl TokenomicsEngine {
    pub fn new() -> Self {
        Self {
            supply: TokenSupply {
                total_supply: 1_000_000_000 * 10u128.pow(18),  // 1B PHNX
                circulating_supply: 300_000_000 * 10u128.pow(18),
                staked_supply: 0,
                burned_total: 0,
                minted_total: 0,
                target_staking_ratio: 0.6,
            },
            burn_rate: 0.001,   // 0.1% base burn rate
            emission_rate: 0.005, // 0.5% base emission
            epoch: 0,
            halving_interval: 730, // ~2 years in epochs
        }
    }

    /// Adjust supply based on AI-detected market conditions
    pub fn adjust_supply(&mut self, condition: &MarketCondition) {
        match condition {
            MarketCondition::BullMarket { intensity } => {
                // Increase burn rate during bull markets
                self.burn_rate = 0.001 + (intensity * 0.01);
                let burn_amount = (self.supply.circulating_supply as f64 * self.burn_rate) as u128;
                self.burn(burn_amount);
                info!("Bull market burn: {} PHNX (rate: {:.4}%)", burn_amount, self.burn_rate * 100.0);
            }
            MarketCondition::BearMarket { intensity } => {
                // Reduce burn, maintain liquidity
                self.burn_rate = (0.001 - intensity * 0.0005).max(0.0001);
                info!("Bear market: reduced burn rate to {:.4}%", self.burn_rate * 100.0);
            }
            MarketCondition::BlackSwan { severity } => {
                // Emergency supply adjustment
                self.burn_rate = 0.0;
                info!("Black swan event (severity: {:.2}): burn suspended", severity);
            }
            MarketCondition::Neutral => {
                self.burn_rate = 0.001;
            }
        }
    }

    /// Burn tokens
    pub fn burn(&mut self, amount: u128) {
        let actual = amount.min(self.supply.circulating_supply);
        self.supply.circulating_supply -= actual;
        self.supply.total_supply -= actual;
        self.supply.burned_total += actual;
    }

    /// Emit staking rewards
    pub fn emit_rewards(&mut self) -> u128 {
        let halving_factor = 2u128.pow((self.epoch / self.halving_interval) as u32);
        let base_reward = (self.supply.total_supply as f64 * self.emission_rate) as u128;
        let reward = base_reward / halving_factor;

        self.supply.circulating_supply += reward;
        self.supply.minted_total += reward;
        self.epoch += 1;

        reward
    }

    /// Get current staking ratio
    pub fn staking_ratio(&self) -> f64 {
        if self.supply.circulating_supply == 0 {
            return 0.0;
        }
        self.supply.staked_supply as f64 / self.supply.circulating_supply as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenomics_creation() {
        let engine = TokenomicsEngine::new();
        assert_eq!(engine.supply.total_supply, 1_000_000_000 * 10u128.pow(18));
    }

    #[test]
    fn test_burn() {
        let mut engine = TokenomicsEngine::new();
        let before = engine.supply.total_supply;
        engine.burn(1_000_000);
        assert!(engine.supply.total_supply < before);
    }
}
