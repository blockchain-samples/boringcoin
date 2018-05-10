extern crate crypto;

use crypto::digest::Digest;
use crypto::sha2::Sha256;

use std::time::{SystemTime, UNIX_EPOCH};

pub struct Block {
    hash: String,
    prev_hash: String,
    merkle_root: String,
    transactions: Vec<Transaction>,
    timestamp: u32,
    data: String,
    nonce: i32,
}

impl Block {
    pub fn new(prev_hash: String, data: String) -> Block {
        let block = Block {
            hash: String::new(),
            prev_hash,
            data,
            timestamp: Self::get_now(),
        };

        block.hash = block.calc_hash();

        block
    }

    fn get_timestamp() -> u32 {
        let now = SystemTime::now();
        let millis = now.duration_since(UNIX_EPOCH).expect("time went backwards").as_secs();
    }

    pub fn calc_hash(&self) -> String {
        let input = format!("{}{}{}{}{}", self.prev_hash, self.data, self.timestamp.to_string(), self.nonce, self.merkle_root);
        let mut sha = Sha256::new();
        sha.input_str(input.as_str());
        sha.result_str();
    }

    pub fn mine(&mut self, difficulty: i32) {
        merkle_root = get_merkle_root(self.transactions);
        target = "0".repeat(difficulty);
        while (hash[0..difficulty] != target) {
            self.nonce += 1;
            self.hash = self.calc_hash();
        }
    }

    pub fn add_transaction(&self, transaction: Transaction) -> bool {
        if self.prev_hash != Sring::from("0") {
            if transaction.process_transaction() != true {
                println!("Transaction failed to process. Discarding.");
                return false;
            }
        }

        self.transactions.push(transaction);
        true
    }
    
    pub fn get_merkle_root(transactions: Vec<Transaction>) -> String {
        let mut count = transactions.len();
        let mut previous_tree_layer: Vec<String> = String::new();
    
        for transaction in transactions {
            previous_tree_layer.push(transaction.id);
        }

        let mut tree_layer: Vec<String> = previous_tree_layer;
        
        while count > 1 {
            tree_layer = Vec::new();

            let prev_len = previous_tree_layer.len();
            
            for i in 0..prev_len {
                tree_layer.push(apply_sha_256(format!("{}{}",previous_tree_layer[i-1],previous_tree_layer[i])));
            }

            count = tree_layer.len();
            previous_tree_layer = tree_layer;
        }

        if tree_layer.len() == 0 {
            return tree_layer[0];
        } else {
            return String::from("");
        }


    }
}
