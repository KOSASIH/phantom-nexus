//! Iotcommanders Division — 50 agents
//!
//! Mission: Device mesh networking, micro-payment routing, edge computing, telemetry

use crate::core::*;
use crate::registry::AgentRegistry;
use super::quantum_security::DivisionAgent;

macro_rules! agent { ($c:expr,$n:expr,$d:expr,$r:expr,$l:expr,$s:expr,$desc:expr) => { Box::new(DivisionAgent::new($c,$n,$d,$r,$l,$s,$desc)) }; }
fn sk(n:&str,c:SkillCategory,p:u8,d:&str)->Skill{Skill{name:n.into(),category:c,proficiency:p,description:d.into()}}

pub fn spawn_iot_commanders(registry: &AgentRegistry) {
    let d = Division::IoTCommanders;
    let agents: Vec<Box<dyn AutonomousAgent>> = vec![
        agent!("IOTC-CMD-001","IoT Supreme",d.clone(),AgentRole::DivisionCommander,CognitiveLevel::L6Omniscient,vec![sk("Device Mesh",SkillCategory::IoT,100,"Supreme mastery"),sk("Edge Computing",SkillCategory::IoT,100,"Supreme mastery"),sk("Micro-Payments",SkillCategory::IoT,100,"Supreme mastery")],"Supreme commander of IoTCommanders division. Orchestrates all 50 agents for 24/7 operations."),
        agent!("IOTC-SQL-001","Device Commander",d.clone(),AgentRole::SquadLeader,CognitiveLevel::L5Transcendent,vec![sk("Device Mesh",SkillCategory::IoT,98,"Expert leadership")],"Device Commander: Squad leader for device mesh operations."),
        agent!("IOTC-SQL-002","Edge Marshal",d.clone(),AgentRole::SquadLeader,CognitiveLevel::L5Transcendent,vec![sk("Edge Computing",SkillCategory::IoT,98,"Expert leadership")],"Edge Marshal: Squad leader for edge computing operations."),
        agent!("IOTC-SQL-003","Payment Chief",d.clone(),AgentRole::SquadLeader,CognitiveLevel::L5Transcendent,vec![sk("Micro-Payments",SkillCategory::IoT,98,"Expert leadership")],"Payment Chief: Squad leader for micro-payments operations."),
        agent!("IOTC-SQL-004","Telemetry Architect",d.clone(),AgentRole::SquadLeader,CognitiveLevel::L5Transcendent,vec![sk("Telemetry Processing",SkillCategory::IoT,98,"Expert leadership")],"Telemetry Architect: Squad leader for telemetry processing operations."),
        agent!("IOTC-SPC-001","Mesh Coordinator",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Device Mesh",SkillCategory::IoT,97,"Deep expertise")],"Mesh Coordinator: Specialist in device mesh."),
        agent!("IOTC-SPC-002","Edge Scheduler",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Edge Computing",SkillCategory::IoT,97,"Deep expertise")],"Edge Scheduler: Specialist in edge computing."),
        agent!("IOTC-SPC-003","Channel Manager",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Micro-Payments",SkillCategory::IoT,97,"Deep expertise")],"Channel Manager: Specialist in micro-payments."),
        agent!("IOTC-SPC-004","Sensor Fusion",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Telemetry Processing",SkillCategory::IoT,97,"Deep expertise")],"Sensor Fusion: Specialist in telemetry processing."),
        agent!("IOTC-SPC-005","OTA Deployer",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Firmware Updates",SkillCategory::IoT,97,"Deep expertise")],"OTA Deployer: Specialist in firmware updates."),
        agent!("IOTC-SPC-006","Power Manager",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Device Mesh",SkillCategory::IoT,97,"Deep expertise")],"Power Manager: Specialist in device mesh."),
        agent!("IOTC-SPC-007","Protocol Adapter",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Edge Computing",SkillCategory::IoT,97,"Deep expertise")],"Protocol Adapter: Specialist in edge computing."),
        agent!("IOTC-SPC-008","Security Scanner",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Micro-Payments",SkillCategory::IoT,97,"Deep expertise")],"Security Scanner: Specialist in micro-payments."),
        agent!("IOTC-SPC-009","Data Compressor",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Telemetry Processing",SkillCategory::IoT,97,"Deep expertise")],"Data Compressor: Specialist in telemetry processing."),
        agent!("IOTC-SPC-010","Routing Optimizer",d.clone(),AgentRole::Specialist,CognitiveLevel::L5Transcendent,vec![sk("Firmware Updates",SkillCategory::IoT,97,"Deep expertise")],"Routing Optimizer: Specialist in firmware updates."),
        agent!("IOTC-ANL-001","Device Health",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Device Mesh",SkillCategory::IoT,94,"Advanced analysis")],"Device Health: Analyst for device mesh."),
        agent!("IOTC-ANL-002","Network Topology",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Edge Computing",SkillCategory::IoT,94,"Advanced analysis")],"Network Topology: Analyst for edge computing."),
        agent!("IOTC-ANL-003","Payment Flow",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Micro-Payments",SkillCategory::IoT,94,"Advanced analysis")],"Payment Flow: Analyst for micro-payments."),
        agent!("IOTC-ANL-004","Data Quality",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Telemetry Processing",SkillCategory::IoT,94,"Advanced analysis")],"Data Quality: Analyst for telemetry processing."),
        agent!("IOTC-ANL-005","Latency Map",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Firmware Updates",SkillCategory::IoT,94,"Advanced analysis")],"Latency Map: Analyst for firmware updates."),
        agent!("IOTC-ANL-006","Battery Status",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Device Mesh",SkillCategory::IoT,94,"Advanced analysis")],"Battery Status: Analyst for device mesh."),
        agent!("IOTC-ANL-007","Coverage Map",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Edge Computing",SkillCategory::IoT,94,"Advanced analysis")],"Coverage Map: Analyst for edge computing."),
        agent!("IOTC-ANL-008","Error Rate",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Micro-Payments",SkillCategory::IoT,94,"Advanced analysis")],"Error Rate: Analyst for micro-payments."),
        agent!("IOTC-ANL-009","Throughput Check",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Telemetry Processing",SkillCategory::IoT,94,"Advanced analysis")],"Throughput Check: Analyst for telemetry processing."),
        agent!("IOTC-ANL-010","Capacity Plan",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Firmware Updates",SkillCategory::IoT,94,"Advanced analysis")],"Capacity Plan: Analyst for firmware updates."),
        agent!("IOTC-ANL-011","Cost Analysis",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Device Mesh",SkillCategory::IoT,94,"Advanced analysis")],"Cost Analysis: Analyst for device mesh."),
        agent!("IOTC-ANL-012","Security Audit",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Edge Computing",SkillCategory::IoT,94,"Advanced analysis")],"Security Audit: Analyst for edge computing."),
        agent!("IOTC-ANL-013","Firmware Status",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Micro-Payments",SkillCategory::IoT,94,"Advanced analysis")],"Firmware Status: Analyst for micro-payments."),
        agent!("IOTC-ANL-014","Signal Strength",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Telemetry Processing",SkillCategory::IoT,94,"Advanced analysis")],"Signal Strength: Analyst for telemetry processing."),
        agent!("IOTC-ANL-015","Uptime Score",d.clone(),AgentRole::Analyst,CognitiveLevel::L4Creative,vec![sk("Firmware Updates",SkillCategory::IoT,94,"Advanced analysis")],"Uptime Score: Analyst for firmware updates."),
        agent!("IOTC-EXE-001","Device Provisioner",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Device Mesh",SkillCategory::IoT,95,"Precision execution")],"Device Provisioner: Executes device mesh tasks."),
        agent!("IOTC-EXE-002","Firmware Pusher",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Edge Computing",SkillCategory::IoT,95,"Precision execution")],"Firmware Pusher: Executes edge computing tasks."),
        agent!("IOTC-EXE-003","Payment Router",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Micro-Payments",SkillCategory::IoT,95,"Precision execution")],"Payment Router: Executes micro-payments tasks."),
        agent!("IOTC-EXE-004","Config Deployer",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Telemetry Processing",SkillCategory::IoT,95,"Precision execution")],"Config Deployer: Executes telemetry processing tasks."),
        agent!("IOTC-EXE-005","Alert Sender",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Firmware Updates",SkillCategory::IoT,95,"Precision execution")],"Alert Sender: Executes firmware updates tasks."),
        agent!("IOTC-EXE-006","Restart Agent",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Device Mesh",SkillCategory::IoT,95,"Precision execution")],"Restart Agent: Executes device mesh tasks."),
        agent!("IOTC-EXE-007","Decommission Agent",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Edge Computing",SkillCategory::IoT,95,"Precision execution")],"Decommission Agent: Executes edge computing tasks."),
        agent!("IOTC-EXE-008","Channel Opener",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Micro-Payments",SkillCategory::IoT,95,"Precision execution")],"Channel Opener: Executes micro-payments tasks."),
        agent!("IOTC-EXE-009","Data Forwarder",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Telemetry Processing",SkillCategory::IoT,95,"Precision execution")],"Data Forwarder: Executes telemetry processing tasks."),
        agent!("IOTC-EXE-010","Backup Agent",d.clone(),AgentRole::Executor,CognitiveLevel::L4Creative,vec![sk("Firmware Updates",SkillCategory::IoT,95,"Precision execution")],"Backup Agent: Executes firmware updates tasks."),
        agent!("IOTC-SCT-001","Device Scout",d.clone(),AgentRole::Scout,CognitiveLevel::L3Adaptive,vec![sk("Device Mesh",SkillCategory::IoT,90,"Reconnaissance")],"Device Scout: Scout for device mesh opportunities."),
        agent!("IOTC-SCT-002","Protocol Scout",d.clone(),AgentRole::Scout,CognitiveLevel::L3Adaptive,vec![sk("Edge Computing",SkillCategory::IoT,90,"Reconnaissance")],"Protocol Scout: Scout for edge computing opportunities."),
        agent!("IOTC-SCT-003","Market Scout",d.clone(),AgentRole::Scout,CognitiveLevel::L3Adaptive,vec![sk("Micro-Payments",SkillCategory::IoT,90,"Reconnaissance")],"Market Scout: Scout for micro-payments opportunities."),
        agent!("IOTC-SCT-004","Integration Scout",d.clone(),AgentRole::Scout,CognitiveLevel::L3Adaptive,vec![sk("Telemetry Processing",SkillCategory::IoT,90,"Reconnaissance")],"Integration Scout: Scout for telemetry processing opportunities."),
        agent!("IOTC-SCT-005","Security Scout",d.clone(),AgentRole::Scout,CognitiveLevel::L3Adaptive,vec![sk("Firmware Updates",SkillCategory::IoT,90,"Reconnaissance")],"Security Scout: Scout for firmware updates opportunities."),
        agent!("IOTC-GRD-001","Device Guard",d.clone(),AgentRole::Guardian,CognitiveLevel::L4Creative,vec![sk("Device Mesh",SkillCategory::IoT,96,"Protective mastery")],"Device Guard: Guards device mesh systems."),
        agent!("IOTC-GRD-002","Network Shield",d.clone(),AgentRole::Guardian,CognitiveLevel::L4Creative,vec![sk("Edge Computing",SkillCategory::IoT,96,"Protective mastery")],"Network Shield: Guards edge computing systems."),
        agent!("IOTC-GRD-003","Payment Sentinel",d.clone(),AgentRole::Guardian,CognitiveLevel::L4Creative,vec![sk("Micro-Payments",SkillCategory::IoT,96,"Protective mastery")],"Payment Sentinel: Guards micro-payments systems."),
        agent!("IOTC-GRD-004","Data Warden",d.clone(),AgentRole::Guardian,CognitiveLevel::L4Creative,vec![sk("Telemetry Processing",SkillCategory::IoT,96,"Protective mastery")],"Data Warden: Guards telemetry processing systems."),
        agent!("IOTC-GRD-005","Firmware Guardian",d.clone(),AgentRole::Guardian,CognitiveLevel::L4Creative,vec![sk("Firmware Updates",SkillCategory::IoT,96,"Protective mastery")],"Firmware Guardian: Guards firmware updates systems."),
    ];
    for agent in agents { registry.register(agent); }
    log::info!("IoTCommanders Division: 50 agents deployed");
}
