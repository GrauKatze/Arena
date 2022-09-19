use crate::user::User;

use std::io::Write;
pub enum ShopList {
// name (name, price, effect)
    Hill(String, u8,u8),    //hill player
    Attack(String, u8,u8),  //add attack for player
    Deffend(String, u8,u8), //add def fo player
    Health(String, u8,u8),  //add max hp fo player
}

impl ShopList {

    pub fn shops_menu(us: &mut User){

        let hill = ShopList::Hill("Hill".to_string(),10,5);
        let attack = ShopList::Attack("attack".to_string(),10,5);
        let deffend = ShopList::Deffend("def".to_string(),10,5);
        let health = ShopList::Health("health".to_string(),10,5);

        let menu_shop_list =["\nShop list",
        "1. hill",
        "2. attack",
        "3. deffend",
        "4. health"];
        for el in menu_shop_list{
            println!("{}",el);
        }

        loop{
            print!("Enter num [SHOP] >> ");
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
                    Self::take_item(us, hill);
                    break;
                },
                2 => {
                    println!("two");
                    Self::take_item(us, attack);
                    break;
                },
                3 => {
                    println!("three");
                    Self::take_item(us, deffend);
                    break;
                },
                4 =>{
                    println!("four");
                    Self::take_item(us, health);
                    break;
                },
                0 => {
                    println!("Exit" );
                    break;
                }
                _ => println!("Error"),
            }
        }

    }
    fn take_item(_us: &mut User, sl: ShopList){
        match sl {
            ShopList::Hill(name, _price, _effect) => {
                println!("{}", name);
            }
            ShopList::Attack(name, _price, _effect)=>{
                println!("{}", name);
            }
            ShopList::Deffend(name, _price, _effect)=>{
                println!("{}", name);
            }
            ShopList::Health(name, _price, _effect)=>{
                println!("{}", name);
            }
        }
    }
}
