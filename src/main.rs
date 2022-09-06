//mod user;
//mod shop;
//use crate::user::{User};
//use std::io;

fn main() {

/*
    let mut user_name = String::new();
    println!("\nEnter your name:");
    std::io::stdin().read_line(&mut user_name).expect("Failed to read line");

    let mut user = User{
        name: user_name,
        health: 10,
        max_health: 10,
        attack: 2,
        deffend: 1,
        coins: 0,
    };

    user.hello();
    user.coins = 10;
    user.hello();
*/
    let _start_menu_list = ["\nОсновное меню:",
            "1. Магазин",
            "2. В бой",
            "3. Стата",
            "4. Выход"];
    println!("\ntest");
    let _result = -1;
    loop {
        let _result = -1;
        println!("Enter num");
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
                        //break;
                    },
                    2 => {
                        println!("two");
                        //break;
                    }
                    _ => println!("Error"),
                }
    }
}
