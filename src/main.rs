mod block;
mod user;
mod network;

use user::UserDB;
use network::Network;

fn main() {
    let mut user_db = UserDB::new();
    let mut network = Network::new();

    let addr_a = user_db.add_user("A", 1000);
    let addr_b = user_db.add_user("B", 1000);
    
    user_db.get_balance(&addr_a);

    if user_db.execute_trade(&mut network ,&addr_a, &addr_b, 200) {
        println!("거래 성공");
    } else {
        println!("거래 실패");
    }

    user_db.get_balance(&addr_a);

    network.print_chain();
}