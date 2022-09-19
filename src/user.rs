pub struct User  {
    name: String,
    max_health: u8,
    health: u8,
    attack: u8,
    deffend: u8,
    coins: u8
}

impl User {
    pub fn init(new_name: String) ->  User{
        User{
            name: new_name,
            max_health: 10,
            health: 10,
            attack: 2,
            deffend: 1,
            coins: 0,
        }
    }

    pub fn _say_hello(&self){
        println!("Hello, my name is {} and i have {} coins", self.name, self.coins);
    }
    pub fn _get_coin(&mut self, coin: u8)->bool{
        if coin <= self.coins{
            self.coins -= coin;
            true
        }else{
            false
        }
    }

    pub fn get_status(&self){
        println!("
        name: {0}
        health {1}/{2}
        attack: {3}
        deffend: {4}
        coins: {5}",
        self.name, self.health, self.max_health, self.attack, self.deffend, self.coins);
    }
    pub fn _attack(&self) -> u8{
        self.attack
    }
    pub fn _deffend(&self) -> u8{
        self.deffend + 1
    }
    pub fn _get_attack(us: &mut User, at: u8){
        Self::_set_health_by_damag(us, at);
    }
    //FIXME: нужна проверка выхода за пределы нуля
    fn _set_health_by_damag(&mut self, damag: u8){
        self.health -= damag + self.deffend;
    }
    pub fn _healing(&mut self, hil: u8){
        if self.health < self.max_health{
            if self.health + hil > self.max_health{
                self.health = self.max_health;
                println!("I am full HP");
            }else{
                let _ = self.health + hil;
            }
        }else{
            println!("I am full HP");
        }
    }

}
