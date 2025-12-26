#[derive(Debug)]
pub struct Block {
    pub index: u64,
    pub data: String,
    pub previous_index: u64,
}

impl Block {
    pub fn new(index: u64, data: &str, previous_index: u64) -> Self {
        Block {
            index,
            data: data.to_string(),
            previous_index,
        }
    }
}
