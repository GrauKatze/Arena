pub mod user_mod {
//TODO: добавить поля для персонажа
    pub struct User  {
            pub name: String,
            pub health: u8,
        }
    pub fn hello(){
        println!("Hello");
    }
    //TODO: добавить действия для атака, защиты и обмену с магазином
    pub fn initial() -> User {
        let mut user_name = String::new();
        println!("Enter your name:");
        std::io::stdin().read_line(&mut user_name);
        println!("You enter {user_name}");
        let _user = User{
            name: user_name,
            health: 10
        };
        return _user;
    }

}
