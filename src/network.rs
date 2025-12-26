use crate::block::Block;  // block 모듈의 Block 가져옴

#[derive(Debug)]
pub struct Network {
    chain: Vec<Block>,
}

impl Network {
    pub fn new() -> Self {
        let genesis_block = Block::new(0, "Genesis Block", 0);
        Network {
            chain: vec![genesis_block],
        }
    }

    pub fn add_block(&mut self, data: &str) {
        let previous_block = self.chain.last().unwrap();
        let new_index = previous_block.index + 1;
        let previous_index = previous_block.index;
        let new_block = Block::new(new_index, data, previous_index);
        self.chain.push(new_block);
        println!("새로운 블록이 생성되었습니다: Index {}", new_index);
    }

    pub fn print_chain(&self) {
        println!("\n========= BLOCKCHAIN =========");
        for block in &self.chain {
            println!("Index: {}", block.index);
            println!("Data: {}", block.data);
            println!("Prev index: {}", block.previous_index);
            println!("------------------------------");
        }
        println!("==============================\n");
    }
}
