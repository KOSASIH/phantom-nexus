//! Swarm Intelligence Coordination
//!
//! Thousands of AI agents collaborating like a bee swarm to optimize
//! yield farming, arbitrage, and risk management.

use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use rand::Rng;

/// Agent role in the swarm
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AgentRole {
    YieldFarmer,
    Arbitrageur,
    RiskManager,
    LiquidityProvider,
    MarketMaker,
    Sentinel,  // Security monitor
}

/// Individual swarm agent
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SwarmAgent {
    pub id: String,
    pub role: AgentRole,
    pub position: Vec<f64>,    // Position in solution space
    pub velocity: Vec<f64>,    // Movement vector
    pub best_position: Vec<f64>,
    pub best_fitness: f64,
    pub energy: f64,           // Agent energy (depletes over time)
    pub experience: u64,       // Number of completed tasks
}

impl SwarmAgent {
    pub fn new(id: &str, role: AgentRole, dimensions: usize) -> Self {
        let mut rng = rand::thread_rng();
        let position: Vec<f64> = (0..dimensions).map(|_| rng.gen_range(-1.0..1.0)).collect();
        let velocity: Vec<f64> = (0..dimensions).map(|_| rng.gen_range(-0.1..0.1)).collect();

        Self {
            id: id.to_string(),
            role,
            position: position.clone(),
            velocity,
            best_position: position,
            best_fitness: f64::NEG_INFINITY,
            energy: 100.0,
            experience: 0,
        }
    }

    /// Update agent position using PSO (Particle Swarm Optimization)
    pub fn update(&mut self, global_best: &[f64], w: f64, c1: f64, c2: f64) {
        let mut rng = rand::thread_rng();
        let dims = self.position.len();

        for i in 0..dims {
            let r1: f64 = rng.gen();
            let r2: f64 = rng.gen();

            // PSO velocity update
            self.velocity[i] = w * self.velocity[i]
                + c1 * r1 * (self.best_position[i] - self.position[i])
                + c2 * r2 * (global_best[i] - self.position[i]);

            // Clamp velocity
            self.velocity[i] = self.velocity[i].clamp(-1.0, 1.0);

            // Position update
            self.position[i] += self.velocity[i];
        }

        self.energy -= 0.1; // Energy decay
    }

    /// Evaluate fitness at current position
    pub fn evaluate(&mut self, fitness: f64) {
        if fitness > self.best_fitness {
            self.best_fitness = fitness;
            self.best_position = self.position.clone();
        }
        self.experience += 1;
    }
}

/// Swarm coordinator managing all agents
pub struct SwarmCoordinator {
    pub agents: Vec<SwarmAgent>,
    pub global_best_position: Vec<f64>,
    pub global_best_fitness: f64,
    pub dimensions: usize,
    pub iteration: u64,
    // PSO parameters
    pub inertia_weight: f64,
    pub cognitive_coefficient: f64,
    pub social_coefficient: f64,
}

impl SwarmCoordinator {
    /// Create a new swarm coordinator
    pub fn new(num_agents: usize, dimensions: usize) -> Self {
        let roles = [
            AgentRole::YieldFarmer,
            AgentRole::Arbitrageur,
            AgentRole::RiskManager,
            AgentRole::LiquidityProvider,
            AgentRole::MarketMaker,
            AgentRole::Sentinel,
        ];

        let agents: Vec<SwarmAgent> = (0..num_agents)
            .map(|i| {
                let role = roles[i % roles.len()].clone();
                SwarmAgent::new(&format!("agent-{:04}", i), role, dimensions)
            })
            .collect();

        Self {
            agents,
            global_best_position: vec![0.0; dimensions],
            global_best_fitness: f64::NEG_INFINITY,
            dimensions,
            iteration: 0,
            inertia_weight: 0.729,
            cognitive_coefficient: 1.49445,
            social_coefficient: 1.49445,
        }
    }

    /// Run one optimization iteration
    pub fn iterate<F>(&mut self, fitness_fn: F)
    where
        F: Fn(&[f64], &AgentRole) -> f64,
    {
        // Evaluate all agents
        for agent in &mut self.agents {
            let fitness = fitness_fn(&agent.position, &agent.role);
            agent.evaluate(fitness);

            if fitness > self.global_best_fitness {
                self.global_best_fitness = fitness;
                self.global_best_position = agent.position.clone();
                info!("New global best: {:.6} by {} ({:?})",
                    fitness, agent.id, agent.role);
            }
        }

        // Update positions
        let global_best = self.global_best_position.clone();
        for agent in &mut self.agents {
            agent.update(&global_best, self.inertia_weight,
                        self.cognitive_coefficient, self.social_coefficient);
        }

        self.iteration += 1;
        debug!("Swarm iteration {}: best fitness = {:.6}",
            self.iteration, self.global_best_fitness);
    }

    /// Get agents by role
    pub fn agents_by_role(&self, role: &AgentRole) -> Vec<&SwarmAgent> {
        self.agents.iter().filter(|a| &a.role == role).collect()
    }

    /// Spawn new agents
    pub fn spawn(&mut self, count: usize, role: AgentRole) {
        let start_id = self.agents.len();
        for i in 0..count {
            self.agents.push(SwarmAgent::new(
                &format!("agent-{:04}", start_id + i),
                role.clone(),
                self.dimensions,
            ));
        }
        info!("Spawned {} {:?} agents (total: {})", count, role, self.agents.len());
    }

    /// Remove depleted agents
    pub fn prune(&mut self) -> usize {
        let before = self.agents.len();
        self.agents.retain(|a| a.energy > 0.0);
        let pruned = before - self.agents.len();
        if pruned > 0 {
            info!("Pruned {} depleted agents (remaining: {})", pruned, self.agents.len());
        }
        pruned
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swarm_creation() {
        let swarm = SwarmCoordinator::new(100, 10);
        assert_eq!(swarm.agents.len(), 100);
        assert_eq!(swarm.dimensions, 10);
    }

    #[test]
    fn test_swarm_optimization() {
        let mut swarm = SwarmCoordinator::new(50, 5);

        // Simple sphere function optimization
        let fitness_fn = |pos: &[f64], _role: &AgentRole| -> f64 {
            -pos.iter().map(|x| x * x).sum::<f64>()
        };

        for _ in 0..100 {
            swarm.iterate(&fitness_fn);
        }

        // Should converge towards 0
        assert!(swarm.global_best_fitness > -1.0);
    }
}
