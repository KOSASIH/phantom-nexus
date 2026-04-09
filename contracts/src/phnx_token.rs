//! $PHNX Token Contract
//!
//! NXS-20 standard token with AI-controlled supply mechanics.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Token metadata
pub const TOKEN_NAME: &str = "Phantom Nexus Token";
pub const TOKEN_SYMBOL: &str = "PHNX";
pub const TOKEN_DECIMALS: u8 = 18;
pub const INITIAL_SUPPLY: u128 = 1_000_000_000; // 1 billion

/// Transfer event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransferEvent {
    pub from: String,
    pub to: String,
    pub amount: u128,
    pub timestamp: u64,
}

/// Approval event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApprovalEvent {
    pub owner: String,
    pub spender: String,
    pub amount: u128,
}

/// PHNX Token Contract State
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PHNXToken {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: u128,
    pub balances: HashMap<String, u128>,
    pub allowances: HashMap<String, HashMap<String, u128>>,
    pub owner: String,
    pub paused: bool,
    pub burn_address: String,
}

impl PHNXToken {
    /// Initialize the token contract
    pub fn new(owner: &str) -> Self {
        let mut balances = HashMap::new();
        let total = INITIAL_SUPPLY * 10u128.pow(TOKEN_DECIMALS as u32);
        balances.insert(owner.to_string(), total);

        Self {
            name: TOKEN_NAME.to_string(),
            symbol: TOKEN_SYMBOL.to_string(),
            decimals: TOKEN_DECIMALS,
            total_supply: total,
            balances,
            allowances: HashMap::new(),
            owner: owner.to_string(),
            paused: false,
            burn_address: "0x0000000000000000000000000000000000000000".to_string(),
        }
    }

    /// Get balance of an address
    pub fn balance_of(&self, address: &str) -> u128 {
        self.balances.get(address).copied().unwrap_or(0)
    }

    /// Transfer tokens
    pub fn transfer(&mut self, from: &str, to: &str, amount: u128) -> Result<TransferEvent, String> {
        if self.paused {
            return Err("Contract is paused".to_string());
        }

        let balance = self.balance_of(from);
        if balance < amount {
            return Err(format!("Insufficient balance: {} < {}", balance, amount));
        }

        *self.balances.entry(from.to_string()).or_insert(0) -= amount;
        *self.balances.entry(to.to_string()).or_insert(0) += amount;

        Ok(TransferEvent {
            from: from.to_string(),
            to: to.to_string(),
            amount,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        })
    }

    /// Approve spender
    pub fn approve(&mut self, owner: &str, spender: &str, amount: u128) -> ApprovalEvent {
        self.allowances
            .entry(owner.to_string())
            .or_insert_with(HashMap::new)
            .insert(spender.to_string(), amount);

        ApprovalEvent {
            owner: owner.to_string(),
            spender: spender.to_string(),
            amount,
        }
    }

    /// Transfer from (with allowance)
    pub fn transfer_from(&mut self, spender: &str, from: &str, to: &str, amount: u128) -> Result<TransferEvent, String> {
        let allowance = self.allowances
            .get(from)
            .and_then(|a| a.get(spender))
            .copied()
            .unwrap_or(0);

        if allowance < amount {
            return Err(format!("Insufficient allowance: {} < {}", allowance, amount));
        }

        // Decrease allowance
        *self.allowances
            .get_mut(from).unwrap()
            .get_mut(spender).unwrap() -= amount;

        self.transfer(from, to, amount)
    }

    /// Burn tokens (deflationary)
    pub fn burn(&mut self, from: &str, amount: u128) -> Result<u128, String> {
        let balance = self.balance_of(from);
        if balance < amount {
            return Err(format!("Insufficient balance to burn: {} < {}", balance, amount));
        }

        *self.balances.get_mut(from).unwrap() -= amount;
        self.total_supply -= amount;

        Ok(amount)
    }

    /// Mint new tokens (only by AI governance)
    pub fn mint(&mut self, to: &str, amount: u128, caller: &str) -> Result<u128, String> {
        if caller != self.owner {
            return Err("Only owner can mint".to_string());
        }

        *self.balances.entry(to.to_string()).or_insert(0) += amount;
        self.total_supply += amount;

        Ok(amount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_creation() {
        let token = PHNXToken::new("owner");
        assert_eq!(token.symbol, "PHNX");
        assert_eq!(token.balance_of("owner"), INITIAL_SUPPLY * 10u128.pow(18));
    }

    #[test]
    fn test_transfer() {
        let mut token = PHNXToken::new("alice");
        let result = token.transfer("alice", "bob", 1000);
        assert!(result.is_ok());
        assert_eq!(token.balance_of("bob"), 1000);
    }

    #[test]
    fn test_insufficient_balance() {
        let mut token = PHNXToken::new("alice");
        let result = token.transfer("bob", "alice", 1000);
        assert!(result.is_err());
    }

    #[test]
    fn test_burn() {
        let mut token = PHNXToken::new("alice");
        let before = token.total_supply;
        token.burn("alice", 1000).unwrap();
        assert_eq!(token.total_supply, before - 1000);
    }
}
