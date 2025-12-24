
#[derive(Debug)]
struct Block {
    index: u64,
    data: String,
    previous_hash: u64,
}

impl Block {
    fn new(index: u64, data: &str, previous_hash: u64) -> Self {
        Block {
            index,
            data: data.to_string(),
            previous_hash,
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
        let from_user = match self.users.iter_mut().find(|u| u.address == from_addr) {
            Some(user) => user,
            None => return false
        };

        let to_user = match self.users.iter_mut().find(|u| u.address == to_addr) {
            Some(user) => user,
            None => return false
        };

        if from_user.address == to_user.address{
            println!("송신자와 수신자가 같습니다.");
            return false;
        }

        if from_user.balance >= amount {
            from_user.balance -= amount;
            to_user.balance += amount;
            return true;
        }

    return false;
    }
}



fn main() {
    println!("Hello World");
}