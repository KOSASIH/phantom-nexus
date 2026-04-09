//! OmniOrchestrator — Master controller for all 1000+ agents
//!
//! Manages task distribution, inter-division coordination, and global optimization.

use crate::core::*;
use crate::registry::AgentRegistry;
use log::{info, error, warn};
use std::sync::Arc;
use std::collections::VecDeque;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Global orchestration strategy
#[derive(Clone, Debug)]
pub enum OrchestrationStrategy {
    /// Round-robin task distribution
    RoundRobin,
    /// Least-loaded agent gets task
    LeastLoaded,
    /// Best-skilled agent gets task
    BestFit,
    /// AI-optimized dynamic routing
    NeuralRouting,
    /// Emergency: all agents focus on crisis
    SwarmMode,
}

/// The OmniOrchestrator — supreme controller of all agents
pub struct OmniOrchestrator {
    pub registry: Arc<AgentRegistry>,
    pub strategy: OrchestrationStrategy,
    pub task_queue: Arc<RwLock<VecDeque<AgentTask>>>,
    pub completed_tasks: Arc<RwLock<Vec<AgentTask>>>,
    pub active: bool,
    pub tick_interval_ms: u64,
    pub global_objectives: Vec<String>,
    pub epoch: u64,
}

impl OmniOrchestrator {
    pub fn new(registry: Arc<AgentRegistry>) -> Self {
        Self {
            registry,
            strategy: OrchestrationStrategy::NeuralRouting,
            task_queue: Arc::new(RwLock::new(VecDeque::new())),
            completed_tasks: Arc::new(RwLock::new(Vec::new())),
            active: false,
            tick_interval_ms: 100, // 10 ticks/second
            global_objectives: vec![
                "Maintain 99.999% network uptime".into(),
                "Optimize all DeFi vault yields above market average".into(),
                "Detect and neutralize threats in <1 second".into(),
                "Process 1M+ TPS with sub-second finality".into(),
                "Evolve agent capabilities every 24 hours".into(),
                "Maintain cross-chain bridge liquidity above $1B".into(),
                "Keep oracle data accuracy at 99.99%+".into(),
                "Ensure zero successful exploits".into(),
                "Grow network to 10,000+ nodes".into(),
                "Achieve regulatory compliance across 195 countries".into(),
            ],
            epoch: 0,
        }
    }

    /// Submit a task to the orchestrator
    pub async fn submit_task(&self, task: AgentTask) {
        let mut queue = self.task_queue.write().await;
        info!("Task submitted: {} (priority: {:?})", task.name, task.priority);
        queue.push_back(task);
    }

    /// Route task to optimal agent based on strategy
    pub async fn route_task(&self, task: &AgentTask) -> Option<AgentId> {
        match &self.strategy {
            OrchestrationStrategy::NeuralRouting => {
                self.neural_route(task).await
            }
            OrchestrationStrategy::LeastLoaded => {
                self.least_loaded_route(task).await
            }
            _ => self.round_robin_route(task).await,
        }
    }

