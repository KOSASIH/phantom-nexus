//! Core Agent Traits and Types
//!
//! Foundational abstractions for all 1000+ autonomous AGI agents.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Unique agent identifier
pub type AgentId = Uuid;

/// Agent cognitive level — determines autonomy and capability tier
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CognitiveLevel {
    /// Reactive: stimulus-response only
    L1Reactive,
    /// Deliberative: plan-execute with memory
    L2Deliberative,
    /// Adaptive: learns from experience, self-modifies behavior
    L3Adaptive,
    /// Creative: generates novel strategies and solutions
    L4Creative,
    /// Transcendent: AGI-level reasoning, cross-domain mastery
    L5Transcendent,
    /// Omniscient: God-level awareness, predictive omniscience
    L6Omniscient,
}

/// Agent operational status
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum AgentStatus {
    Initializing,
    Active,
    Processing { task: String },
    Learning,
    Evolving,
    Coordinating { with: Vec<AgentId> },
    Resting { resume_at: u64 },
    Suspended { reason: String },
    Terminated,
}

/// Agent health metrics
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AgentHealth {
    pub cpu_usage: f64,
    pub memory_usage_mb: f64,
    pub energy_level: f64,        // 0.0 – 1.0
    pub cognitive_load: f64,      // 0.0 – 1.0
    pub uptime_seconds: u64,
    pub tasks_completed: u64,
    pub tasks_failed: u64,
    pub success_rate: f64,
    pub last_heartbeat: u64,
    pub evolution_generation: u32,
}

impl Default for AgentHealth {
    fn default() -> Self {
        Self {
            cpu_usage: 0.0,
            memory_usage_mb: 0.0,
            energy_level: 1.0,
            cognitive_load: 0.0,
            uptime_seconds: 0,
            tasks_completed: 0,
            tasks_failed: 0,
            success_rate: 1.0,
            last_heartbeat: 0,
            evolution_generation: 1,
        }
    }
}

