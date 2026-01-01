use erc20_core::{UserDB, node::Node};

fn main() {
    let mut node = Node::new("node-1", 3);

    let mut user_db = UserDB::new();

    let addr_a = user_db.add_user(&mut node.state, "A", 1000);
    let addr_b = user_db.add_user(&mut node.state, "B", 1000);
    let addr_c = user_db.add_user(&mut node.state, "C", 1000);
    
    user_db.get_balance(&node.state, &addr_a);

    node.state.approve(&mut node.network, &addr_a, &addr_b, 300);

    node.state.transfer_from(&mut node.network, &addr_b, &addr_a, &addr_c, 300);

    node.state.execute_trade(&mut node.network, &addr_c, &addr_a, 300);

    node.state.allowance(&addr_a, &addr_b);

    node.network.print_chain();

    user_db.get_balance(&node.state, &addr_a);
    user_db.get_balance(&node.state, &addr_b);
    user_db.get_balance(&node.state, &addr_c);
}