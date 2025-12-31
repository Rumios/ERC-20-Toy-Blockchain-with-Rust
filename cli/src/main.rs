use erc20_core::{UserDB, Network, WorldState};

fn main() {
    let mut user_db = UserDB::new();
    let mut network = Network::new();
    let mut state = WorldState::new();

    let addr_a = user_db.add_user(&mut state, "A", 1000);
    let addr_b = user_db.add_user(&mut state, "B", 1000);
    let addr_c = user_db.add_user(&mut state, "C", 1000);
    
    user_db.get_balance(&state, &addr_a);

    state.approve(&mut network, &addr_a, &addr_b, 300);

    state.transfer_from(&mut network, &addr_b, &addr_a, &addr_c, 300);

    network.print_chain();

    user_db.get_balance(&state, &addr_a);
    user_db.get_balance(&state, &addr_b);
    user_db.get_balance(&state, &addr_c);
}