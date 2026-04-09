//! Agent Registry — Central catalog of all 1000+ agents
//!
//! Manages agent lifecycle, lookup, and division organization.

use crate::core::*;
use dashmap::DashMap;
use log::{info, warn};
use std::sync::Arc;
use uuid::Uuid;

/// Global agent registry
pub struct AgentRegistry {
    agents: DashMap<AgentId, Arc<tokio::sync::RwLock<Box<dyn AutonomousAgent>>>>,
    by_division: DashMap<Division, Vec<AgentId>>,
    by_role: DashMap<AgentRole, Vec<AgentId>>,
    by_codename: DashMap<String, AgentId>,
    total_registered: std::sync::atomic::AtomicUsize,
}

impl AgentRegistry {
    pub fn new() -> Self {
        Self {
            agents: DashMap::new(),
            by_division: DashMap::new(),
            by_role: DashMap::new(),
            by_codename: DashMap::new(),
            total_registered: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    /// Register a new agent
    pub fn register(&self, agent: Box<dyn AutonomousAgent>) {
        let id = agent.id();
        let division = agent.division().clone();
        let role = agent.role().clone();
        let codename = agent.codename().to_string();

        self.agents.insert(id, Arc::new(tokio::sync::RwLock::new(agent)));
        self.by_division.entry(division.clone()).or_default().push(id);
        self.by_role.entry(role.clone()).or_default().push(id);
        self.by_codename.insert(codename.clone(), id);
        self.total_registered.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        info!("Agent registered: {} in {:?} as {:?}", codename, division, role);
    }

    /// Get agent by ID
    pub fn get(&self, id: &AgentId) -> Option<Arc<tokio::sync::RwLock<Box<dyn AutonomousAgent>>>> {
        self.agents.get(id).map(|a| a.value().clone())
    }

    /// Get agent by codename
    pub fn get_by_codename(&self, codename: &str) -> Option<Arc<tokio::sync::RwLock<Box<dyn AutonomousAgent>>>> {
        self.by_codename.get(codename)
            .and_then(|id| self.agents.get(id.value()).map(|a| a.value().clone()))
    }

    /// Get all agents in a division
    pub fn get_division(&self, division: &Division) -> Vec<AgentId> {
        self.by_division.get(division).map(|v| v.value().clone()).unwrap_or_default()
    }

    /// Get division commander
    pub fn get_commander(&self, division: &Division) -> Option<AgentId> {
        let agents = self.get_division(division);
        for id in agents {
            if let Some(agent) = self.agents.get(&id) {
                let agent = agent.value().clone();
                // Use try_read to avoid blocking
                if let Ok(a) = agent.try_read() {
                    if *a.role() == AgentRole::DivisionCommander {
                        return Some(id);
                    }
                }
            }
        }
        None
    }

    /// Total registered agents
    pub fn total(&self) -> usize {
        self.total_registered.load(std::sync::atomic::Ordering::SeqCst)
    }

    /// List all divisions and their agent counts
    pub fn division_summary(&self) -> Vec<(Division, usize)> {
        Division::all().into_iter().map(|d| {
            let count = self.by_division.get(&d).map(|v| v.len()).unwrap_or(0);
            (d, count)
        }).collect()
    }

    /// Deregister an agent
    pub fn deregister(&self, id: &AgentId) -> bool {
        if let Some((_, _agent)) = self.agents.remove(id) {
            // Clean up indices
            for mut entry in self.by_division.iter_mut() {
                entry.value_mut().retain(|a| a != id);
            }
            for mut entry in self.by_role.iter_mut() {
                entry.value_mut().retain(|a| a != id);
            }
            self.by_codename.retain(|_, v| v != id);
            self.total_registered.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
            true
        } else {
            false
        }
    }
}
