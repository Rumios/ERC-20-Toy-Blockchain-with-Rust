use std::collections::HashMap;

use crate::network::Network;

#[derive(Debug)]
pub struct WorldState {
    pub balances: HashMap<String, u64>,
    pub allowances: HashMap<String, HashMap<String, u64>>,
}

impl WorldState {
    pub fn new() -> Self {
        WorldState {
            balances: HashMap::new(),
            allowances: HashMap::new(),
        }
    }

    pub fn approve(&mut self, network: &mut Network, owner: &str, spender: &str, amount: u64) -> bool{
        if !self.balances.contains_key(owner) {
            println!("Approve: 소유자 {} 없음", owner);
            return false;
        }

        if !self.balances.contains_key(spender) {
            println!("Approve: 대리인 {} 없음", spender);
            return false;
        }

        if owner == spender {
            println!("Approve: 소유자 {}, 대리인 {}이 동일합니다.", owner, spender);
            return false;
        }

        self.allowances
            .entry(owner.to_string())
            .or_insert_with(HashMap::new)
            .insert(spender.to_string(), amount);
        
        println!("Approve: {} -> {} ({})", owner, spender, amount);
        
        let approve_event = format!("Approve: {} -> {} ({})", owner, spender, amount);
        network.add_block(&approve_event);

        return true;
    }

    pub fn allowance(&self, owner: &str, spender: &str) -> u64{
        if !self.balances.contains_key(owner) {
            println!("Allowance: 소유자 {} 없음", owner);
            return 0;
        }

        if !self.balances.contains_key(spender) {
            println!("Allowance: 대리인 {} 없음", spender);
            return 0;
        }

        if owner == spender {
            println!("Allowance: 소유자 {}, 대리인 {}이 동일합니다.", owner, spender);
            return 0;
        }

        self.allowances
            .get(owner)
            .and_then(|u| u.get(spender))
            .copied()
            .unwrap_or(0)
    }

    pub fn transfer_from(&mut self, network: &mut Network, spender: &str, from_addr: &str, to_addr: &str, amount: u64) -> bool {
        if !self.balances.contains_key(spender) {
            println!("TxFrom: 대리인 {} 없음", spender);
            return false;
        }

        if !self.balances.contains_key(from_addr) {
            println!("TxFrom: 소유자 {} 없음", from_addr);
            return false;
        }

        if !self.balances.contains_key(to_addr) {
            println!("TxFrom: 수신자 {} 없음", to_addr);
            return false;
        }

        if from_addr == to_addr {
            println!("TxFrom: 송신자 {}, 수신자 {}가 동일합니다.", from_addr, to_addr);
            return false;
        }

        let allowed = self.allowance(from_addr, spender);
        if allowed < amount {
            println!("TxFrom: {} -> {} 권한 부족: {} < {}", from_addr, spender, allowed, amount);
            return false;
        }

        let from_bal = self.balances[from_addr];
        if from_bal < amount {
            println!("TxFrom: 소유자 {} 잔고 부족 | 대리인 {}", from_addr, spender);
            return false;
        }

        *self.balances.get_mut(from_addr).unwrap() -= amount;
        *self.balances.get_mut(to_addr).unwrap() += amount;

        if let Some(spender_map) = self.allowances.get_mut(from_addr) {
            if let Some(allowance) = spender_map.get_mut(spender) {
                *allowance -= amount;
                if *allowance == 0 {
                    spender_map.remove(spender);
                }
            }
        }

        let txform_event = format!("TxFrom: {}(소유자: {}) -> {} | 양: {}", spender, from_addr, to_addr, amount);
        network.add_block(&txform_event);

        return true;
    }

    pub fn execute_trade(&mut self, network: &mut Network, from_addr: &str, to_addr: &str, amount: u64) -> bool {
        if from_addr == to_addr {
            println!("송신자 {}, 수신자 {}가 동일합니다.", from_addr, to_addr);
            return false;
        }

        if !self.balances.contains_key(to_addr) {
            println!("Trade: 수신자 {} 없음", to_addr);
            return false;
        }

        if !self.balances.contains_key(to_addr) {
            println!("Trade: 수신자 {} 없음", to_addr);
            return false;
        }

        if self.balances[from_addr] < amount {
            println!("Trade: {} 잔고가 부족합니다.", from_addr);
            return false;
        }

        *self.balances.get_mut(from_addr).unwrap() -= amount;
        *self.balances.get_mut(to_addr).unwrap() += amount;

        let trade_event = format!("Trade: {} -> {} | 양: {}", from_addr, to_addr, amount);
        network.add_block(&trade_event);

        return true;
    }
}