//! Inter-Agent Communication System

use crate::core::*;
use dashmap::DashMap;
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Message bus for inter-agent communication
pub struct MessageBus {
    queues: DashMap<AgentId, Arc<RwLock<VecDeque<AgentMessage>>>>,
    broadcast_history: Arc<RwLock<VecDeque<AgentMessage>>>,
    max_queue_size: usize,
}

impl MessageBus {
    pub fn new(max_queue_size: usize) -> Self {
        Self {
            queues: DashMap::new(),
            broadcast_history: Arc::new(RwLock::new(VecDeque::new())),
            max_queue_size,
        }
    }

    /// Send a message to a specific agent
    pub async fn send(&self, msg: AgentMessage) {
        let queue = self.queues
            .entry(msg.to)
            .or_insert_with(|| Arc::new(RwLock::new(VecDeque::new())))
            .clone();

        let mut q = queue.write().await;
        if q.len() >= self.max_queue_size {
            q.pop_front(); // Drop oldest
        }
        q.push_back(msg);
    }

    /// Broadcast to all agents
    pub async fn broadcast(&self, msg: AgentMessage) {
        let mut history = self.broadcast_history.write().await;
        history.push_back(msg.clone());
        if history.len() > 10000 {
            history.pop_front();
        }

        for entry in self.queues.iter() {
            let mut q = entry.value().write().await;
            q.push_back(msg.clone());
        }
    }

    /// Receive next message for an agent
    pub async fn receive(&self, agent_id: &AgentId) -> Option<AgentMessage> {
        if let Some(queue) = self.queues.get(agent_id) {
            let mut q = queue.write().await;
            q.pop_front()
        } else {
            None
        }
    }

    /// Register agent mailbox
    pub fn register_mailbox(&self, agent_id: AgentId) {
        self.queues.entry(agent_id)
            .or_insert_with(|| Arc::new(RwLock::new(VecDeque::new())));
    }
}
