mod network;
mod zkp_auth;
mod data;
mod consensus;

use anyhow::Result;
use network::DecentralizedNetwork;
use zkp_auth::{ZKPAuth, AuthCircuit};
use data::SignedData;
use consensus::ConsensusEngine;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    // Initialize network
    let mut network = DecentralizedNetwork::new().await?;
    network.start("/ip4/0.0.0.0/tcp/0".parse()?).await?;

    // Initialize ZKP and consensus
    let zkp_auth = ZKPAuth::setup(&mut rand::thread_rng());
    let consensus = ConsensusEngine::new();

    // Example: Publish scientific data
    let keypair = ed25519_dalek::Keypair::generate(&mut rand::rngs::OsRng);
    let data = SignedData::new(
        "/synthetic-biology/protein-folding",
        b"Research data...",
        &keypair,
    );

    // Verify and broadcast
    data.verify()?;
    if data.check_blacklist(&consensus.blacklist.read().await.clone().into_iter().collect()).is_ok() {
        // Send to network (implement network broadcast logic)
    }

    Ok(())
}
