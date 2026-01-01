pub mod pow;

use crate::block::Block;

pub trait Consensus {
    fn create_block(&self, data: &str, prev: &Block) -> Block;
}