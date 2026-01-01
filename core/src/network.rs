use crate::block::Block;
use crate::consensus::{Consensus, pow::PowConsensus};

#[derive(Debug)]
pub struct Network<C: Consensus> {
    chain: Vec<Block>,
    pub consensus: C,
}

// PowConsensus 사용 시에만 제공
impl Network<PowConsensus> {
    pub fn new(difficulty: u32) -> Self {
        let genesis_block = Block::new(0, "Genesis Block", "0");
        Network {
            chain: vec![genesis_block],
            consensus: PowConsensus { difficulty }
        }
    }
}

// 모든 합의 타입에 공통으로 제공
impl <C: Consensus> Network<C> {
    pub fn add_block(&mut self, data: &str) {
        let prev = self.chain.last().unwrap();

        let new_block = self.consensus.create_block(data, prev);

        self.chain.push(new_block);
    }

    pub fn print_chain(&self) {
        println!("\n========= BLOCKCHAIN =========");
        for block in &self.chain {
            println!("Index: {}", block.index);
            println!("Timestamp: {}", block.timestamp);
            println!("Data: {}", block.data);
            println!("Prev hash: {}", block.prev_hash);
            println!("Hash: {}", block.hash);
            println!("Nonce: {}", block.nonce);
            println!("------------------------------");
        }
        println!("==============================\n");
    }
}
