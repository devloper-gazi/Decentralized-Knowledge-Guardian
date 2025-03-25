use blake3::Hash;
use ed25519_dalek::{Keypair, Signer, Verifier};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataError {
    #[error("Invalid signature")]
    InvalidSignature,
    #[error("Prohibited content")]
    ProhibitedContent,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SignedData {
    pub name: String,
    pub content: Vec<u8>,
    pub signature: Vec<u8>,
    pub public_key: Vec<u8>,
}

impl SignedData {
    pub fn new(name: &str, content: &[u8], keypair: &Keypair) -> Self {
        let signature = keypair.sign(content).to_bytes().to_vec();
        Self {
            name: name.to_string(),
            content: content.to_vec(),
            signature,
            public_key: keypair.public.to_bytes().to_vec(),
        }
    }

    pub fn verify(&self) -> Result<(), DataError> {
        let public_key = ed25519_dalek::PublicKey::from_bytes(&self.public_key)
            .map_err(|_| DataError::InvalidSignature)?;
        let signature = ed25519_dalek::Signature::from_bytes(&self.signature)
            .map_err(|_| DataError::InvalidSignature)?;
        public_key
            .verify(&self.content, &signature)
            .map_err(|_| DataError::InvalidSignature)
    }

    pub fn check_blacklist(&self, blacklist: &[Hash]) -> Result<(), DataError> {
        let hash = blake3::hash(&self.content);
        if blacklist.contains(&hash) {
            Err(DataError::ProhibitedContent)
        } else {
            Ok(())
        }
    }
}
