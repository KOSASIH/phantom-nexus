//! Hantu DAO - AI-Oracle Governance
//!
//! Proposals auto-approved if AI predicts >95% success probability.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use log::info;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ProposalType {
    ParameterChange { parameter: String, new_value: String },
    TreasurySpend { amount: u128, recipient: String, purpose: String },
    ProtocolUpgrade { version: String, description: String },
    EmergencyAction { description: String },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ProposalStatus {
    Pending,
    AIEvaluating,
    AutoApproved { confidence: f64 },
    VotingOpen { end_time: u64 },
    Passed,
    Rejected,
    Executed,
    Cancelled,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Proposal {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub proposal_type: ProposalType,
    pub proposer: String,
    pub status: ProposalStatus,
    pub ai_success_probability: f64,
    pub votes_for: u128,
    pub votes_against: u128,
    pub created_at: u64,
}

/// Hantu DAO
pub struct HantuDAO {
    pub proposals: Vec<Proposal>,
    pub next_id: u64,
    pub auto_approve_threshold: f64,  // Default: 0.95
    pub super_majority: f64,          // Default: 0.67
    pub token_holders: HashMap<String, u128>,
}

impl HantuDAO {
    pub fn new() -> Self {
        Self {
            proposals: Vec::new(),
            next_id: 1,
            auto_approve_threshold: 0.95,
            super_majority: 0.67,
            token_holders: HashMap::new(),
        }
    }

    /// Submit a new proposal
    pub fn submit_proposal(
        &mut self,
        title: &str,
        description: &str,
        proposal_type: ProposalType,
        proposer: &str,
    ) -> &Proposal {
        let proposal = Proposal {
            id: self.next_id,
            title: title.to_string(),
            description: description.to_string(),
            proposal_type,
            proposer: proposer.to_string(),
            status: ProposalStatus::AIEvaluating,
            ai_success_probability: 0.0,
            votes_for: 0,
            votes_against: 0,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };

        self.proposals.push(proposal);
        self.next_id += 1;
        self.proposals.last().unwrap()
    }

    /// AI evaluates proposal success probability
    pub fn ai_evaluate(&mut self, proposal_id: u64, success_probability: f64) {
        if let Some(proposal) = self.proposals.iter_mut().find(|p| p.id == proposal_id) {
            proposal.ai_success_probability = success_probability;

            if success_probability >= self.auto_approve_threshold {
                proposal.status = ProposalStatus::AutoApproved {
                    confidence: success_probability,
                };
                info!("Proposal #{} auto-approved (AI confidence: {:.2}%)",
                    proposal_id, success_probability * 100.0);
            } else {
                proposal.status = ProposalStatus::VotingOpen {
                    end_time: proposal.created_at + 7 * 24 * 3600, // 7 days
                };
                info!("Proposal #{} requires voting (AI confidence: {:.2}%)",
                    proposal_id, success_probability * 100.0);
            }
        }
    }

    /// Cast vote on a proposal
    pub fn vote(&mut self, proposal_id: u64, voter: &str, support: bool) -> Result<(), String> {
        let voting_power = self.token_holders.get(voter).copied().unwrap_or(0);
        if voting_power == 0 {
            return Err("No voting power".to_string());
        }

        if let Some(proposal) = self.proposals.iter_mut().find(|p| p.id == proposal_id) {
            if support {
                proposal.votes_for += voting_power;
            } else {
                proposal.votes_against += voting_power;
            }
            Ok(())
        } else {
            Err("Proposal not found".to_string())
        }
    }
}
