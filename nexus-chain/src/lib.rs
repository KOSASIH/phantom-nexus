//! # Nexus Chain
//!
//! Quantum-resistant Layer-1 blockchain powering the Phantom Nexus ecosystem.
//! Features PhantomBFT consensus, post-quantum cryptography, and stealth networking.

pub mod consensus;
pub mod crypto;
pub mod network;
pub mod storage;
pub mod types;

pub use types::{Block, Transaction, Address};
