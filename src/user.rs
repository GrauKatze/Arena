pub struct User  {
    name: String,
    health: u8,
    max_health: u8,
    attack: u8,
    deffend: u8,
    coins: u8
}

impl User {
    pub fn say_hello(&self){
        println!("Hello, my name is {} and i have {} coins", self.name, self.coins);
    }
    pub fn get_coin(&mut self, coin: u8)->bool{
        if coin <= self.coins{
            self.coins = self.coins - coin;
            true
        }else{
            false
        }
    }
    /*
    pub fn get_name(&self) -> String{
       self.name
    }*/
    pub fn attack(&self) -> u8{
        self.attack
    }
    pub fn deffend(&self) -> u8{
        self.deffend
    }
    pub fn healing(&mut self, hil: u8){
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
