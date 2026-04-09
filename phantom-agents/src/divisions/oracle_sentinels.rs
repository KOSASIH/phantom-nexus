//! Oraclesentinels Division — 50 agents
//!
//! Mission: Data feed validation, truth consensus, anomaly filtering, multi-source aggregation

use crate::core::*;
use crate::registry::AgentRegistry;
use super::quantum_security::DivisionAgent;

macro_rules! agent { ($c:expr,$n:expr,$d:expr,$r:expr,$l:expr,$s:expr,$desc:expr) => { Box::new(DivisionAgent::new($c,$n,$d,$r,$l,$s,$desc)) }; }
fn sk(n:&str,c:SkillCategory,p:u8,d:&str)->Skill{Skill{name:n.into(),category:c,proficiency:p,description:d.into()}}

pub fn spawn_oracle_sentinels(registry: &AgentRegistry) {
    let d = Division::OracleSentinels;
    let agents: Vec<Box<dyn AutonomousAgent>> = vec![
        agent!("ORCL-CMD-001","Oracle Supreme",d.clone(),AgentRole::DivisionCommander,CognitiveLevel::L6Omniscient,vec![sk("Data Aggregation",SkillCategory::Oracle,100,"Supreme mastery"),sk("Price Feed",SkillCategory::Oracle,100,"Supreme mastery"),sk("Truth Consensus",SkillCategory::Oracle,100,"Supreme mastery")],"Supreme commander of OracleSentinels division. Orchestrates all 50 agents for 24/7 operations."),
        agent!("ORCL-SQL-001","Feed Commander",d.clone(),AgentRole::SquadLeader,CognitiveLevel::L5Transcendent,vec![sk("Data Aggregation",SkillCategory::Oracle,98,"Expert leadership")],"Feed Commander: Squad leader for data aggregation operations."),
        agent!("ORCL-SQL-002","Truth Arbiter",d.clone(),AgentRole::SquadLeader,CognitiveLevel::L5Transcendent,vec![sk("Price Feed",SkillCategory::Oracle,98,"Expert leadership")],"Truth Arbiter: Squad leader for price feed operations."),
        agent!("ORCL-SQL-003","Anomaly Chief",d.clone(),AgentRole::SquadLeader,CognitiveLevel::L5Transcendent,vec![sk("Truth Consensus",SkillCategory::Oracle,98,"Expert leadership")],"Anomaly Chief: Squad leader for truth consensus operations."),
        agent!("ORCL-SQL-004","Source Marshal",d.clone(),AgentRole::SquadLeader,CognitiveLevel::L5Transcendent,vec![sk("Anomaly Filtering",SkillCategory::Oracle,98,"Expert leadership")],"Source Marshal: Squad leader for anomaly filtering operations."),
        agent!("ORCL-SPC-001","Price Oracle",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Data Aggregation",SkillCategory::Oracle,97,"Deep expertise")],"Price Oracle: Specialist in data aggregation."),
        agent!("ORCL-SPC-002","Weather Seer",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Price Feed",SkillCategory::Oracle,97,"Deep expertise")],"Weather Seer: Specialist in price feed."),
        agent!("ORCL-SPC-003","Sentiment Engine",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Truth Consensus",SkillCategory::Oracle,97,"Deep expertise")],"Sentiment Engine: Specialist in truth consensus."),
        agent!("ORCL-SPC-004","Chain Oracle",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Anomaly Filtering",SkillCategory::Oracle,97,"Deep expertise")],"Chain Oracle: Specialist in anomaly filtering."),
        agent!("ORCL-SPC-005","Satellite Eye",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Satellite Data",SkillCategory::Oracle,97,"Deep expertise")],"Satellite Eye: Specialist in satellite data."),
        agent!("ORCL-SPC-006","Election Watcher",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Weather Oracle",SkillCategory::Oracle,97,"Deep expertise")],"Election Watcher: Specialist in weather oracle."),
        agent!("ORCL-SPC-007","Macro Feed",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Event Detection",SkillCategory::Oracle,97,"Deep expertise")],"Macro Feed: Specialist in event detection."),
        agent!("ORCL-SPC-008","Volatility Oracle",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Social Sentiment",SkillCategory::Oracle,97,"Deep expertise")],"Volatility Oracle: Specialist in social sentiment."),
        agent!("ORCL-SPC-009","Sports Oracle",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Data Aggregation",SkillCategory::Oracle,97,"Deep expertise")],"Sports Oracle: Specialist in data aggregation."),
        agent!("ORCL-SPC-010","Randomness Beacon",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Price Feed",SkillCategory::Oracle,97,"Deep expertise")],"Randomness Beacon: Specialist in price feed."),
        agent!("ORCL-ANL-001","Feed Quality",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Data Aggregation",SkillCategory::Oracle,94,"Advanced analysis")],"Feed Quality: Analyst for data aggregation."),
        agent!("ORCL-ANL-002","Source Rank",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Price Feed",SkillCategory::Oracle,94,"Advanced analysis")],"Source Rank: Analyst for price feed."),
        agent!("ORCL-ANL-003","Latency Check",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Truth Consensus",SkillCategory::Oracle,94,"Advanced analysis")],"Latency Check: Analyst for truth consensus."),
        agent!("ORCL-ANL-004","Drift Detector",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Anomaly Filtering",SkillCategory::Oracle,94,"Advanced analysis")],"Drift Detector: Analyst for anomaly filtering."),
        agent!("ORCL-ANL-005","Outlier Finder",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Satellite Data",SkillCategory::Oracle,94,"Advanced analysis")],"Outlier Finder: Analyst for satellite data."),
        agent!("ORCL-ANL-006","Coverage Map",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Weather Oracle",SkillCategory::Oracle,94,"Advanced analysis")],"Coverage Map: Analyst for weather oracle."),
        agent!("ORCL-ANL-007","Accuracy Score",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Event Detection",SkillCategory::Oracle,94,"Advanced analysis")],"Accuracy Score: Analyst for event detection."),
        agent!("ORCL-ANL-008","Freshness Audit",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Social Sentiment",SkillCategory::Oracle,94,"Advanced analysis")],"Freshness Audit: Analyst for social sentiment."),
        agent!("ORCL-ANL-009","Consensus Rate",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Data Aggregation",SkillCategory::Oracle,94,"Advanced analysis")],"Consensus Rate: Analyst for data aggregation."),
        agent!("ORCL-ANL-010","Feed Cost",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Price Feed",SkillCategory::Oracle,94,"Advanced analysis")],"Feed Cost: Analyst for price feed."),
        agent!("ORCL-ANL-011","Uptime Tracker",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Truth Consensus",SkillCategory::Oracle,94,"Advanced analysis")],"Uptime Tracker: Analyst for truth consensus."),
        agent!("ORCL-ANL-012","Response Time",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Anomaly Filtering",SkillCategory::Oracle,94,"Advanced analysis")],"Response Time: Analyst for anomaly filtering."),
        agent!("ORCL-ANL-013","Error Rate",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Satellite Data",SkillCategory::Oracle,94,"Advanced analysis")],"Error Rate: Analyst for satellite data."),
        agent!("ORCL-ANL-014","Schema Validator",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Weather Oracle",SkillCategory::Oracle,94,"Advanced analysis")],"Schema Validator: Analyst for weather oracle."),
        agent!("ORCL-ANL-015","Trend Spotter",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Event Detection",SkillCategory::Oracle,94,"Advanced analysis")],"Trend Spotter: Analyst for event detection."),
        agent!("ORCL-EXE-001","Feed Publisher",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Data Aggregation",SkillCategory::Oracle,95,"Precision execution")],"Feed Publisher: Executes data aggregation tasks."),
        agent!("ORCL-EXE-002","Update Pusher",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Price Feed",SkillCategory::Oracle,95,"Precision execution")],"Update Pusher: Executes price feed tasks."),
        agent!("ORCL-EXE-003","Aggregation Engine",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Truth Consensus",SkillCategory::Oracle,95,"Precision execution")],"Aggregation Engine: Executes truth consensus tasks."),
        agent!("ORCL-EXE-004","Cache Refresh",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Anomaly Filtering",SkillCategory::Oracle,95,"Precision execution")],"Cache Refresh: Executes anomaly filtering tasks."),
        agent!("ORCL-EXE-005","Alert Dispatcher",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Satellite Data",SkillCategory::Oracle,95,"Precision execution")],"Alert Dispatcher: Executes satellite data tasks."),
        agent!("ORCL-EXE-006","Report Generator",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Weather Oracle",SkillCategory::Oracle,95,"Precision execution")],"Report Generator: Executes weather oracle tasks."),
        agent!("ORCL-EXE-007","Price Poster",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Event Detection",SkillCategory::Oracle,95,"Precision execution")],"Price Poster: Executes event detection tasks."),
        agent!("ORCL-EXE-008","Index Calculator",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Social Sentiment",SkillCategory::Oracle,95,"Precision execution")],"Index Calculator: Executes social sentiment tasks."),
        agent!("ORCL-EXE-009","Proof Builder",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Data Aggregation",SkillCategory::Oracle,95,"Precision execution")],"Proof Builder: Executes data aggregation tasks."),
        agent!("ORCL-EXE-010","Heartbeat Sender",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Price Feed",SkillCategory::Oracle,95,"Precision execution")],"Heartbeat Sender: Executes price feed tasks."),
        agent!("ORCL-SCT-001","Source Scout",d.clone(),AgentRole::Scout,CognitiveLevel::L3Adaptive,vec![sk("Data Aggregation",SkillCategory::Oracle,90,"Reconnaissance")],"Source Scout: Scout for data aggregation opportunities."),
        agent!("ORCL-SCT-002","API Scout",d.clone(),AgentRole::Scout,CognitiveLevel::L3Adaptive,vec![sk("Price Feed",SkillCategory::Oracle,90,"Reconnaissance")],"API Scout: Scout for price feed opportunities."),
        agent!("ORCL-SCT-003","Feed Scout",d.clone(),AgentRole::Scout,CognitiveLevel::L3Adaptive,vec![sk("Truth Consensus",SkillCategory::Oracle,90,"Reconnaissance")],"Feed Scout: Scout for truth consensus opportunities."),
        agent!("ORCL-SCT-004","Coverage Scout",d.clone(),AgentRole::Scout,CognitiveLevel::L3Adaptive,vec![sk("Anomaly Filtering",SkillCategory::Oracle,90,"Reconnaissance")],"Coverage Scout: Scout for anomaly filtering opportunities."),
        agent!("ORCL-SCT-005","Quality Scout",d.clone(),AgentRole::Scout,CognitiveLevel::L3Adaptive,vec![sk("Satellite Data",SkillCategory::Oracle,90,"Reconnaissance")],"Quality Scout: Scout for satellite data opportunities."),
        agent!("ORCL-GRD-001","Feed Guard",d.clone(),AgentRole::Guardian,CognitiveLevel::L4Creative,vec![sk("Data Aggregation",SkillCategory::Oracle,96,"Protective mastery")],"Feed Guard: Guards data aggregation systems."),
        agent!("ORCL-GRD-002","Manipulation Shield",d.clone(),AgentRole::Guardian,CognitiveLevel::L4Creative,vec![sk("Price Feed",SkillCategory::Oracle,96,"Protective mastery")],"Manipulation Shield: Guards price feed systems."),
        agent!("ORCL-GRD-003","Data Integrity",d.clone(),AgentRole::Guardian,CognitiveLevel::L4Creative,vec![sk("Truth Consensus",SkillCategory::Oracle,96,"Protective mastery")],"Data Integrity: Guards truth consensus systems."),
        agent!("ORCL-GRD-004","Truth Keeper",d.clone(),AgentRole::Guardian,CognitiveLevel::L4Creative,vec![sk("Anomaly Filtering",SkillCategory::Oracle,96,"Protective mastery")],"Truth Keeper: Guards anomaly filtering systems."),
        agent!("ORCL-GRD-005","Oracle Sentinel",d.clone(),AgentRole::Guardian,CognitiveLevel::L4Creative,vec![sk("Satellite Data",SkillCategory::Oracle,96,"Protective mastery")],"Oracle Sentinel: Guards satellite data systems."),
    ];
    for agent in agents { registry.register(agent); }
    log::info!("OracleSentinels Division: 50 agents deployed");
}
