//! Agent Lifecycle Management

use crate::core::*;
use crate::registry::AgentRegistry;
use log::{info, warn};
use std::sync::Arc;

/// Lifecycle manager — handles birth, evolution, and retirement of agents
pub struct LifecycleManager {
    pub registry: Arc<AgentRegistry>,
    pub evolution_interval_secs: u64,
    pub retirement_threshold: f64,  // Energy below this = retire
    pub spawn_threshold: usize,     // Min agents per division
}

impl LifecycleManager {
    pub fn new(registry: Arc<AgentRegistry>) -> Self {
        Self {
            registry,
            evolution_interval_secs: 86400, // 24 hours
            retirement_threshold: 0.05,
            spawn_threshold: 50,
        }
    }

    /// Check division health and report understaffed divisions
    pub fn health_check(&self) -> Vec<(Division, usize, bool)> {
        Division::all().into_iter().map(|d| {
            let count = self.registry.get_division(&d).len();
            let healthy = count >= self.spawn_threshold;
            (d, count, healthy)
        }).collect()
    }

    /// Get total agent count
    pub fn total_agents(&self) -> usize {
        self.registry.total()
    }
}
