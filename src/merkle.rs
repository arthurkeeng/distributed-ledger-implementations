
use sha2::{Sha256, Digest};

#[derive(Debug)]
pub struct MerkleTree{
    pub root : String , 
    leaves : Vec<String>, 
    levels : Vec<Vec<String>>
}

impl MerkleTree {
    pub fn new(data : Vec<&str>) ->Self{
        if data.is_empty(){
            return MerkleTree { root: String::new(), leaves: vec![], levels: vec![] };
        }

        let leaves: Vec<String> = data.iter().map(|s| Self::hash(s))
        .collect();

    let mut levels = Vec::new();
    levels.push(leaves.clone());

    let mut current_level = leaves.clone();

    while current_level.len() > 1 {
        let mut next_level = Vec::new();
        for chunk in current_level.chunks(2){
            if chunk.len() == 2 {
                let combined = format!("{}{}", chunk[0] , chunk[1]);
                next_level.push(Self::hash(&combined));

            }
            else{
                let combined = format!("{}{}", chunk[0] , chunk[0]);
                next_level.push(Self::hash(&combined));
            }
        }
        levels.push(next_level.clone());
        current_level = next_level;
    }
    MerkleTree { root: current_level[0].clone(), leaves, levels }

    }
    pub fn hash(data : &str) -> String{
        let mut hasher = Sha256::new();

        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())

    }

    pub fn verify (&self , data : &str , proof : Vec<(String , bool)>) -> bool {
        let mut current_hash = Self::hash(data);
       for  (sibling_hash , is_left) in proof{
        current_hash = if is_left{
            Self::hash(&format!("{}{}" , sibling_hash , current_hash))
        }
        else{
            Self::hash(&format!("{}{}" , current_hash,sibling_hash ))

        }
       }
       current_hash == self.root
    }
    pub fn get_proof(&self , index : usize) -> Option<Vec<(String , bool)>> {
        if index >= self.leaves.len(){
            return None;
        }
        let mut proof = Vec::new();
        let mut current_index = index;

        for level in 0..self.levels.len() - 1 {
            let level_nodes = &self.levels[level];

            if level_nodes.len() <= 1 {
                break;
            }
            let is_left = current_index % 2 == 0 ;

            let sibling_index = if is_left{
                current_index + 1 
            }else {
                current_index - 1
            };
            if sibling_index < level_nodes.len() {
                proof.push((level_nodes[sibling_index].clone(), !is_left));
            }
            current_index /= 2;
        }

        Some(proof)
    }

      pub fn print_tree(&self) {
        println!("\n=== Merkle Tree Structure ===");
        for (i, level) in self.levels.iter().enumerate().rev() {
            println!("Level {} ({} nodes):", i, level.len());
            for (j, hash) in level.iter().enumerate() {
                println!("  [{}]: {}...", j, &hash[0..8]);
            }
        }
        println!("Root: {}", self.root);
    }
}

