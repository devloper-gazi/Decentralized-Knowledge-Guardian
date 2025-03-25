use libp2p::{
    core::{upgrade, Multiaddr},
    identity,
    kad::{Kademlia, KademliaEvent, Record},
    mdns::{Mdns, MdnsEvent},
    noise, ping,
    swarm::{NetworkBehaviour, SwarmBuilder, SwarmEvent},
    tcp::TokioTcpConfig,
    PeerId, Transport,
};
use std::error::Error;

#[derive(NetworkBehaviour)]
pub struct NetworkBehavior {
    pub kademlia: Kademlia,
    pub mdns: Mdns,
    pub ping: ping::Behaviour,
}

pub struct DecentralizedNetwork {
    pub swarm: libp2p::Swarm<NetworkBehavior>,
}

impl DecentralizedNetwork {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        
        let transport = TokioTcpConfig::new()
            .upgrade(upgrade::Version::V1)
            .authenticate(noise::NoiseAuthenticated::xx(&local_key)?)
            .multiplex(libp2p::yamux::YamuxConfig::default())
            .boxed();

        let mut kademlia = Kademlia::new(local_peer_id, libp2p::kad::store::MemoryStore::new());
        kademlia.set_mode(Some(libp2p::kad::Mode::Server));

        let mdns = Mdns::new(Default::default()).await?;
        let behavior = NetworkBehavior {
            kademlia,
            mdns,
            ping: ping::Behaviour::new(ping::Config::new()),
        };

        let swarm = SwarmBuilder::new(transport, behavior, local_peer_id)
            .executor(Box::new(|fut| {
                tokio::spawn(fut);
            }))
            .build();

        Ok(Self { swarm })
    }

    pub async fn start(&mut self, addr: Multiaddr) -> Result<(), Box<dyn Error>> {
        self.swarm.listen_on(addr)?;
        loop {
            match self.swarm.select_next_some().await {
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("Listening on {}", address);
                }
                SwarmEvent::Behaviour(KademliaEvent::OutboundQueryCompleted { result, .. }) => {
                    if let Ok(libp2p::kad::QueryResult::Bootstrap(_)) = result {
                        println!("Kademlia bootstrapped successfully");
                    }
                }
                _ => {}
            }
        }
    }
}
