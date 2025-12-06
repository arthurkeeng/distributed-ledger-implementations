use std::time::Instant;

use sha2::{Sha256, Digest};
pub mod merkle;
pub mod wallet;

#[derive(Debug , Clone)]
pub struct Transaction{
    pub from : String , 
    pub to : String, 
    pub amount : u64
}
#[derive(Debug)]

pub struct Block{
    index : u64 , 
    nonce : u64 , 
    transactions : Vec<Transaction>, 
    previous_hash : String , 
    hash : String , 
    timestamp : u128
}
impl Block {
    fn new(index : u64 , transactions : Vec<Transaction> , previous_hash : String)-> Self{
        let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis();
        let hash = String::new();

        let mut block = Block { index, nonce: 0, transactions, previous_hash , hash , timestamp};
        block.hash = block.calculate_hash();
        block
        
    }
    fn calculate_hash(&self)-> String {
        let hash = format!("{}{}{:?}{}{}" , 
            self.index,
            self.timestamp,
            self.transactions,
            self.previous_hash,
            self.nonce    
        );
        let mut hasher = Sha256::new();
        hasher.update(hash);
        format!("{:x}", hasher.finalize())

    }

     pub fn mine(&mut self, difficulty: usize) {
        let prefix = "0".repeat(difficulty);
        while !self.hash.starts_with(&prefix) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
        println!("Block mined: {}", self.hash);
    }
}
#[derive(Debug)]
pub struct Blockchain{
    chain : Vec<Block>, 
    difficulty : usize
}

impl Blockchain{
    fn new(difficulty : usize) -> Self {
        let mut genesis_block = Block::new(0, vec![], "0".to_string());
        genesis_block.mine(difficulty);
        Self { chain: vec![genesis_block] , difficulty }
    }
    fn add_block(&mut self , transactions : Vec<Transaction> ){
        let previous_block = self.chain.last().unwrap();
        let mut new_block = Block::new(previous_block.index + 1, transactions, previous_block.hash.clone());
        
        new_block.mine(self.difficulty);
        self.chain.push(new_block);


    }
    fn is_valid(&self) -> bool{
        for i in 1..self.chain.len(){
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if current.previous_hash != previous.hash {
                return false;
            }
            if current.hash != current.calculate_hash(){
                return  false;
            }
            
        }
        true
    }

    fn latest_block(&self) -> &Block{
        self.chain.last().unwrap()
    }
}

// src/main.rs

fn main() {
    println!("=== Starting Blockchain Learning ===");
    
    // Create a new blockchain with mining difficulty
    let mut blockchain = Blockchain::new(2);  // 2 leading zeros
    
    // Create some transactions
    let transactions = vec![
        Transaction {
            from: "Alice".to_string(),
            to: "Bob".to_string(),
            amount: 100,
        },
        Transaction {
            from: "Bob".to_string(),
            to: "Charlie".to_string(),
            amount: 50,
        },
    ];
    
    let mut hasher = Sha256::new();
    let start = Instant::now();
    for i in 0..100_000 {
        hasher.update(format!("{}" , i));
    }
    let duration = start.elapsed();
    println!("   100,000 hashes in: {:?}", duration);
    println!("   ~{:.0} hashes/second", 100_000.0 / duration.as_secs_f64());
    println!("\nMining block 1...");
    blockchain.add_block(transactions);
    
    println!("\nBlockchain status:");
    println!("Number of blocks: {}", blockchain.chain.len());
    println!("Latest block hash: {}", blockchain.latest_block().hash);
    println!("Is chain valid? {}", blockchain.is_valid());
    
    // Demonstrate chain structure
    println!("\nBlockchain structure:");
    for (i, block) in blockchain.chain.iter().enumerate() {
        println!("\nBlock {}:", i);
        println!("  Hash: {}", block.hash);
        println!("  Previous: {}", block.previous_hash);
        println!("  Nonce: {}", block.nonce);
        println!("  Transactions: {}", block.transactions.len());
    }
    
    // Tamper test
    println!("\n=== Tampering Test ===");
    let mut tampered_blockchain = blockchain;
    if let Some(block) = tampered_blockchain.chain.get_mut(1) {
        block.transactions[0].amount = 1000;
        println!("Changed transaction amount from 100 to 1000");
        println!("Is chain still valid? {}", tampered_blockchain.is_valid());
    }
}