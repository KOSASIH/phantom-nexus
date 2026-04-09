//! Nexus Node Binary
//!
//! Entry point for running a Phantom Nexus validator/full node.

use nexus_chain::consensus::PhantomBFT;
use nexus_chain::consensus::phantom_bft::Validator;
use nexus_chain::crypto::QuantumKeyPair;
use nexus_chain::crypto::quantum_resistant::PQAlgorithm;
use nexus_chain::network::StealthNetwork;
use nexus_chain::types::Address;
use log::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    info!("\n");
    info!("  👻 Phantom Nexus Node v0.1.0");
    info!("  Invisible Intelligence, Infinite Scale");
    info!("\n");

    // Generate node identity
    let keypair = QuantumKeyPair::generate(PQAlgorithm::Dilithium3)?;
    let address = Address::from_public_key(&keypair.public_key);
    info!("Node address: {}", address);
    info!("Algorithm: {:?}", keypair.algorithm);

    // Initialize network
    let mut network = StealthNetwork::new(256);
    network.set_stealth_mode(true);
    info!("Stealth network initialized (max peers: 256)");

    // Initialize consensus
    let validator = Validator {
        address: address.clone(),
        stake: 1_000_000,
        ai_score: 0.95,
        uptime: 1.0,
        blocks_produced: 0,
        slashing_count: 0,
    };

    let consensus = PhantomBFT::new(vec![validator]);
    info!("PhantomBFT consensus initialized at height {}", consensus.height());

    info!("Node ready. Waiting for peers...");

    // Keep node running
    tokio::signal::ctrl_c().await?;
    info!("Shutting down...");

    Ok(())
}
