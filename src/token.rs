use std::collections::HashMap;



trait Token {
    fn total_supply(&self) -> u64;
    fn balance (&self , account : &str) -> u64;
    fn transfer(&self , to : &str , from : &str , amount : u64) -> Result<(), String>;
    fn approve(&mut self , owner : &str , spender : &str , amount : u64) -> Result<() , String>;
}

pub struct EVMToken{
    pub storage : HashMap<String , u64>, 
    pub total_supply : u64
}

impl EVMToken{
    pub fn new(initial_supply : u64 , owner : &str) -> Self{
        let mut storage = HashMap::new();

        storage.insert(format!("balance_{}", owner), initial_supply);
        storage.insert("total_supply".to_string(), initial_supply);

        EVMToken { storage, total_supply: initial_supply }
    }
    fn sload(&self , key: &str) -> u64{
        *self.storage.get(key).unwrap_or(&0)
    }
    fn sstore(&mut self, key : String , value : u64) {
        self.storage.insert(key, value);
    }
}

// impl Token for EVMToken{
//     fn total_supply(&self) -> u64 {
//         self.sload("total_supply")
//     }

//     fn balance (&self , account : &str) -> u64 {
//         self.sload(&format!("balance_{}" , account))
//     }
// }