//! Core types for Nexus Chain

use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};
use std::fmt;

/// Block height type
pub type BlockHeight = u64;

/// Transaction hash
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct TxHash(pub [u8; 32]);

impl fmt::Display for TxHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{}", hex::encode(self.0))
    }
}

/// Address on Nexus Chain (32 bytes, quantum-safe)
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Address(pub [u8; 32]);

impl Address {
    pub fn from_public_key(pubkey: &[u8]) -> Self {
        let mut hasher = Sha3_256::new();
        hasher.update(pubkey);
        let result = hasher.finalize();
        let mut addr = [0u8; 32];
        addr.copy_from_slice(&result);
        Address(addr)
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{}", hex::encode(self.0))
    }
}

/// Transaction on Nexus Chain
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub hash: TxHash,
    pub from: Address,
    pub to: Address,
    pub value: u128,
    pub data: Vec<u8>,
    pub nonce: u64,
    pub gas_limit: u64,
    pub gas_price: u64,
    pub signature: Vec<u8>,
    pub timestamp: u64,
}

impl Transaction {
    /// Compute transaction hash
    pub fn compute_hash(&self) -> TxHash {
        let mut hasher = Sha3_256::new();
        hasher.update(&self.from.0);
        hasher.update(&self.to.0);
        hasher.update(&self.value.to_le_bytes());
        hasher.update(&self.data);
        hasher.update(&self.nonce.to_le_bytes());
        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        TxHash(hash)
    }
}

/// Block header
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlockHeader {
    pub height: BlockHeight,
    pub previous_hash: [u8; 32],
    pub state_root: [u8; 32],
    pub transactions_root: [u8; 32],
    pub timestamp: u64,
    pub proposer: Address,
    pub shard_id: u32,
    pub signature: Vec<u8>,
}

/// Block containing header and transactions
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

impl Block {
    /// Compute block hash
    pub fn hash(&self) -> [u8; 32] {
        let mut hasher = Sha3_256::new();
        hasher.update(&self.header.height.to_le_bytes());
        hasher.update(&self.header.previous_hash);
        hasher.update(&self.header.state_root);
        hasher.update(&self.header.transactions_root);
        hasher.update(&self.header.timestamp.to_le_bytes());
        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        hash
    }

    /// Validate block integrity
    pub fn validate(&self) -> bool {
        // Verify transactions root
        let computed_root = self.compute_transactions_root();
        computed_root == self.header.transactions_root
    }

    fn compute_transactions_root(&self) -> [u8; 32] {
        let mut hasher = Sha3_256::new();
        for tx in &self.transactions {
            hasher.update(&tx.hash.0);
        }
        let result = hasher.finalize();
        let mut root = [0u8; 32];
        root.copy_from_slice(&result);
        root
    }
}
