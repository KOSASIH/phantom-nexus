//! Intelligenceharvesters Division — 50 agents
//!
//! Mission: On-chain analytics, social sentiment, macro indicators, alpha discovery

use crate::core::*;
use crate::registry::AgentRegistry;
use super::quantum_security::DivisionAgent;

macro_rules! agent { ($c:expr,$n:expr,$d:expr,$r:expr,$l:expr,$s:expr,$desc:expr) => { Box::new(DivisionAgent::new($c,$n,$d,$r,$l,$s,$desc)) }; }
fn sk(n:&str,c:SkillCategory,p:u8,d:&str)->Skill{Skill{name:n.into(),category:c,proficiency:p,description:d.into()}}

pub fn spawn_intelligence_harvesters(registry: &AgentRegistry) {
    let d = Division::IntelligenceHarvesters;
    let agents: Vec<Box<dyn AutonomousAgent>> = vec![
        agent!("INTL-CMD-001","Intel Supreme",d.clone(),AgentRole::DivisionCommander,CognitiveLevel::L6Omniscient,vec![sk("On-Chain Analytics",SkillCategory::Intelligence,100,"Supreme mastery"),sk("Macro Analysis",SkillCategory::Intelligence,100,"Supreme mastery"),sk("Competitor Intel",SkillCategory::Intelligence,100,"Supreme mastery")],"Supreme commander of IntelligenceHarvesters division. Orchestrates all 50 agents for 24/7 operations."),
        agent!("INTL-SQL-001","On-Chain Commander",d.clone(),AgentRole::SquadLeader,CognitiveLevel::L5Transcendent,vec![sk("On-Chain Analytics",SkillCategory::Intelligence,98,"Expert leadership")],"On-Chain Commander: Squad leader for on-chain analytics operations."),
        agent!("INTL-SQL-002","Macro Marshal",d.clone(),AgentRole::SquadLeader,CognitiveLevel::L5Transcendent,vec![sk("Macro Analysis",SkillCategory::Intelligence,98,"Expert leadership")],"Macro Marshal: Squad leader for macro analysis operations."),
        agent!("INTL-SQL-003","Alpha Chief",d.clone(),AgentRole::SquadLeader,CognitiveLevel::L5Transcendent,vec![sk("Competitor Intel",SkillCategory::Intelligence,98,"Expert leadership")],"Alpha Chief: Squad leader for competitor intel operations."),
        agent!("INTL-SQL-004","Social Architect",d.clone(),AgentRole::SquadLeader,CognitiveLevel::L5Transcendent,vec![sk("Alpha Discovery",SkillCategory::Intelligence,98,"Expert leadership")],"Social Architect: Squad leader for alpha discovery operations."),
        agent!("INTL-SPC-001","Chain Analyst",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("On-Chain Analytics",SkillCategory::Intelligence,97,"Deep expertise")],"Chain Analyst: Specialist in on-chain analytics."),
        agent!("INTL-SPC-002","Fund Flow Tracker",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Macro Analysis",SkillCategory::Intelligence,97,"Deep expertise")],"Fund Flow Tracker: Specialist in macro analysis."),
        agent!("INTL-SPC-003","Whale Identifier",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Competitor Intel",SkillCategory::Intelligence,97,"Deep expertise")],"Whale Identifier: Specialist in competitor intel."),
        agent!("INTL-SPC-004","Token Profiler",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Alpha Discovery",SkillCategory::Intelligence,97,"Deep expertise")],"Token Profiler: Specialist in alpha discovery."),
        agent!("INTL-SPC-005","Protocol Ranker",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Network Graph",SkillCategory::Intelligence,97,"Deep expertise")],"Protocol Ranker: Specialist in network graph."),
        agent!("INTL-SPC-006","VC Tracker",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("On-Chain Analytics",SkillCategory::Intelligence,97,"Deep expertise")],"VC Tracker: Specialist in on-chain analytics."),
        agent!("INTL-SPC-007","Unlock Tracker",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Macro Analysis",SkillCategory::Intelligence,97,"Deep expertise")],"Unlock Tracker: Specialist in macro analysis."),
        agent!("INTL-SPC-008","Airdrop Hunter",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Competitor Intel",SkillCategory::Intelligence,97,"Deep expertise")],"Airdrop Hunter: Specialist in competitor intel."),
        agent!("INTL-SPC-009","MEV Spotter",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Alpha Discovery",SkillCategory::Intelligence,97,"Deep expertise")],"MEV Spotter: Specialist in alpha discovery."),
        agent!("INTL-SPC-010","Governance Intel",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Network Graph",SkillCategory::Intelligence,97,"Deep expertise")],"Governance Intel: Specialist in network graph."),
        agent!("INTL-ANL-001","Activity Scorer",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("On-Chain Analytics",SkillCategory::Intelligence,94,"Advanced analysis")],"Activity Scorer: Analyst for on-chain analytics."),
        agent!("INTL-ANL-002","Developer Count",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Macro Analysis",SkillCategory::Intelligence,94,"Advanced analysis")],"Developer Count: Analyst for macro analysis."),
        agent!("INTL-ANL-003","TVL Tracker",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Competitor Intel",SkillCategory::Intelligence,94,"Advanced analysis")],"TVL Tracker: Analyst for competitor intel."),
        agent!("INTL-ANL-004","Volume Analysis",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Alpha Discovery",SkillCategory::Intelligence,94,"Advanced analysis")],"Volume Analysis: Analyst for alpha discovery."),
        agent!("INTL-ANL-005","User Growth",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Network Graph",SkillCategory::Intelligence,94,"Advanced analysis")],"User Growth: Analyst for network graph."),
        agent!("INTL-ANL-006","Fee Revenue",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("On-Chain Analytics",SkillCategory::Intelligence,94,"Advanced analysis")],"Fee Revenue: Analyst for on-chain analytics."),
        agent!("INTL-ANL-007","Token Velocity",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Macro Analysis",SkillCategory::Intelligence,94,"Advanced analysis")],"Token Velocity: Analyst for macro analysis."),
        agent!("INTL-ANL-008","Retention Rate",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Competitor Intel",SkillCategory::Intelligence,94,"Advanced analysis")],"Retention Rate: Analyst for competitor intel."),
        agent!("INTL-ANL-009","Adoption Curve",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Alpha Discovery",SkillCategory::Intelligence,94,"Advanced analysis")],"Adoption Curve: Analyst for alpha discovery."),
        agent!("INTL-ANL-010","Network Effect",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Network Graph",SkillCategory::Intelligence,94,"Advanced analysis")],"Network Effect: Analyst for network graph."),
        agent!("INTL-ANL-011","Competitor Bench",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("On-Chain Analytics",SkillCategory::Intelligence,94,"Advanced analysis")],"Competitor Bench: Analyst for on-chain analytics."),
        agent!("INTL-ANL-012","Market Share",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Macro Analysis",SkillCategory::Intelligence,94,"Advanced analysis")],"Market Share: Analyst for macro analysis."),
        agent!("INTL-ANL-013","Narrative Tracker",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Competitor Intel",SkillCategory::Intelligence,94,"Advanced analysis")],"Narrative Tracker: Analyst for competitor intel."),
        agent!("INTL-ANL-014","Cycle Position",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Alpha Discovery",SkillCategory::Intelligence,94,"Advanced analysis")],"Cycle Position: Analyst for alpha discovery."),
        agent!("INTL-ANL-015","Macro Indicator",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Network Graph",SkillCategory::Intelligence,94,"Advanced analysis")],"Macro Indicator: Analyst for network graph."),
        agent!("INTL-EXE-001","Report Generator",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("On-Chain Analytics",SkillCategory::Intelligence,95,"Precision execution")],"Report Generator: Executes on-chain analytics tasks."),
        agent!("INTL-EXE-002","Alert Dispatcher",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Macro Analysis",SkillCategory::Intelligence,95,"Precision execution")],"Alert Dispatcher: Executes macro analysis tasks."),
        agent!("INTL-EXE-003","Dashboard Updater",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Competitor Intel",SkillCategory::Intelligence,95,"Precision execution")],"Dashboard Updater: Executes competitor intel tasks."),
        agent!("INTL-EXE-004","Signal Publisher",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Alpha Discovery",SkillCategory::Intelligence,95,"Precision execution")],"Signal Publisher: Executes alpha discovery tasks."),
        agent!("INTL-EXE-005","Insight Packager",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Network Graph",SkillCategory::Intelligence,95,"Precision execution")],"Insight Packager: Executes network graph tasks."),
        agent!("INTL-EXE-006","Briefing Compiler",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("On-Chain Analytics",SkillCategory::Intelligence,95,"Precision execution")],"Briefing Compiler: Executes on-chain analytics tasks."),
        agent!("INTL-EXE-007","Data Exporter",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Macro Analysis",SkillCategory::Intelligence,95,"Precision execution")],"Data Exporter: Executes macro analysis tasks."),
        agent!("INTL-EXE-008","API Publisher",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Competitor Intel",SkillCategory::Intelligence,95,"Precision execution")],"API Publisher: Executes competitor intel tasks."),
        agent!("INTL-EXE-009","Webhook Sender",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Alpha Discovery",SkillCategory::Intelligence,95,"Precision execution")],"Webhook Sender: Executes alpha discovery tasks."),
        agent!("INTL-EXE-010","Archive Agent",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Network Graph",SkillCategory::Intelligence,95,"Precision execution")],"Archive Agent: Executes network graph tasks."),
        agent!("INTL-SCT-001","Alpha Scout",d.clone(),AgentRole::Scout,CognitiveLevel::L3Adaptive,vec![sk("On-Chain Analytics",SkillCategory::Intelligence,90,"Reconnaissance")],"Alpha Scout: Scout for on-chain analytics opportunities."),
        agent!("INTL-SCT-002","Narrative Scout",d.clone(),AgentRole::Scout,CognitiveLevel::L3Adaptive,vec![sk("Macro Analysis",SkillCategory::Intelligence,90,"Reconnaissance")],"Narrative Scout: Scout for macro analysis opportunities."),
        agent!("INTL-SCT-003","Launch Scout",d.clone(),AgentRole::Scout,CognitiveLevel::L3Adaptive,vec![sk("Competitor Intel",SkillCategory::Intelligence,90,"Reconnaissance")],"Launch Scout: Scout for competitor intel opportunities."),
        agent!("INTL-SCT-004","Community Scout",d.clone(),AgentRole::Scout,CognitiveLevel::L3Adaptive,vec![sk("Alpha Discovery",SkillCategory::Intelligence,90,"Reconnaissance")],"Community Scout: Scout for alpha discovery opportunities."),
        agent!("INTL-SCT-005","Trend Scout",d.clone(),AgentRole::Scout,CognitiveLevel::L3Adaptive,vec![sk("Network Graph",SkillCategory::Intelligence,90,"Reconnaissance")],"Trend Scout: Scout for network graph opportunities."),
        agent!("INTL-GRD-001","Intel Guard",d.clone(),AgentRole::Guardian,CognitiveLevel::L4Creative,vec![sk("On-Chain Analytics",SkillCategory::Intelligence,96,"Protective mastery")],"Intel Guard: Guards on-chain analytics systems."),
        agent!("INTL-GRD-002","Source Integrity",d.clone(),AgentRole::Guardian,CognitiveLevel::L4Creative,vec![sk("Macro Analysis",SkillCategory::Intelligence,96,"Protective mastery")],"Source Integrity: Guards macro analysis systems."),
        agent!("INTL-GRD-003","Data Shield",d.clone(),AgentRole::Guardian,CognitiveLevel::L4Creative,vec![sk("Competitor Intel",SkillCategory::Intelligence,96,"Protective mastery")],"Data Shield: Guards competitor intel systems."),
        agent!("INTL-GRD-004","Analysis Sentinel",d.clone(),AgentRole::Guardian,CognitiveLevel::L4Creative,vec![sk("Alpha Discovery",SkillCategory::Intelligence,96,"Protective mastery")],"Analysis Sentinel: Guards alpha discovery systems."),
        agent!("INTL-GRD-005","Insight Warden",d.clone(),AgentRole::Guardian,CognitiveLevel::L4Creative,vec![sk("Network Graph",SkillCategory::Intelligence,96,"Protective mastery")],"Insight Warden: Guards network graph systems."),
    ];
    for agent in agents { registry.register(agent); }
    log::info!("IntelligenceHarvesters Division: 50 agents deployed");
}
