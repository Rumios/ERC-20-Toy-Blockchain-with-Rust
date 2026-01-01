use std::time::Instant;

use crate::block::Block;
use  super::Consensus;

// PoW 합의 구현
#[derive(Debug)]
pub struct PowConsensus {
    pub difficulty: u32,
}

impl PowConsensus {
    pub fn new(difficulty: u32) -> Self {
        PowConsensus {difficulty}
    }
}

impl Consensus for PowConsensus {
    fn create_block(&self, data: &str, prev: &Block) -> Block {
        let mut block = Block::new(prev.index + 1, data, &prev.prev_hash);

        let target = "0".repeat(self.difficulty as usize);

        let start = Instant::now();

        loop {
            let hash = block.calculate_hash();

            if hash.starts_with(&target) {
                block.hash = hash;
                break;
            }

            block.nonce += 1;
        }

        let elapsed= start.elapsed();

        println!("[PoW] Block mined: hash = {} | nonce = {} | time = {:?}", block.hash, block.nonce, elapsed);

        block
    }    
}