/// Agent capability / skill
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Skill {
    pub name: String,
    pub category: SkillCategory,
    pub proficiency: u8,   // 0–100
    pub description: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SkillCategory {
    Security,
    Trading,
    DeFi,
    Consensus,
    CrossChain,
    Oracle,
    Governance,
    Networking,
    Storage,
    SmartContract,
    Tokenomics,
    Risk,
    Intelligence,
    Metaverse,
    IoT,
    Privacy,
    Evolution,
    Compliance,
    Community,
    Emergency,
    Mathematics,
    NaturalLanguage,
    Planning,
    Reasoning,
    Creativity,
}

/// Division assignment
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Division {
    QuantumSecurity,
    NeuralTrading,
    DeFiOperations,
    ConsensusGuardians,
    CrossChainDiplomats,
    OracleSentinels,
    GovernanceArchitects,
    NetworkWeavers,
    StorageAlchemists,
    SmartContractForge,
    TokenomicsEngineers,
    RiskSentinels,
    IntelligenceHarvesters,
    MetaverseBuilders,
    IoTCommanders,
    PrivacyPhantoms,
    EvolutionEngineers,
    ComplianceShadows,
    CommunityCatalysts,
    EmergencyResponse,
}

impl Division {
    pub fn all() -> Vec<Division> {
        vec![
            Division::QuantumSecurity,
            Division::NeuralTrading,
            Division::DeFiOperations,
            Division::ConsensusGuardians,
            Division::CrossChainDiplomats,
            Division::OracleSentinels,
            Division::GovernanceArchitects,
            Division::NetworkWeavers,
            Division::StorageAlchemists,
            Division::SmartContractForge,
            Division::TokenomicsEngineers,
            Division::RiskSentinels,
            Division::IntelligenceHarvesters,
            Division::MetaverseBuilders,
            Division::IoTCommanders,
            Division::PrivacyPhantoms,
            Division::EvolutionEngineers,
            Division::ComplianceShadows,
            Division::CommunityCatalysts,
            Division::EmergencyResponse,
        ]
    }

    pub fn display_name(&self) -> &str {
        match self {
            Division::QuantumSecurity => "Quantum Security Division",
            Division::NeuralTrading => "Neural Trading Division",
            Division::DeFiOperations => "DeFi Operations Division",
            Division::ConsensusGuardians => "Consensus Guardians Division",
            Division::CrossChainDiplomats => "Cross-Chain Diplomats Division",
            Division::OracleSentinels => "Oracle Sentinels Division",
            Division::GovernanceArchitects => "Governance Architects Division",
            Division::NetworkWeavers => "Network Weavers Division",
            Division::StorageAlchemists => "Storage Alchemists Division",
            Division::SmartContractForge => "Smart Contract Forge Division",
            Division::TokenomicsEngineers => "Tokenomics Engineers Division",
            Division::RiskSentinels => "Risk Sentinels Division",
            Division::IntelligenceHarvesters => "Intelligence Harvesters Division",
            Division::MetaverseBuilders => "Metaverse Builders Division",
            Division::IoTCommanders => "IoT Commanders Division",
            Division::PrivacyPhantoms => "Privacy Phantoms Division",
            Division::EvolutionEngineers => "Evolution Engineers Division",
            Division::ComplianceShadows => "Compliance Shadows Division",
            Division::CommunityCatalysts => "Community Catalysts Division",
            Division::EmergencyResponse => "Emergency Response Division",
        }
    }
}

/// Agent role within a division
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AgentRole {
    DivisionCommander,
    SquadLeader,
    Specialist,
    Operative,
    Scout,
    Analyst,
    Executor,
    Guardian,
    Architect,
    Strategist,
}

/// Agent memory — persistent knowledge store
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AgentMemory {
    pub short_term: Vec<MemoryEntry>,
    pub long_term: Vec<MemoryEntry>,
    pub episodic: Vec<EpisodicMemory>,
    pub semantic: HashMap<String, serde_json::Value>,
    pub max_short_term: usize,
    pub max_long_term: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub key: String,
    pub value: serde_json::Value,
    pub timestamp: u64,
    pub importance: f64,
    pub access_count: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EpisodicMemory {
    pub event: String,
    pub context: HashMap<String, String>,
    pub outcome: String,
    pub lesson_learned: String,
    pub timestamp: u64,
}

impl AgentMemory {
    pub fn new(short_term_cap: usize, long_term_cap: usize) -> Self {
        Self {
            short_term: Vec::new(),
            long_term: Vec::new(),
            episodic: Vec::new(),
            semantic: HashMap::new(),
            max_short_term: short_term_cap,
            max_long_term: long_term_cap,
        }
    }

    pub fn remember(&mut self, key: &str, value: serde_json::Value, importance: f64) {
        let entry = MemoryEntry {
            key: key.to_string(),
            value,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            importance,
            access_count: 0,
        };
        if importance >= 0.8 {
            self.long_term.push(entry);
            if self.long_term.len() > self.max_long_term {
                self.long_term.sort_by(|a, b| a.importance.partial_cmp(&b.importance).unwrap());
                self.long_term.remove(0);
            }
        } else {
            self.short_term.push(entry);
            if self.short_term.len() > self.max_short_term {
                self.short_term.remove(0);
            }
        }
    }

    pub fn record_episode(&mut self, event: &str, context: HashMap<String, String>, outcome: &str, lesson: &str) {
        self.episodic.push(EpisodicMemory {
            event: event.to_string(),
            context,
            outcome: outcome.to_string(),
            lesson_learned: lesson.to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        });
    }
}

/// Task assigned to an agent
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AgentTask {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub priority: TaskPriority,
    pub assigned_to: AgentId,
    pub status: TaskStatus,
    pub created_at: u64,
    pub deadline: Option<u64>,
    pub dependencies: Vec<Uuid>,
    pub result: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Critical,
    Apocalyptic,  // System-survival level
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskStatus {
    Queued,
    InProgress,
    Completed,
    Failed { reason: String },
    Cancelled,
}

/// Core trait all agents must implement
#[async_trait]
pub trait AutonomousAgent: Send + Sync {
    /// Unique agent identifier
    fn id(&self) -> AgentId;

    /// Human-readable agent name
    fn name(&self) -> &str;

    /// Agent's codename (e.g., "PHANTOM-SEC-001")
    fn codename(&self) -> &str;

    /// Division assignment
    fn division(&self) -> &Division;

    /// Role within division
    fn role(&self) -> &AgentRole;

    /// Cognitive level
    fn cognitive_level(&self) -> &CognitiveLevel;

    /// List of skills
    fn skills(&self) -> &[Skill];

    /// Current status
    fn status(&self) -> &AgentStatus;

    /// Health metrics
    fn health(&self) -> &AgentHealth;

    /// Initialize the agent
    async fn initialize(&mut self) -> anyhow::Result<()>;

    /// Main execution loop tick
    async fn tick(&mut self) -> anyhow::Result<()>;

    /// Process an assigned task
    async fn process_task(&mut self, task: AgentTask) -> anyhow::Result<serde_json::Value>;

    /// Handle inter-agent message
    async fn handle_message(&mut self, from: AgentId, message: AgentMessage) -> anyhow::Result<Option<AgentMessage>>;

    /// Self-evolve: improve capabilities based on experience
    async fn evolve(&mut self) -> anyhow::Result<()>;

    /// Graceful shutdown
    async fn shutdown(&mut self) -> anyhow::Result<()>;

    /// Describe capabilities in natural language
    fn describe(&self) -> String;
}

/// Inter-agent communication message
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AgentMessage {
    pub id: Uuid,
    pub from: AgentId,
    pub to: AgentId,
    pub msg_type: MessageType,
    pub payload: serde_json::Value,
    pub timestamp: u64,
    pub priority: TaskPriority,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MessageType {
    TaskAssignment,
    TaskResult,
    StatusReport,
    Alert,
    CoordinationRequest,
    CoordinationResponse,
    KnowledgeShare,
    EvolutionSync,
    Heartbeat,
    EmergencyBroadcast,
}
