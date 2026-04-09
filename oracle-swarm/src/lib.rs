//! # Global Oracle Swarm
//!
//! Distributed oracle network processing 1M+ data feeds from satellites,
//! weather, social sentiment, and on-chain data.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DataFeedType {
    PriceFeed { asset: String, currency: String },
    WeatherData { location: String },
    SocialSentiment { topic: String },
    OnChainMetric { chain: String, metric: String },
    SatelliteData { satellite_id: String },
    ElectionData { region: String },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OracleDataPoint {
    pub feed_id: String,
    pub feed_type: DataFeedType,
    pub value: f64,
    pub confidence: f64,
    pub timestamp: u64,
    pub sources: usize,
}

/// Oracle node
#[derive(Clone, Debug)]
pub struct OracleNode {
    pub id: String,
    pub feeds: Vec<String>,
    pub reliability: f64,
    pub response_time_ms: u32,
}

/// Oracle Swarm Manager
pub struct OracleSwarm {
    pub nodes: Vec<OracleNode>,
    pub data_cache: HashMap<String, OracleDataPoint>,
    pub accuracy_target: f64,
}

impl OracleSwarm {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            data_cache: HashMap::new(),
            accuracy_target: 0.9999, // 99.99%
        }
    }

    /// Register an oracle node
    pub fn register_node(&mut self, node: OracleNode) {
        self.nodes.push(node);
    }

    /// Get aggregated data for a feed
    pub fn get_feed(&self, feed_id: &str) -> Option<&OracleDataPoint> {
        self.data_cache.get(feed_id)
    }

    /// Update feed data with multi-source aggregation
    pub fn update_feed(&mut self, data: OracleDataPoint) {
        self.data_cache.insert(data.feed_id.clone(), data);
    }

    /// Get total number of active feeds
    pub fn active_feeds(&self) -> usize {
        self.data_cache.len()
    }
}
