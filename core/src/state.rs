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

    const BASE_FEE_RATE: f64 = 0.001;
    const MIN_FEE: u64 = 100;
    const MAX_FEE_RATE: f64 = 0.01;

    // 수수료 비율은 나중에 조절
    fn calculate_fee(amount: u64, tx_type: &str) -> u64 {
        let base_rate = match tx_type {
            "transfer" => Self::BASE_FEE_RATE,
            "transfer_from" => Self::BASE_FEE_RATE,
            "approve" => 0.0,
            _ => Self::BASE_FEE_RATE,
        };

        if base_rate == 0.0 {
            return 0;
        }

        let fee = (amount as f64 * base_rate) as u64;
        fee.max(Self::MIN_FEE).min((amount as f64 * Self::MAX_FEE_RATE) as u64)
    }

    fn fee_percent(amount: u64, fee: u64) -> f64 {
        if amount == 0 {
            0.0
        } else {
            (fee as f64 / amount as f64) * 100.0
        }
    }

    pub fn approve(&mut self, network: &mut Network, owner: &str, spender: &str, amount: u64) -> bool{
        let fee = Self::calculate_fee(amount, "approve");

        if !self.balances.contains_key(owner) {
            println!("[Approve] 소유자 {} 없음", owner);
            return false;
        }

        if !self.balances.contains_key(spender) {
            println!("[Approve] 대리인 {} 없음", spender);
            return false;
        }

        if owner == spender {
            println!("[Approve] 소유자 {}, 대리인 {}이 동일", owner, spender);
            return false;
        }

        self.allowances
            .entry(owner.to_string())
            .or_insert_with(HashMap::new)
            .insert(spender.to_string(), amount);
        
        println!("[Approve] {} -> {} | amount: {}", owner, spender, amount);
        
        let approve_event = format!("[Approve] {} -> {} | amount: {}", owner, spender, amount);
        network.add_block(&approve_event);

        return true;
    }

    pub fn allowance(&self, owner: &str, spender: &str) -> u64{
        if !self.balances.contains_key(owner) {
            println!("[Allowance] 소유자 {} 없음", owner);
            return 0;
        }

        if !self.balances.contains_key(spender) {
            println!("[Allowance] 대리인 {} 없음", spender);
            return 0;
        }

        if owner == spender {
            println!("[Allowance] 소유자 {}, 대리인 {}이 동일", owner, spender);
            return 0;
        }

        self.allowances
            .get(owner)
            .and_then(|u| u.get(spender))
            .copied()
            .unwrap_or(0)
    }

    pub fn transfer_from(&mut self, network: &mut Network, spender: &str, from_addr: &str, to_addr: &str, amount: u64) -> bool {
        let fee = Self::calculate_fee(amount, "transfer_from");

        if !self.balances.contains_key(spender) {
            println!("[TxFrom] 대리인 {} 없음", spender);
            return false;
        }

        if !self.balances.contains_key(from_addr) {
            println!("[TxFrom] 소유자 {} 없음", from_addr);
            return false;
        }

        if !self.balances.contains_key(to_addr) {
            println!("[TxFrom] 수신자 {} 없음", to_addr);
            return false;
        }

        if from_addr == to_addr {
            println!("[TxFrom] 송신자 {}, 수신자 {}가 동일", from_addr, to_addr);
            return false;
        }

        let allowed = self.allowance(from_addr, spender);
        if allowed < amount {
            println!("[TxFrom] 권한 부족 | owner: {} | spender: {} | allowed: {} | amount: {}", from_addr, spender, allowed, amount);
            return false;
        }

        let from_bal = self.balances[from_addr];
        if from_bal < amount {
            println!("[TxFrom] 소유자 잔고 부족 | owner: {} | spender: {} | balance {} | need {}", from_addr, spender, from_bal, amount);
            return false;
        }

        *self.balances.get_mut(spender).unwrap() -= fee;
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

        let percent = Self::fee_percent(amount, fee);
        
        println!(
            "[TxFrom] {}가 {} 대신 {}에게 {} 전송 | fee: {} ({:.4}%)",
            spender,
            from_addr,
            to_addr,
            amount,
            fee,
            percent
        );

        let txform_event = format!("[TxFrom] {}(owner: {}) -> {} | amount: {} | fee: {} ({:.4}%)", spender, from_addr, to_addr, amount, fee, percent);
        network.add_block(&txform_event);

        return true;
    }

    pub fn execute_trade(&mut self, network: &mut Network, from_addr: &str, to_addr: &str, amount: u64) -> bool {
        let fee = Self::calculate_fee(amount, "transfer");

        if from_addr == to_addr {
            println!("[Trade] 송신자 {}, 수신자 {} 동일", from_addr, to_addr);
            return false;
        }

        if !self.balances.contains_key(to_addr) {
            println!("[Trade] 수신자 {} 없음", to_addr);
            return false;
        }

        if !self.balances.contains_key(from_addr) {
            println!("[Trade] 송신자 {} 없음", from_addr);
            return false;
        }

        let from_bal = self.balances.get(from_addr).copied().unwrap_or(0);
        if from_bal < amount + fee {
            println!(
                "[Trade] 잔고 부족 | from: {}, balance: {}, need: {} (amount: {} + fee: {})",
                from_addr,
                from_bal,
                amount + fee,
                amount,
                fee
            );            
            return false;
        }

        let percent = Self::fee_percent(amount, fee);

        println!(
            "[Trade] {} -> {} | amount: {} | fee: {} ({:.4}%)",
            from_addr,
            to_addr,
            amount,
            fee,
            percent
        );

        *self.balances.get_mut(from_addr).unwrap() -= amount + fee;
        *self.balances.get_mut(to_addr).unwrap() += amount;

        let trade_event = format!("[Trade] {} -> {} | amount: {} | fee: {} ({:.4}%)", from_addr, to_addr, amount, fee, percent);
        network.add_block(&trade_event);

        return true;
    }
}