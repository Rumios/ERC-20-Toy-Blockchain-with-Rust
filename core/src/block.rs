use sha2::{Sha256, Digest};

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub data: String,
    pub prev_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, data: &str, prev_hash: &str) -> Self {
        let mut block = Block {
            index,
            data: data.to_string(),
            prev_hash: prev_hash.to_string(),
            hash: String::new()
        };

        let mut hasher = Sha256::new();
        hasher.update(format!("{} {} {}", index, data, prev_hash));
        block.hash = format!("{:x}", hasher.finalize());
        
        block
    }


}
