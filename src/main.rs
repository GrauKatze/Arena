mod user;
mod shop;

use std::io::Write;
use crate::shop::ShopList;
use crate::user::User;

fn main() {
    let mut new_name = String::new();
    std::io::stdin().read_line(&mut new_name).expect("wrong line");
    let mut user = User::init(new_name);
    menu(&mut user);
}
fn menu(user: &mut User){
    let start_menu_list = ["\nОсновное меню:",
            "1. Магазин",
            "2. В бой (not work)",
            "3. Стата",
            "4. Выход"];
    for el in start_menu_list {
        println!("{}",el);
    }
    loop {

        print!("Enter num [MENU] >> ");
        std::io::stdout().flush().expect("some error");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("wrong line");
        let input: u8 = match input.trim().parse() {
            Ok(n)=>n,
            Err(err)=>{
                println!("{:?}", err);
            0}
        };
        match input {
            1 => {
                println!("Shop");
                ShopList::shops_menu(user);
            },
            2 => {
                println!("Battle");
            },
            3 => {
                println!("Your status");
                User::get_status(user);
            },
            4 => {
                println!("Exit");
                break;
            },
            _ => println!("Error"),
        }
    }
}
