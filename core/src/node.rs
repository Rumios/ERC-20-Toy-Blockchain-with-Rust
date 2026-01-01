use crate::network::Network;
use crate::state::WorldState;
use crate::consensus::pow::PowConsensus;

#[derive(Debug)]
pub struct Node {
    pub id: String,
    pub network: Network<PowConsensus>,
    pub state: WorldState,
}

impl Node {
    pub fn new(id: &str, difficulty: u32) -> Self {
        let network = Network::new(difficulty);
        let state = WorldState::new();

        Node {
            id: id.to_string(),
            network,
            state,
        }
    }

    pub fn print_balances(&self) {
        println!("===== Node {} Balances =====", self.id);
        for (addr, bal) in &self.state.balances {
            println!("addr: {} | balance: {}", addr, bal);
        }
        println!("============================");
    }

    pub fn print_chain(&self) {
        println!("===== Node {} Chain =====", self.id);
        self.network.print_chain();
    }
}