    async fn neural_route(&self, task: &AgentTask) -> Option<AgentId> {
        // AI-optimized routing: find agent with best skill match + lowest load
        let mut best_agent: Option<(AgentId, f64)> = None;

        for division in Division::all() {
            let agents = self.registry.get_division(&division);
            for agent_id in agents {
                if let Some(agent_lock) = self.registry.get(&agent_id) {
                    if let Ok(agent) = agent_lock.try_read() {
                        let health = agent.health();
                        if health.energy_level < 0.1 { continue; } // Skip exhausted agents

                        let load_score = 1.0 - health.cognitive_load;
                        let success_score = health.success_rate;
                        let energy_score = health.energy_level;

                        let composite = load_score * 0.3 + success_score * 0.4 + energy_score * 0.3;

                        match &best_agent {
                            Some((_, best_score)) if composite > *best_score => {
                                best_agent = Some((agent_id, composite));
                            }
                            None => {
                                best_agent = Some((agent_id, composite));
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        best_agent.map(|(id, _)| id)
    }

    async fn least_loaded_route(&self, _task: &AgentTask) -> Option<AgentId> {
        let mut best: Option<(AgentId, f64)> = None;
        for division in Division::all() {
            for agent_id in self.registry.get_division(&division) {
                if let Some(agent_lock) = self.registry.get(&agent_id) {
                    if let Ok(agent) = agent_lock.try_read() {
                        let load = agent.health().cognitive_load;
                        match &best {
                            Some((_, best_load)) if load < *best_load => {
                                best = Some((agent_id, load));
                            }
                            None => best = Some((agent_id, load)),
                            _ => {}
                        }
                    }
                }
            }
        }
        best.map(|(id, _)| id)
    }

    async fn round_robin_route(&self, _task: &AgentTask) -> Option<AgentId> {
        // Simple: pick first available from each division in rotation
        let divisions = Division::all();
        let div_idx = (self.epoch as usize) % divisions.len();
        let agents = self.registry.get_division(&divisions[div_idx]);
        agents.first().copied()
    }

    /// Run one orchestration cycle
    pub async fn tick(&mut self) {
        let task = {
            let mut queue = self.task_queue.write().await;
            // Sort by priority (highest first)
            let sorted: Vec<_> = queue.drain(..).collect();
            let mut sorted = sorted;
            sorted.sort_by(|a, b| b.priority.cmp(&a.priority));
            for t in sorted.into_iter().skip(1).rev() {
                queue.push_front(t);
            }
            queue.pop_front()
        };

        if let Some(task) = task {
            if let Some(agent_id) = self.route_task(&task).await {
                if let Some(agent_lock) = self.registry.get(&agent_id) {
                    let mut agent = agent_lock.write().await;
                    match agent.process_task(task.clone()).await {
                        Ok(result) => {
                            let mut completed = AgentTask { result: Some(result), ..task };
                            completed.status = TaskStatus::Completed;
                            self.completed_tasks.write().await.push(completed);
                        }
                        Err(e) => {
                            error!("Task {} failed on agent {}: {}", task.name, agent.codename(), e);
                            let mut failed = task;
                            failed.status = TaskStatus::Failed { reason: e.to_string() };
                            self.completed_tasks.write().await.push(failed);
                        }
                    }
                }
            } else {
                warn!("No suitable agent found for task: {}", task.name);
                // Re-queue
                self.task_queue.write().await.push_back(task);
            }
        }

        self.epoch += 1;
    }

    /// Trigger global evolution cycle
    pub async fn evolve_all(&self) {
        info!("=== GLOBAL EVOLUTION CYCLE ===");
        for division in Division::all() {
            let agents = self.registry.get_division(&division);
            for agent_id in agents {
                if let Some(agent_lock) = self.registry.get(&agent_id) {
                    let mut agent = agent_lock.write().await;
                    if let Err(e) = agent.evolve().await {
                        warn!("Evolution failed for {}: {}", agent.codename(), e);
                    }
                }
            }
        }
        info!("=== EVOLUTION COMPLETE ===");
    }

    /// Emergency: switch all agents to swarm mode
    pub fn activate_swarm_mode(&mut self) {
        self.strategy = OrchestrationStrategy::SwarmMode;
        info!("⚠️  SWARM MODE ACTIVATED — All 1000+ agents focused on emergency");
    }

    /// Get system-wide statistics
    pub fn stats(&self) -> OrchestratorStats {
        OrchestratorStats {
            total_agents: self.registry.total(),
            epoch: self.epoch,
            strategy: format!("{:?}", self.strategy),
            divisions: self.registry.division_summary()
                .into_iter()
                .map(|(d, c)| (d.display_name().to_string(), c))
                .collect(),
        }
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct OrchestratorStats {
    pub total_agents: usize,
    pub epoch: u64,
    pub strategy: String,
    pub divisions: Vec<(String, usize)>,
}
