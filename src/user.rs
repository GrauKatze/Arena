pub struct User  {
    pub name: String,
    pub health: u8,
    pub max_health: u8,
    pub attack: u8,
    pub deffend: u8,
    pub coins: u8
}

impl User {
    pub fn hello(&self){
        println!("Hello, my name is {} and i have {} coins", self.name, self.coins);
    }
    pub fn give_coin(&mut self, coin: u8)->u8{
        if coin <= self.coins{
            self.coins = self.coins - coin;
            coin
        }else{
            0
        }
    }
}
