mod user_mod;
use user_mod::user_mod::{hello,initial};

fn main() {
    hello();
    //TODO: add init
    let _user = initial();

    println!("User name: {} and health {}", _user.name, _user.health);
}
