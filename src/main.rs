use std::{vec};


#[derive(Debug)]
struct Block {
    index: u64,
    data: String,
    previous_index: u64,
}

impl Block {
    fn new(index: u64, data: &str, previous_index: u64) -> Self {
        Block {
            index,
            data: data.to_string(),
            previous_index,
        }
    }
}

// User
#[derive(Debug)]
struct User {
    name: String,
    address: String,
    balance: u64,
}

// 주소 랜덤 생성
impl User {
    fn new(name: &str, address: &str, balance: u64) -> Self {
        User {
            name: name.to_string(),
            address: address.to_string(),
            balance, 
        }
    }
}

// UserDB
#[derive(Debug)]
struct UserDB {
    users: Vec<User>
}

impl UserDB {
    fn new() -> Self {
        UserDB {
            users:vec![
                User::new("Rumio", "0xA", 1000),
                User::new("Alice", "0xB", 100),
            ]
        }
    }

    fn execute_trade(&mut self, from_addr: &str, to_addr: &str, amount: u64) -> bool {

        // 송신자, 수신자 idx 확인
        let from_idx = match self.users.iter().position(|u| u.address == from_addr) {
            Some(idx) => idx,
            None => return false
        };

        let to_idx = match self.users.iter().position(|u| u.address == to_addr) {
            Some(idx) => idx,
            None => return false
        };
        
        // 동일 인물 확인
        if from_idx == to_idx {
            println!("송신자와 수신자가 같습니다.");
            return false;
        }

        // 잔액 확인
        if self.users[from_idx].balance < amount {
            println!("잔액이 부족합니다.");
            return false;
        }

        self.users[from_idx].balance -= amount;
        self.users[to_idx].balance += amount;

        return true;
    }

    fn get_balance(&self, user_addr: &str) {
        println!("============ 잔고 ===========\n");
        match self.users.iter().find(|u| u.address == user_addr) {
            Some(user) => {
                println!("이름: {}", user.name);
                println!("주소: {}", user.address);
                println!("잔액: {}", user.balance);
            }
            None => println!("사용자를 찾을 수 없습니다."),
        }
    }
}


// Network (chaining)
#[derive(Debug)]
struct Network {
    chain: Vec<Block>,
}

impl Network {
    fn new() -> Self {
        let genesis_block = Block::new(0, "Genesis Block", 0);
        Network {
            chain: vec![genesis_block],
        }
    }

    fn add_block(&mut self, data: &str) {
        // 이전 블록 가져오기
        /*
            self.chain.last() : vector의 마지막 요소 Option<&Block> 타입으로 변환
         */
        let previous_block = self.chain.last().unwrap();

        let new_index = previous_block.index + 1;

        let previous_index = previous_block.index;

        let new_block = Block::new(new_index, data, previous_index);
        self.chain.push(new_block);

        println!("새로운 블록이 생성되었습니다: Index {}", new_index);
    }

     fn print_chain(&self) {
        println!("\n========= BLOCKCHAIN =========");
        for block in &self.chain {
            println!("Index: {}", block.index);
            println!("Data: {}", block.data);
            println!("Prev Hash: {}", block.previous_index);
            println!("------------------------------");
        }
        println!("==============================\n");
    }
}

fn main() {
    let mut user_db = UserDB::new();
    let mut network = Network::new();

    println!(">>> system start\n");

    user_db.get_balance("0xA");

    let sender = "0xA";
    let receiver = "0xB";
    let amount = 200;

    if user_db.execute_trade(sender, receiver, amount) {
        let transaction = format!("Tx: {} -> {} | Amt: {}", sender, receiver, amount);
        network.add_block(&transaction);

    } else {
        println!(">>> trade failed\n");
    }

    if user_db.execute_trade(sender, receiver, amount) {
        let transaction = format!("Tx: {} -> {} | Amt: {}", sender, receiver, amount);
        network.add_block(&transaction);

    } else {
        println!(">>> trade failed\n");
    }

    user_db.get_balance("0xA");
    network.print_chain();
} 