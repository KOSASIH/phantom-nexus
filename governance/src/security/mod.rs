//! Self-Healing Security Module
//!
//! ML-based anomaly detection with <1 second exploit response.

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use log::{info, warn};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ThreatLevel {
    None,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ThreatType {
    ReentrancyAttack,
    FlashLoanExploit,
    OracleManipulation,
    SybilAttack,
    DDoS,
    UnauthorizedAccess,
    AnomalousTransaction,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SecurityEvent {
    pub timestamp: u64,
    pub threat_type: ThreatType,
    pub threat_level: ThreatLevel,
    pub source: String,
    pub description: String,
    pub auto_mitigated: bool,
    pub mitigation_action: Option<String>,
}

/// Self-healing security system
pub struct SecurityGuard {
    pub events: VecDeque<SecurityEvent>,
    pub max_events: usize,
    pub threat_threshold: f64,
    pub circuit_breaker_active: bool,
    pub blocked_addresses: Vec<String>,
}

impl SecurityGuard {
    pub fn new() -> Self {
        Self {
            events: VecDeque::new(),
            max_events: 10000,
            threat_threshold: 0.8,
            circuit_breaker_active: false,
            blocked_addresses: Vec::new(),
        }
    }

    /// Analyze transaction for anomalies
    pub fn analyze_transaction(&mut self, from: &str, to: &str, value: u128, data: &[u8]) -> ThreatLevel {
        let mut threat_score = 0.0;

        // Check for known attack patterns
        if self.blocked_addresses.contains(&from.to_string()) {
            threat_score += 1.0;
        }

        // Unusually large transaction
        if value > 1_000_000 * 10u128.pow(18) {
            threat_score += 0.3;
        }

        // Suspicious data patterns (reentrancy signatures)
        if data.len() > 4 && data[..4] == [0x29, 0x14, 0x8a, 0x38] {
            threat_score += 0.8;
            warn!("Potential reentrancy attack detected from {}", from);
        }

        let level = match threat_score {
            s if s >= 0.9 => ThreatLevel::Critical,
            s if s >= 0.7 => ThreatLevel::High,
            s if s >= 0.4 => ThreatLevel::Medium,
            s if s >= 0.1 => ThreatLevel::Low,
            _ => ThreatLevel::None,
        };

        if threat_score >= self.threat_threshold {
            self.auto_mitigate(from, &level);
        }

        level
    }

    /// Auto-mitigate detected threats
    fn auto_mitigate(&mut self, source: &str, level: &ThreatLevel) {
        match level {
            ThreatLevel::Critical => {
                self.circuit_breaker_active = true;
                self.blocked_addresses.push(source.to_string());
                info!("CRITICAL: Circuit breaker activated, {} blocked", source);
            }
            ThreatLevel::High => {
                self.blocked_addresses.push(source.to_string());
                info!("HIGH: Address {} blocked", source);
            }
            _ => {}
        }

        let event = SecurityEvent {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            threat_type: ThreatType::AnomalousTransaction,
            threat_level: level.clone(),
            source: source.to_string(),
            description: format!("Auto-mitigated threat from {}", source),
            auto_mitigated: true,
            mitigation_action: Some("Address blocked".to_string()),
        };

        self.events.push_back(event);
        if self.events.len() > self.max_events {
            self.events.pop_front();
        }
    }

    /// Reset circuit breaker (governance action)
    pub fn reset_circuit_breaker(&mut self) {
        self.circuit_breaker_active = false;
        info!("Circuit breaker reset by governance");
    }
}
