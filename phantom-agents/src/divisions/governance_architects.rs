//! Governancearchitects Division — 50 agents
//!
//! Mission: Proposal analysis, voting optimization, policy simulation, treasury management

use crate::core::*;
use crate::registry::AgentRegistry;
use super::quantum_security::DivisionAgent;

macro_rules! agent { ($c:expr,$n:expr,$d:expr,$r:expr,$l:expr,$s:expr,$desc:expr) => { Box::new(DivisionAgent::new($c,$n,$d,$r,$l,$s,$desc)) }; }
fn sk(n:&str,c:SkillCategory,p:u8,d:&str)->Skill{Skill{name:n.into(),category:c,proficiency:p,description:d.into()}}

pub fn spawn_governance_architects(registry: &AgentRegistry) {
    let d = Division::GovernanceArchitects;
    let agents: Vec<Box<dyn AutonomousAgent>> = vec![
        agent!("GOVN-CMD-001","DAO Sovereign",d.clone(),AgentRole::DivisionCommander,CognitiveLevel::L6Omniscient,vec![sk("Proposal Analysis",SkillCategory::Governance,100,"Supreme mastery"),sk("Voting Optimization",SkillCategory::Governance,100,"Supreme mastery"),sk("Policy Simulation",SkillCategory::Governance,100,"Supreme mastery")],"Supreme commander of GovernanceArchitects division. Orchestrates all 50 agents for 24/7 operations."),
        agent!("GOVN-SQL-001","Proposal Lead",d.clone(),AgentRole::SquadLeader,CognitiveLevel::L5Transcendent,vec![sk("Proposal Analysis",SkillCategory::Governance,98,"Expert leadership")],"Proposal Lead: Squad leader for proposal analysis operations."),
        agent!("GOVN-SQL-002","Vote Strategist",d.clone(),AgentRole::SquadLeader,CognitiveLevel::L5Transcendent,vec![sk("Voting Optimization",SkillCategory::Governance,98,"Expert leadership")],"Vote Strategist: Squad leader for voting optimization operations."),
        agent!("GOVN-SQL-003","Treasury Chief",d.clone(),AgentRole::SquadLeader,CognitiveLevel::L5Transcendent,vec![sk("Policy Simulation",SkillCategory::Governance,98,"Expert leadership")],"Treasury Chief: Squad leader for policy simulation operations."),
        agent!("GOVN-SQL-004","Policy Architect",d.clone(),AgentRole::SquadLeader,CognitiveLevel::L5Transcendent,vec![sk("Treasury Management",SkillCategory::Governance,98,"Expert leadership")],"Policy Architect: Squad leader for treasury management operations."),
        agent!("GOVN-SPC-001","Sim Engine",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Proposal Analysis",SkillCategory::Governance,97,"Deep expertise")],"Sim Engine: Specialist in proposal analysis."),
        agent!("GOVN-SPC-002","Quorum Master",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Voting Optimization",SkillCategory::Governance,97,"Deep expertise")],"Quorum Master: Specialist in voting optimization."),
        agent!("GOVN-SPC-003","Delegate Manager",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Policy Simulation",SkillCategory::Governance,97,"Deep expertise")],"Delegate Manager: Specialist in policy simulation."),
        agent!("GOVN-SPC-004","Timelock Guardian",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Treasury Management",SkillCategory::Governance,97,"Deep expertise")],"Timelock Guardian: Specialist in treasury management."),
        agent!("GOVN-SPC-005","Multi-Sig Oracle",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Conflict Resolution",SkillCategory::Governance,97,"Deep expertise")],"Multi-Sig Oracle: Specialist in conflict resolution."),
        agent!("GOVN-SPC-006","Veto Analyst",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Quorum Engineering",SkillCategory::Governance,97,"Deep expertise")],"Veto Analyst: Specialist in quorum engineering."),
        agent!("GOVN-SPC-007","Budget Planner",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Proposal Analysis",SkillCategory::Governance,97,"Deep expertise")],"Budget Planner: Specialist in proposal analysis."),
        agent!("GOVN-SPC-008","Amendment Writer",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Voting Optimization",SkillCategory::Governance,97,"Deep expertise")],"Amendment Writer: Specialist in voting optimization."),
        agent!("GOVN-SPC-009","Emergency Gov",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Policy Simulation",SkillCategory::Governance,97,"Deep expertise")],"Emergency Gov: Specialist in policy simulation."),
        agent!("GOVN-SPC-010","Constitution Guard",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Treasury Management",SkillCategory::Governance,97,"Deep expertise")],"Constitution Guard: Specialist in treasury management."),
        agent!("GOVN-ANL-001","Vote Tracker",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Proposal Analysis",SkillCategory::Governance,94,"Advanced analysis")],"Vote Tracker: Analyst for proposal analysis."),
        agent!("GOVN-ANL-002","Quorum Monitor",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Voting Optimization",SkillCategory::Governance,94,"Advanced analysis")],"Quorum Monitor: Analyst for voting optimization."),
        agent!("GOVN-ANL-003","Treasury Auditor",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Policy Simulation",SkillCategory::Governance,94,"Advanced analysis")],"Treasury Auditor: Analyst for policy simulation."),
        agent!("GOVN-ANL-004","Proposal Scorer",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Treasury Management",SkillCategory::Governance,94,"Advanced analysis")],"Proposal Scorer: Analyst for treasury management."),
        agent!("GOVN-ANL-005","Participation Rate",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Conflict Resolution",SkillCategory::Governance,94,"Advanced analysis")],"Participation Rate: Analyst for conflict resolution."),
        agent!("GOVN-ANL-006","Delegation Map",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Quorum Engineering",SkillCategory::Governance,94,"Advanced analysis")],"Delegation Map: Analyst for quorum engineering."),
        agent!("GOVN-ANL-007","Budget Analyst",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Proposal Analysis",SkillCategory::Governance,94,"Advanced analysis")],"Budget Analyst: Analyst for proposal analysis."),
        agent!("GOVN-ANL-008","Timeline Checker",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Voting Optimization",SkillCategory::Governance,94,"Advanced analysis")],"Timeline Checker: Analyst for voting optimization."),
        agent!("GOVN-ANL-009","Conflict Finder",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Policy Simulation",SkillCategory::Governance,94,"Advanced analysis")],"Conflict Finder: Analyst for policy simulation."),
        agent!("GOVN-ANL-010","Impact Assessor",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Treasury Management",SkillCategory::Governance,94,"Advanced analysis")],"Impact Assessor: Analyst for treasury management."),
        agent!("GOVN-ANL-011","Precedent Search",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Conflict Resolution",SkillCategory::Governance,94,"Advanced analysis")],"Precedent Search: Analyst for conflict resolution."),
        agent!("GOVN-ANL-012","Stakeholder Map",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Quorum Engineering",SkillCategory::Governance,94,"Advanced analysis")],"Stakeholder Map: Analyst for quorum engineering."),
        agent!("GOVN-ANL-013","Sentiment Gauge",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Proposal Analysis",SkillCategory::Governance,94,"Advanced analysis")],"Sentiment Gauge: Analyst for proposal analysis."),
        agent!("GOVN-ANL-014","Cost Estimator",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Voting Optimization",SkillCategory::Governance,94,"Advanced analysis")],"Cost Estimator: Analyst for voting optimization."),
        agent!("GOVN-ANL-015","Success Predictor",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Policy Simulation",SkillCategory::Governance,94,"Advanced analysis")],"Success Predictor: Analyst for policy simulation."),
        agent!("GOVN-EXE-001","Vote Caster",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Proposal Analysis",SkillCategory::Governance,95,"Precision execution")],"Vote Caster: Executes proposal analysis tasks."),
        agent!("GOVN-EXE-002","Proposal Submitter",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Voting Optimization",SkillCategory::Governance,95,"Precision execution")],"Proposal Submitter: Executes voting optimization tasks."),
        agent!("GOVN-EXE-003","Treasury Executor",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Policy Simulation",SkillCategory::Governance,95,"Precision execution")],"Treasury Executor: Executes policy simulation tasks."),
        agent!("GOVN-EXE-004","Timelock Trigger",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Treasury Management",SkillCategory::Governance,95,"Precision execution")],"Timelock Trigger: Executes treasury management tasks."),
        agent!("GOVN-EXE-005","Budget Disburser",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Conflict Resolution",SkillCategory::Governance,95,"Precision execution")],"Budget Disburser: Executes conflict resolution tasks."),
        agent!("GOVN-EXE-006","Amendment Applier",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Quorum Engineering",SkillCategory::Governance,95,"Precision execution")],"Amendment Applier: Executes quorum engineering tasks."),
        agent!("GOVN-EXE-007","Election Runner",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Proposal Analysis",SkillCategory::Governance,95,"Precision execution")],"Election Runner: Executes proposal analysis tasks."),
        agent!("GOVN-EXE-008","Delegate Assigner",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Voting Optimization",SkillCategory::Governance,95,"Precision execution")],"Delegate Assigner: Executes voting optimization tasks."),
        agent!("GOVN-EXE-009","Reward Payer",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Policy Simulation",SkillCategory::Governance,95,"Precision execution")],"Reward Payer: Executes policy simulation tasks."),
        agent!("GOVN-EXE-010","Archive Agent",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Treasury Management",SkillCategory::Governance,95,"Precision execution")],"Archive Agent: Executes treasury management tasks."),
        agent!("GOVN-SCT-001","Proposal Scout",d.clone(),AgentRole::Scout,CognitiveLevel::L3Adaptive,vec![sk("Proposal Analysis",SkillCategory::Governance,90,"Reconnaissance")],"Proposal Scout: Scout for proposal analysis opportunities."),
        agent!("GOVN-SCT-002","Delegate Scout",d.clone(),AgentRole::Scout,CognitiveLevel::L3Adaptive,vec![sk("Voting Optimization",SkillCategory::Governance,90,"Reconnaissance")],"Delegate Scout: Scout for voting optimization opportunities."),
        agent!("GOVN-SCT-003","Community Pulse",d.clone(),AgentRole::Scout,CognitiveLevel::L3Adaptive,vec![sk("Policy Simulation",SkillCategory::Governance,90,"Reconnaissance")],"Community Pulse: Scout for policy simulation opportunities."),
        agent!("GOVN-SCT-004","Trend Scout",d.clone(),AgentRole::Scout,CognitiveLevel::L3Adaptive,vec![sk("Treasury Management",SkillCategory::Governance,90,"Reconnaissance")],"Trend Scout: Scout for treasury management opportunities."),
        agent!("GOVN-SCT-005","Opposition Scout",d.clone(),AgentRole::Scout,CognitiveLevel::L3Adaptive,vec![sk("Conflict Resolution",SkillCategory::Governance,90,"Reconnaissance")],"Opposition Scout: Scout for conflict resolution opportunities."),
        agent!("GOVN-GRD-001","DAO Shield",d.clone(),AgentRole::Guardian,CognitiveLevel::L4Creative,vec![sk("Proposal Analysis",SkillCategory::Governance,96,"Protective mastery")],"DAO Shield: Guards proposal analysis systems."),
        agent!("GOVN-GRD-002","Treasury Guard",d.clone(),AgentRole::Guardian,CognitiveLevel::L4Creative,vec![sk("Voting Optimization",SkillCategory::Governance,96,"Protective mastery")],"Treasury Guard: Guards voting optimization systems."),
        agent!("GOVN-GRD-003","Vote Integrity",d.clone(),AgentRole::Guardian,CognitiveLevel::L4Creative,vec![sk("Policy Simulation",SkillCategory::Governance,96,"Protective mastery")],"Vote Integrity: Guards policy simulation systems."),
        agent!("GOVN-GRD-004","Governance Sentinel",d.clone(),AgentRole::Guardian,CognitiveLevel::L4Creative,vec![sk("Treasury Management",SkillCategory::Governance,96,"Protective mastery")],"Governance Sentinel: Guards treasury management systems."),
        agent!("GOVN-GRD-005","Constitution Guard",d.clone(),AgentRole::Guardian,CognitiveLevel::L4Creative,vec![sk("Conflict Resolution",SkillCategory::Governance,96,"Protective mastery")],"Constitution Guard: Guards conflict resolution systems."),
    ];
    for agent in agents { registry.register(agent); }
    log::info!("GovernanceArchitects Division: 50 agents deployed");
}
