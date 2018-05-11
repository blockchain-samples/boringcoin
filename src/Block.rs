extern crate sodiumoxide;

use std::time::{SystemTime, UNIX_EPOCH};

use sodiumoxide::crypto::hash;
use sodiumoxide::crypto::box_::SecretKey;

use transaction::Transaction;
use blockchain::Blockchain;

#[derive(Clone)]
pub struct Block {
    pub hash: String,
    pub prev_hash: String,
    merkle_root: String,
    pub transactions: Vec<Transaction>,
    timestamp: u64,
    nonce: i32,
}

impl Block {
    pub fn new(prev_hash: String) -> Block {
        let mut block = Block {
            hash: String::new(),
            prev_hash,
            merkle_root: String::new(),
            transactions: Vec::new(),
            timestamp: Self::get_timestamp(),
            nonce: 0,
        };

        block.hash = block.calc_hash();

        block
    }

    fn get_timestamp() -> u64 {
        let now = SystemTime::now();
        let millis = now.duration_since(UNIX_EPOCH).expect("time went backwards").as_secs();
        millis
    }

    pub fn calc_hash(&self) -> String {
        let input = format!("{}{}{}{}", self.prev_hash, self.timestamp.to_string(), self.nonce, self.merkle_root);
        let hashed_input = hash::hash(&input.into_bytes());
        String::from_utf8(hashed_input.0.to_vec()).unwrap()
    }

    pub fn mine(&mut self, difficulty: i32) {
        self.merkle_root = Self::get_merkle_root(self.transactions.clone());
        let target = "0".repeat(difficulty as usize);
        let hash_prefix: String = self.hash.chars().take(difficulty as usize).collect(); 
        while hash_prefix != target {
            self.nonce += 1;
            self.hash = self.calc_hash();
        }
    }

    pub fn add_transaction(&mut self, mut transaction: Transaction, blockchain: Blockchain, receiver_priv_key: &SecretKey) -> bool {
        if self.prev_hash != String::from("0") {
            if transaction.process_transaction(blockchain, receiver_priv_key) != true {
                println!("Transaction failed to process. Discarding.");
                return false;
            }
        }

        self.transactions.push(transaction);
        true
    }
    
    pub fn get_merkle_root(transactions: Vec<Transaction>) -> String {
        let mut count = transactions.len();
        let mut previous_tree_layer: Vec<String> = Vec::new();
    
        for transaction in transactions {
            previous_tree_layer.push(transaction.id);
        }

        let mut tree_layer: Vec<String> = previous_tree_layer.clone();
       
        let mut to_return = String::new();
        while count > 1 {
            tree_layer = Vec::new();

            let prev_len = previous_tree_layer.len();
            
            for i in 0..prev_len {
                let input = format!("{}{}", previous_tree_layer[i-1].clone(), previous_tree_layer[i].clone());
                let input_hash = hash::hash(&input.into_bytes());
                let hashed_string = String::from_utf8(input_hash.0.to_vec()).unwrap();
                tree_layer.push(hashed_string);
            }

            count = tree_layer.len();
            to_return = tree_layer[0].clone();
            previous_tree_layer = tree_layer;
        }

        if count == 0 {
            return to_return;
        } else {
            return String::from("");
        }
    }
}
