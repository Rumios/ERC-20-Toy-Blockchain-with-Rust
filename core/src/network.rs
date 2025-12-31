use crate::block::Block;

#[derive(Debug)]
pub struct Network {
    chain: Vec<Block>,
}

impl Network {
    pub fn new() -> Self {
        let genesis_block = Block::new(0, "Genesis Block", "0");
        Network {
            chain: vec![genesis_block],
        }
    }

    pub fn add_block(&mut self, data: &str) {
        let previous_block = self.chain.last().unwrap();
        let new_index = previous_block.index + 1;
        let prev_hash = &previous_block.prev_hash;

        let new_block = Block::new(new_index, data, prev_hash);
        self.chain.push(new_block);
        println!("새로운 블록이 생성되었습니다: Index {}", new_index);
    }

    pub fn print_chain(&self) {
        println!("\n========= BLOCKCHAIN =========");
        for block in &self.chain {
            println!("Index: {}", block.index);
            println!("Data: {}", block.data);
            println!("Prev hash: {}", block.prev_hash);
            println!("Hash: {}", block.hash);
            println!("------------------------------");
        }
        println!("==============================\n");
    }
}
