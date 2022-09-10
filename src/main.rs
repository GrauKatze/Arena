mod user;
mod shop;

use std::io::Write;
use crate::shop::{ShopList};

fn main() {
    menu();
}
fn menu(){
    let _start_menu_list = ["\nОсновное меню:",
            "1. Магазин",
            "2. В бой",
            "3. Стата",
            "4. Выход"];
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
                println!("one");
                ShopList::shops_menu();
                break;
            },
            2 => {
                println!("two");
                break;
            }
            _ => println!("Error"),
        }
    }
}
