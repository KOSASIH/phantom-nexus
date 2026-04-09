//! PhantomBFT - AI-Enhanced Byzantine Fault Tolerant Consensus
//!
//! A novel consensus mechanism that combines classical BFT with AI-based
//! validator scoring for optimal block production.

use crate::types::{Address, Block, BlockHeader, BlockHeight};
use async_trait::async_trait;
use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConsensusError {
    #[error("insufficient validators: need {required}, have {available}")]
    InsufficientValidators { required: usize, available: usize },
    #[error("invalid block proposal from {proposer}")]
    InvalidProposal { proposer: String },
    #[error("double voting detected from {validator}")]
    DoubleVoting { validator: String },
    #[error("quorum not reached: {votes}/{required}")]
    QuorumNotReached { votes: usize, required: usize },
    #[error("timeout during consensus round {round}")]
    Timeout { round: u64 },
}

/// Validator information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Validator {
    pub address: Address,
    pub stake: u128,
    pub ai_score: f64,  // AI-computed reliability score (0.0 - 1.0)
    pub uptime: f64,
    pub blocks_produced: u64,
    pub slashing_count: u32,
}

/// Consensus round state
#[derive(Clone, Debug, PartialEq)]
pub enum RoundState {
    Propose,
    PreVote,
    PreCommit,
    Commit,
    Finalized,
}

/// Vote in the consensus process
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Vote {
    pub validator: Address,
    pub block_hash: [u8; 32],
    pub round: u64,
    pub vote_type: VoteType,
    pub signature: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum VoteType {
    PreVote,
    PreCommit,
}

/// PhantomBFT Consensus Engine
pub struct PhantomBFT {
    validators: Vec<Validator>,
    current_round: u64,
    current_height: BlockHeight,
    state: RoundState,
    votes: HashMap<[u8; 32], Vec<Vote>>,
    finality_threshold: f64,  // Default: 2/3 + 1
}

impl PhantomBFT {
    pub fn new(validators: Vec<Validator>) -> Self {
        Self {
            validators,
            current_round: 0,
            current_height: 0,
            state: RoundState::Propose,
            votes: HashMap::new(),
            finality_threshold: 0.667,
        }
    }

    /// Select block proposer using AI-weighted stake scoring
    pub fn select_proposer(&self, height: BlockHeight) -> Option<&Validator> {
        if self.validators.is_empty() {
            return None;
        }

        // AI-weighted selection: combine stake weight with AI reliability score
        let total_weight: f64 = self.validators.iter()
            .map(|v| (v.stake as f64) * v.ai_score * v.uptime)
            .sum();

        let selection_point = (height as f64 * std::f64::consts::PI).sin().abs() * total_weight;

        let mut cumulative = 0.0;
        for validator in &self.validators {
            cumulative += (validator.stake as f64) * validator.ai_score * validator.uptime;
            if cumulative >= selection_point {
                return Some(validator);
            }
        }

        self.validators.last()
    }

    /// Process a block proposal
    pub fn propose_block(&mut self, block: &Block) -> Result<(), ConsensusError> {
        let min_validators = (self.validators.len() * 2 / 3) + 1;
        if self.validators.len() < 4 {
            return Err(ConsensusError::InsufficientValidators {
                required: 4,
                available: self.validators.len(),
            });
        }

        // Verify proposer is valid
        let proposer = self.select_proposer(block.header.height);
        if proposer.is_none() || proposer.unwrap().address != block.header.proposer {
            return Err(ConsensusError::InvalidProposal {
                proposer: format!("{}", block.header.proposer),
            });
        }

        info!("Block proposed at height {} by {}", block.header.height, block.header.proposer);
        self.state = RoundState::PreVote;
        Ok(())
    }

    /// Process a vote
    pub fn process_vote(&mut self, vote: Vote) -> Result<Option<Block>, ConsensusError> {
        let entry = self.votes.entry(vote.block_hash).or_insert_with(Vec::new);

        // Check for double voting
        if entry.iter().any(|v| v.validator == vote.validator && v.vote_type == VoteType::PreVote && vote.vote_type == VoteType::PreVote) {
            return Err(ConsensusError::DoubleVoting {
                validator: format!("{}", vote.validator),
            });
        }

        entry.push(vote);

        // Check if quorum reached
        let quorum_size = (self.validators.len() * 2 / 3) + 1;
        let prevote_count = entry.iter().filter(|v| matches!(v.vote_type, VoteType::PreVote)).count();

        if prevote_count >= quorum_size && self.state == RoundState::PreVote {
            info!("PreVote quorum reached: {}/{}", prevote_count, quorum_size);
            self.state = RoundState::PreCommit;
        }

        let precommit_count = entry.iter().filter(|v| matches!(v.vote_type, VoteType::PreCommit)).count();
        if precommit_count >= quorum_size && self.state == RoundState::PreCommit {
            info!("PreCommit quorum reached: {}/{}", precommit_count, quorum_size);
            self.state = RoundState::Finalized;
            self.current_height += 1;
            self.current_round = 0;
        }

        Ok(None)
    }

    /// Get current consensus state
    pub fn state(&self) -> &RoundState {
        &self.state
    }

    /// Get current block height
    pub fn height(&self) -> BlockHeight {
        self.current_height
    }

    /// Add a new validator
    pub fn add_validator(&mut self, validator: Validator) {
        self.validators.push(validator);
    }

    /// Update AI scores for all validators
    pub fn update_ai_scores(&mut self, scores: HashMap<Address, f64>) {
        for validator in &mut self.validators {
            if let Some(&score) = scores.get(&validator.address) {
                validator.ai_score = score.clamp(0.0, 1.0);
                debug!("Updated AI score for {}: {:.4}", validator.address, validator.ai_score);
            }
        }
    }
}

impl PartialEq for VoteType {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (VoteType::PreVote, VoteType::PreVote) | (VoteType::PreCommit, VoteType::PreCommit)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_validators(count: usize) -> Vec<Validator> {
        (0..count)
            .map(|i| {
                let mut addr = [0u8; 32];
                addr[0] = i as u8;
                Validator {
                    address: Address(addr),
                    stake: 1_000_000 * (i as u128 + 1),
                    ai_score: 0.9 + (i as f64 * 0.01),
                    uptime: 0.99,
                    blocks_produced: 100 + i as u64,
                    slashing_count: 0,
                }
            })
            .collect()
    }

    #[test]
    fn test_proposer_selection() {
        let validators = mock_validators(10);
        let bft = PhantomBFT::new(validators);
        let proposer = bft.select_proposer(1);
        assert!(proposer.is_some());
    }

    #[test]
    fn test_insufficient_validators() {
        let validators = mock_validators(2);
        let mut bft = PhantomBFT::new(validators);
        let block = Block {
            header: BlockHeader {
                height: 1,
                previous_hash: [0u8; 32],
                state_root: [0u8; 32],
                transactions_root: [0u8; 32],
                timestamp: 0,
                proposer: Address([0u8; 32]),
                shard_id: 0,
                signature: vec![],
            },
            transactions: vec![],
        };
        assert!(bft.propose_block(&block).is_err());
    }
}
