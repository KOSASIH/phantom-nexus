//! Post-Quantum Cryptography Module
//!
//! Implements lattice-based and hash-based signatures resistant to quantum attacks.

pub mod quantum_resistant;
pub mod ring_signatures;

pub use quantum_resistant::{QuantumKeyPair, QuantumSigner};
