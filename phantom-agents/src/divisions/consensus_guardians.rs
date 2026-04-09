//! Consensusguardians Division — 50 agents
//!
//! Mission: Block validation, fork resolution, finality assurance, PhantomBFT coordination

use crate::core::*;
use crate::registry::AgentRegistry;
use super::quantum_security::DivisionAgent;

macro_rules! agent { ($c:expr,$n:expr,$d:expr,$r:expr,$l:expr,$s:expr,$desc:expr) => { Box::new(DivisionAgent::new($c,$n,$d,$r,$l,$s,$desc)) }; }
fn sk(n:&str,c:SkillCategory,p:u8,d:&str)->Skill{Skill{name:n.into(),category:c,proficiency:p,description:d.into()}}

pub fn spawn_consensus_guardians(registry: &AgentRegistry) {
    let d = Division::ConsensusGuardians;
    let agents: Vec<Box<dyn AutonomousAgent>> = vec![
        agent!("CONS-CMD-001","Finality Supreme",d.clone(),AgentRole::DivisionCommander,CognitiveLevel::L6Omniscient,vec![sk("Block Validation",SkillCategory::Consensus,100,"Supreme mastery"),sk("Fork Resolution",SkillCategory::Consensus,100,"Supreme mastery"),sk("Proposer Selection",SkillCategory::Consensus,100,"Supreme mastery")],"Supreme commander of ConsensusGuardians division. Orchestrates all 50 agents for 24/7 operations."),
        agent!("CONS-SQL-001","Block Marshal",d.clone(),AgentRole::SquadLeader,CognitiveLevel::L5Transcendent,vec![sk("Block Validation",SkillCategory::Consensus,98,"Expert leadership")],"Block Marshal: Squad leader for block validation operations."),
        agent!("CONS-SQL-002","Fork Oracle",d.clone(),AgentRole::SquadLeader,CognitiveLevel::L5Transcendent,vec![sk("Fork Resolution",SkillCategory::Consensus,98,"Expert leadership")],"Fork Oracle: Squad leader for fork resolution operations."),
        agent!("CONS-SQL-003","Shard Commander",d.clone(),AgentRole::SquadLeader,CognitiveLevel::L5Transcendent,vec![sk("Proposer Selection",SkillCategory::Consensus,98,"Expert leadership")],"Shard Commander: Squad leader for proposer selection operations."),
        agent!("CONS-SQL-004","Epoch Warden",d.clone(),AgentRole::SquadLeader,CognitiveLevel::L5Transcendent,vec![sk("Finality Assurance",SkillCategory::Consensus,98,"Expert leadership")],"Epoch Warden: Squad leader for finality assurance operations."),
        agent!("CONS-SPC-001","BFT Master",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Block Validation",SkillCategory::Consensus,97,"Deep expertise")],"BFT Master: Specialist in block validation."),
        agent!("CONS-SPC-002","Quorum Keeper",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Fork Resolution",SkillCategory::Consensus,97,"Deep expertise")],"Quorum Keeper: Specialist in fork resolution."),
        agent!("CONS-SPC-003","State Root",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Proposer Selection",SkillCategory::Consensus,97,"Deep expertise")],"State Root: Specialist in proposer selection."),
        agent!("CONS-SPC-004","Mempool Sage",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Finality Assurance",SkillCategory::Consensus,97,"Deep expertise")],"Mempool Sage: Specialist in finality assurance."),
        agent!("CONS-SPC-005","Checkpoint Seal",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Shard Coordination",SkillCategory::Consensus,97,"Deep expertise")],"Checkpoint Seal: Specialist in shard coordination."),
        agent!("CONS-SPC-006","Proposer Brain",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("State Sync",SkillCategory::Consensus,97,"Deep expertise")],"Proposer Brain: Specialist in state sync."),
        agent!("CONS-SPC-007","Validator Eye",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Mempool Management",SkillCategory::Consensus,97,"Deep expertise")],"Validator Eye: Specialist in mempool management."),
        agent!("CONS-SPC-008","Sync Wave",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Slashing Detection",SkillCategory::Consensus,97,"Deep expertise")],"Sync Wave: Specialist in slashing detection."),
        agent!("CONS-SPC-009","Attestation Core",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Epoch Management",SkillCategory::Consensus,97,"Deep expertise")],"Attestation Core: Specialist in epoch management."),
        agent!("CONS-SPC-010","Rollup Weaver",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Checkpoint Verification",SkillCategory::Consensus,97,"Deep expertise")],"Rollup Weaver: Specialist in checkpoint verification."),
        agent!("CONS-ANL-001","Block Timer",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Block Validation",SkillCategory::Consensus,94,"Advanced analysis")],"Block Timer: Analyst for block validation."),
        agent!("CONS-ANL-002","Fork Tracker",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Fork Resolution",SkillCategory::Consensus,94,"Advanced analysis")],"Fork Tracker: Analyst for fork resolution."),
        agent!("CONS-ANL-003","Shard Monitor",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Proposer Selection",SkillCategory::Consensus,94,"Advanced analysis")],"Shard Monitor: Analyst for proposer selection."),
        agent!("CONS-ANL-004","State Diff",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Finality Assurance",SkillCategory::Consensus,94,"Advanced analysis")],"State Diff: Analyst for finality assurance."),
        agent!("CONS-ANL-005","Gas Flow",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Shard Coordination",SkillCategory::Consensus,94,"Advanced analysis")],"Gas Flow: Analyst for shard coordination."),
        agent!("CONS-ANL-006","Slot Watcher",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("State Sync",SkillCategory::Consensus,94,"Advanced analysis")],"Slot Watcher: Analyst for state sync."),
        agent!("CONS-ANL-007","Validator Score",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Mempool Management",SkillCategory::Consensus,94,"Advanced analysis")],"Validator Score: Analyst for mempool management."),
        agent!("CONS-ANL-008","Proof Checker",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Slashing Detection",SkillCategory::Consensus,94,"Advanced analysis")],"Proof Checker: Analyst for slashing detection."),
        agent!("CONS-ANL-009","Epoch Pulse",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Epoch Management",SkillCategory::Consensus,94,"Advanced analysis")],"Epoch Pulse: Analyst for epoch management."),
        agent!("CONS-ANL-010","Chain Health",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Checkpoint Verification",SkillCategory::Consensus,94,"Advanced analysis")],"Chain Health: Analyst for checkpoint verification."),
        agent!("CONS-ANL-011","Block Size",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Block Validation",SkillCategory::Consensus,94,"Advanced analysis")],"Block Size: Analyst for block validation."),
        agent!("CONS-ANL-012","Uncle Rate",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Fork Resolution",SkillCategory::Consensus,94,"Advanced analysis")],"Uncle Rate: Analyst for fork resolution."),
        agent!("CONS-ANL-013","Reorg Detector",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Proposer Selection",SkillCategory::Consensus,94,"Advanced analysis")],"Reorg Detector: Analyst for proposer selection."),
        agent!("CONS-ANL-014","Tx Throughput",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Finality Assurance",SkillCategory::Consensus,94,"Advanced analysis")],"Tx Throughput: Analyst for finality assurance."),
        agent!("CONS-ANL-015","Queue Depth",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Shard Coordination",SkillCategory::Consensus,94,"Advanced analysis")],"Queue Depth: Analyst for shard coordination."),
        agent!("CONS-EXE-001","Block Builder",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Block Validation",SkillCategory::Consensus,95,"Precision execution")],"Block Builder: Executes block validation tasks."),
        agent!("CONS-EXE-002","Fork Resolver",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Fork Resolution",SkillCategory::Consensus,95,"Precision execution")],"Fork Resolver: Executes fork resolution tasks."),
        agent!("CONS-EXE-003","Shard Splitter",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Proposer Selection",SkillCategory::Consensus,95,"Precision execution")],"Shard Splitter: Executes proposer selection tasks."),
        agent!("CONS-EXE-004","State Committer",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Finality Assurance",SkillCategory::Consensus,95,"Precision execution")],"State Committer: Executes finality assurance tasks."),
        agent!("CONS-EXE-005","Mempool Flush",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Shard Coordination",SkillCategory::Consensus,95,"Precision execution")],"Mempool Flush: Executes shard coordination tasks."),
        agent!("CONS-EXE-006","Slash Executor",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("State Sync",SkillCategory::Consensus,95,"Precision execution")],"Slash Executor: Executes state sync tasks."),
        agent!("CONS-EXE-007","Epoch Roller",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Mempool Management",SkillCategory::Consensus,95,"Precision execution")],"Epoch Roller: Executes mempool management tasks."),
        agent!("CONS-EXE-008","Sync Pusher",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Slashing Detection",SkillCategory::Consensus,95,"Precision execution")],"Sync Pusher: Executes slashing detection tasks."),
        agent!("CONS-EXE-009","Attestation Submit",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Epoch Management",SkillCategory::Consensus,95,"Precision execution")],"Attestation Submit: Executes epoch management tasks."),
        agent!("CONS-EXE-010","Proposal Sender",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Checkpoint Verification",SkillCategory::Consensus,95,"Precision execution")],"Proposal Sender: Executes checkpoint verification tasks."),
        agent!("CONS-SCT-001","Chain Scout",d.clone(),AgentRole::Scout,CognitiveLevel::L3Adaptive,vec![sk("Block Validation",SkillCategory::Consensus,90,"Reconnaissance")],"Chain Scout: Scout for block validation opportunities."),
        agent!("CONS-SCT-002","Fork Scout",d.clone(),AgentRole::Scout,CognitiveLevel::L3Adaptive,vec![sk("Fork Resolution",SkillCategory::Consensus,90,"Reconnaissance")],"Fork Scout: Scout for fork resolution opportunities."),
        agent!("CONS-SCT-003","Latency Probe",d.clone(),AgentRole::Scout,CognitiveLevel::L3Adaptive,vec![sk("Proposer Selection",SkillCategory::Consensus,90,"Reconnaissance")],"Latency Probe: Scout for proposer selection opportunities."),
        agent!("CONS-SCT-004","Peer Scout",d.clone(),AgentRole::Scout,CognitiveLevel::L3Adaptive,vec![sk("Finality Assurance",SkillCategory::Consensus,90,"Reconnaissance")],"Peer Scout: Scout for finality assurance opportunities."),
        agent!("CONS-SCT-005","Sync Scout",d.clone(),AgentRole::Scout,CognitiveLevel::L3Adaptive,vec![sk("Shard Coordination",SkillCategory::Consensus,90,"Reconnaissance")],"Sync Scout: Scout for shard coordination opportunities."),
        agent!("CONS-GRD-001","Chain Shield",d.clone(),AgentRole::Guardian,CognitiveLevel::L4Creative,vec![sk("Block Validation",SkillCategory::Consensus,96,"Protective mastery")],"Chain Shield: Guards block validation systems."),
        agent!("CONS-GRD-002","Fork Guard",d.clone(),AgentRole::Guardian,CognitiveLevel::L4Creative,vec![sk("Fork Resolution",SkillCategory::Consensus,96,"Protective mastery")],"Fork Guard: Guards fork resolution systems."),
        agent!("CONS-GRD-003","Shard Sentinel",d.clone(),AgentRole::Guardian,CognitiveLevel::L4Creative,vec![sk("Proposer Selection",SkillCategory::Consensus,96,"Protective mastery")],"Shard Sentinel: Guards proposer selection systems."),
        agent!("CONS-GRD-004","State Guardian",d.clone(),AgentRole::Guardian,CognitiveLevel::L4Creative,vec![sk("Finality Assurance",SkillCategory::Consensus,96,"Protective mastery")],"State Guardian: Guards finality assurance systems."),
        agent!("CONS-GRD-005","Epoch Protector",d.clone(),AgentRole::Guardian,CognitiveLevel::L4Creative,vec![sk("Shard Coordination",SkillCategory::Consensus,96,"Protective mastery")],"Epoch Protector: Guards shard coordination systems."),
    ];
    for agent in agents { registry.register(agent); }
    log::info!("ConsensusGuardians Division: 50 agents deployed");
}
