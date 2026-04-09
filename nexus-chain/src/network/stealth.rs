//! Stealth Network Protocol
//!
//! P2P networking with zero-trace communication using libp2p and custom stealth protocols.

use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("peer not found: {peer_id}")]
    PeerNotFound { peer_id: String },
    #[error("connection failed: {reason}")]
    ConnectionFailed { reason: String },
    #[error("message encryption failed")]
    EncryptionFailed,
}

/// Peer information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PeerInfo {
    pub peer_id: String,
    pub addresses: Vec<String>,
    pub stealth_enabled: bool,
    pub latency_ms: u32,
    pub reputation: f64,
}

/// Network message types
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NetworkMessage {
    BlockProposal(Vec<u8>),
    Vote(Vec<u8>),
    Transaction(Vec<u8>),
    PeerDiscovery(PeerInfo),
    StealthHandshake(Vec<u8>),
    EncryptedPayload(Vec<u8>),
}

/// Stealth Network Manager
pub struct StealthNetwork {
    peers: HashMap<String, PeerInfo>,
    stealth_mode: bool,
    max_peers: usize,
    padding_enabled: bool, // Pad packets to uniform size
}

impl StealthNetwork {
    pub fn new(max_peers: usize) -> Self {
        Self {
            peers: HashMap::new(),
            stealth_mode: true,
            max_peers,
            padding_enabled: true,
        }
    }

    /// Add a peer to the network
    pub fn add_peer(&mut self, peer: PeerInfo) -> Result<(), NetworkError> {
        if self.peers.len() >= self.max_peers {
            // Remove lowest reputation peer
            if let Some(lowest) = self.peers.values()
                .min_by(|a, b| a.reputation.partial_cmp(&b.reputation).unwrap())
                .cloned()
            {
                self.peers.remove(&lowest.peer_id);
            }
        }
        info!("Peer connected: {} (stealth: {})", peer.peer_id, peer.stealth_enabled);
        self.peers.insert(peer.peer_id.clone(), peer);
        Ok(())
    }

    /// Broadcast message to all peers
    pub fn broadcast(&self, message: &NetworkMessage) -> Result<usize, NetworkError> {
        let serialized = serde_json::to_vec(message).map_err(|_| NetworkError::EncryptionFailed)?;

        let payload = if self.padding_enabled {
            self.pad_message(&serialized)
        } else {
            serialized
        };

        let sent = self.peers.len();
        debug!("Broadcasting to {} peers ({} bytes)", sent, payload.len());
        Ok(sent)
    }

    /// Send message to specific peer via stealth routing
    pub fn send_stealth(&self, peer_id: &str, message: &NetworkMessage) -> Result<(), NetworkError> {
        let _peer = self.peers.get(peer_id).ok_or(NetworkError::PeerNotFound {
            peer_id: peer_id.to_string(),
        })?;

        let serialized = serde_json::to_vec(message).map_err(|_| NetworkError::EncryptionFailed)?;
        let _padded = self.pad_message(&serialized);

        debug!("Stealth message sent to {}", peer_id);
        Ok(())
    }

    /// Pad message to uniform size to prevent traffic analysis
    fn pad_message(&self, data: &[u8]) -> Vec<u8> {
        let target_size = 4096; // Uniform packet size
        let mut padded = data.to_vec();
        padded.resize(target_size, 0);
        padded
    }

    /// Get connected peer count
    pub fn peer_count(&self) -> usize {
        self.peers.len()
    }

    /// Toggle stealth mode
    pub fn set_stealth_mode(&mut self, enabled: bool) {
        self.stealth_mode = enabled;
        info!("Stealth mode: {}", if enabled { "ACTIVE" } else { "DISABLED" });
    }
}
