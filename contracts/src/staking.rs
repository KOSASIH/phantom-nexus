//! Staking Contract
//!
//! PHNX token staking for network security and rewards.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StakeInfo {
    pub amount: u128,
    pub start_time: u64,
    pub lock_until: u64,
    pub rewards_earned: u128,
    pub rewards_claimed: u128,
}

/// Staking contract
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StakingContract {
    pub total_staked: u128,
    pub reward_rate: f64,  // Rewards per second per token
    pub stakes: HashMap<String, StakeInfo>,
    pub min_stake: u128,
    pub lock_period: u64,   // Minimum lock in seconds
}

impl StakingContract {
    pub fn new(reward_rate: f64, min_stake: u128, lock_period: u64) -> Self {
        Self {
            total_staked: 0,
            reward_rate,
            stakes: HashMap::new(),
            min_stake,
            lock_period,
        }
    }

    /// Stake tokens
    pub fn stake(&mut self, user: &str, amount: u128) -> Result<(), String> {
        if amount < self.min_stake {
            return Err(format!("Below minimum stake: {}", self.min_stake));
        }

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let stake = self.stakes.entry(user.to_string()).or_insert(StakeInfo {
            amount: 0,
            start_time: now,
            lock_until: now + self.lock_period,
            rewards_earned: 0,
            rewards_claimed: 0,
        });

        stake.amount += amount;
        stake.lock_until = now + self.lock_period;
        self.total_staked += amount;

        Ok(())
    }

    /// Unstake tokens
    pub fn unstake(&mut self, user: &str, amount: u128) -> Result<u128, String> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let stake = self.stakes.get_mut(user)
            .ok_or("No stake found")?;

        if now < stake.lock_until {
            return Err("Stake is still locked".to_string());
        }

        if stake.amount < amount {
            return Err(format!("Insufficient stake: {} < {}", stake.amount, amount));
        }

        stake.amount -= amount;
        self.total_staked -= amount;

        Ok(amount)
    }

    /// Calculate pending rewards
    pub fn pending_rewards(&self, user: &str) -> u128 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if let Some(stake) = self.stakes.get(user) {
            let duration = now - stake.start_time;
            let rewards = (stake.amount as f64 * self.reward_rate * duration as f64) as u128;
            rewards - stake.rewards_claimed
        } else {
            0
        }
    }
}
