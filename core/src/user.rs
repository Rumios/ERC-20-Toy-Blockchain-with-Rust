use std::collections::HashMap;
use crate::wallet::Wallet;
use crate::state::WorldState;

#[derive(Debug, PartialEq, Clone)]
pub struct User {
    pub name: String,
    pub wallet: Wallet,
}

#[derive(Debug)]
pub struct UserDB {
    pub users: HashMap<String, User>,
}

impl UserDB {
    pub fn new() -> Self {
        UserDB {
            users: HashMap::new(),
        }
    }

    pub fn add_user(&mut self, state: &mut WorldState, name: &str, balance: u64) -> String {
        let wallet = Wallet::new_random();
        let addr = wallet.address.clone();

        self.users.insert(addr.clone(), User {
            name: name.to_string(),
            wallet,
        });
        state.balances.insert(addr.clone(), balance);

        println!("사용자 추가됨: {} (address: {})", name, &addr);
        return addr;
    }

        pub fn get_balance(&self, state: &WorldState, user_addr: &str) {
        println!("============ 잔고 ===========\n");
        match self.users.get(user_addr) {
            Some(user) => {
                println!("이름: {}", user.name);
                println!("주소: {}", user.wallet.address);
                println!("잔액: {}", state.balances[user_addr]);
            }
            None => println!("사용자를 찾을 수 없습니다."),
        }
    }
}
