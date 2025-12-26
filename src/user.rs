use rand::rngs::StdRng;  
use rand::SeedableRng;
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use tiny_keccak::{Hasher, Keccak};
use hex;

use crate::network::Network;

// Keccak-256 해시
fn keccak256(input: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak::v256();
    let mut output = [0u8; 32];
    hasher.update(input);
    hasher.finalize(&mut output);
    output
}

#[derive(Debug)]
pub struct User {
    pub name: String,
    pub address: String,
    pub private_key: String,
    pub balance: u64,
}

impl User {
    pub fn new(name: &str, address: &str, balance: u64) -> Self {
        User {
            name: name.to_string(),
            address: address.to_string(),
            private_key: String::new(), // 빈 문자열
            balance,
        }
    }

    pub fn new_random(name: &str, balance: u64) -> Self {
        let (address, private_key) = User::generate_address();
        return User {
            name: name.to_string(),
            address,
            private_key,
            balance,
        }
    }

    pub fn generate_address() -> (String, String) {  
        let secp = Secp256k1::new(); 
        let mut rng = StdRng::from_entropy();
        let secret_key = SecretKey::new(&mut rng as &mut dyn rand::RngCore);
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        let public_key_bytes = public_key.serialize_uncompressed();
        let hash = keccak256(&public_key_bytes[1..]);
        let address_bytes = &hash[12..];
        let address = format!("0x{}", hex::encode(address_bytes));  
        let private_key_hex = hex::encode(secret_key.secret_bytes());

        return (address, private_key_hex);
    }
}

#[derive(Debug)]
pub struct UserDB {
    users: Vec<User>,
}

impl UserDB {
    pub fn new() -> Self {
        UserDB { users: vec![] }
    }

    pub fn add_user(&mut self, name: &str, balance: u64) -> String {
        let new_user = User::new_random(name, balance);
        let addr = new_user.address.clone();
        self.users.push(new_user);
        println!("사용자 추가됨: {} (address: {})", name, &addr);
        return addr;
    }

    pub fn execute_trade(&mut self, network: &mut Network ,from_addr: &str, to_addr: &str, amount: u64) -> bool {
        let from_idx = match self.users.iter().position(|u| u.address == from_addr) {
            Some(idx) => idx,
            None => return false,
        };

        let to_idx = match self.users.iter().position(|u| u.address == to_addr) {
            Some(idx) => idx,
            None => return false,
        };

        if from_idx == to_idx {
            println!("송신자와 수신자가 같습니다.");
            return false;
        }

        if self.users[from_idx].balance < amount {
            println!("잔액이 부족합니다.");
            return false;
        }

        self.users[from_idx].balance -= amount;
        self.users[to_idx].balance += amount;

        let transaction = format!("거래: {} -> {} | 양: {}", self.users[from_idx].name, self.users[to_idx].name, amount);
        network.add_block(&transaction);

        return true;
    }

    pub fn get_balance(&self, user_addr: &str) {
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
