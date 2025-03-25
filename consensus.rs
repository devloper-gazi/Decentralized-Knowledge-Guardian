use std::collections::HashSet;
use tokio::sync::RwLock;

pub struct ConsensusEngine {
    blacklist: RwLock<HashSet<blake3::Hash>>,
    votes: RwLock<Vec<blake3::Hash>>,
}

impl ConsensusEngine {
    pub fn new() -> Self {
        Self {
            blacklist: RwLock::new(HashSet::new()),
            votes: RwLock::new(Vec::new()),
        }
    }

    pub async fn propose_blacklist(&self, hash: blake3::Hash) {
        let mut votes = self.votes.write().await;
        votes.push(hash);
    }

    pub async fn finalize_blacklist(&self, threshold: usize) {
        let votes = self.votes.read().await;
        let mut frequency = std::collections::HashMap::new();
        for hash in votes.iter() {
            *frequency.entry(hash).or_insert(0) += 1;
        }

        let mut blacklist = self.blacklist.write().await;
        for (hash, count) in frequency {
            if count >= threshold {
                blacklist.insert(*hash);
            }
        }
    }
}
