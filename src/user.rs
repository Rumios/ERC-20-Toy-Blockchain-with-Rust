use rand::rngs::StdRng;  
use rand::SeedableRng;
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use tiny_keccak::{Hasher, Keccak};
use hex;
use std::collections::HashMap;

use crate::network::Network;

// Keccak-256 해시
fn keccak256(input: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak::v256();
    let mut output = [0u8; 32];
    hasher.update(input);
    hasher.finalize(&mut output);
    output
}

// key는 wallet 메서드로 분류할 예정.
#[derive(Debug, PartialEq, Clone)]
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

/*
Vec -> HashMap

실제 이더리움에서는 Trie 사용. (접두사로 관리)

그러나 소형 프로젝트이므로 HashMap 채택
*/
#[derive(Debug)]
pub struct UserDB {
    users: HashMap<String, User>,
    allowances: HashMap<String, HashMap<String, u64>>
}

impl UserDB {
    pub fn new() -> Self {
        UserDB {
            users: HashMap::new(),
            allowances: HashMap::new(),
        }
    }

    // 위임 (모든 것은 주소로 처리)
    pub fn approve(&mut self, network: &mut Network, owner: &str, spender: &str, amount: u64) -> bool {
        if !self.users.contains_key(owner) {
            println!("존재하지 않는 주소입니다.");
            return false;
        }

        let owner_name = self.users.get(owner)
            .map(|u| &u.name)
            .unwrap();
        let spender_name = self.users.get(spender)
            .map(|u| &u.name)
            .unwrap();

        // owner의 spender HashMap 생성 및 업데이트
        self.allowances
            .entry(owner.to_string())
            .or_insert_with(HashMap::new)
            .insert(spender.to_string(), amount);

        println!("{}가 {}에게 {} 토큰을 위임", 
            owner_name,
            spender_name, 
            amount
        );

        let approve_event = format!("위임: {} -> {} ({})", owner_name, spender_name, amount);
        network.add_block(&approve_event);

        return true;
    }

    // 남은 권한 조회
    pub fn allowance(&self, owner: &str, spender: &str) -> u64{
        return self.allowances
            .get(owner)
            .and_then(|u| u.get(spender))
            .copied()
            .unwrap_or(0);
    }

    // 권한 사용
    pub fn transfer_from(&mut self, network: &mut Network, spender: &str, from_addr: &str, to_addr: &str, amount: u64) -> bool {
        // 사용자 존재 여부 확인
        if !self.users.contains_key(spender) {
            println!("사용자 {} 없음", spender);
            return false;
        }

        if !self.users.contains_key(from_addr) {
            println!("주인 {} 없음", from_addr);
            return false;
        }

        if !self.users.contains_key(to_addr) {
            println!("대상 {} 없음", to_addr);
            return false;
        } 

        // 예외 처리
        let allowed = self.allowance(from_addr, spender);
        if allowed < amount {
            println!("권한 부족: {} < {}", allowed, amount);
            return false;
        }

        let from_bal = self.users[from_addr].balance;
        if from_bal < amount {
            println!("잔고 부족: {} < {}", from_bal, amount);
            return false;
        }

        // 이체
        self.users.get_mut(from_addr).unwrap().balance -= amount;
        self.users.get_mut(to_addr).unwrap().balance += amount;

        if let Some(spender_map) = self.allowances.get_mut(from_addr) {
            if let Some(allowance) = spender_map.get_mut(spender) {
                *allowance -= amount;
                if *allowance == 0 {
                    spender_map.remove(spender);
                }
            }
        }
        
        let spender_name = self.users[spender].name.clone();
        let from_name = self.users[from_addr].name.clone();
        let to_name = self.users[to_addr].name.clone();

        // 기록
        let transfer_event = format!("거래: {}({}) -> {} | 양: {}", spender_name, from_name, to_name, amount);
        network.add_block(&transfer_event);

        return true;
    }

    pub fn add_user(&mut self, name: &str, balance: u64) -> String {
        let new_user = User::new_random(name, balance);
        let addr = new_user.address.clone();

        // self.users.push(new_user);
        self.users.insert(addr.clone(), new_user);

        println!("사용자 추가됨: {} (address: {})", name, &addr);
        return addr;
    }

    pub fn execute_trade(&mut self, network: &mut Network ,from_addr: &str, to_addr: &str, amount: u64) -> bool {
        if from_addr == to_addr {
            println!("송신자와 수신자가 같습니다.");
            return false;
        }

        if !self.users.contains_key(from_addr) || !self.users.contains_key(to_addr) {
            return false;
        }

        if self.users[from_addr].balance < amount {
            println!("잔액이 부족합니다.");
            return false;
        }

        let from_name = self.users[from_addr].name.clone();
        let to_name = self.users[to_addr].name.clone();

        self.users.get_mut(from_addr).unwrap().balance -= amount;
        self.users.get_mut(to_addr).unwrap().balance += amount;

        let transaction = format!(
            "거래: {} -> {} | 양: {}", from_name, to_name, amount
        );
        network.add_block(&transaction);

        return true;
    }

    pub fn get_balance(&self, user_addr: &str) {
        println!("============ 잔고 ===========\n");
        match self.users.get(user_addr) {
            Some(user) => {
                println!("이름: {}", user.name);
                println!("주소: {}", user.address);
                println!("잔액: {}", user.balance);
            }
            None => println!("사용자를 찾을 수 없습니다."),
        }
    }
}
