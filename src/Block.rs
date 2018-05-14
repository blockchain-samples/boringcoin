use std::time::{SystemTime, UNIX_EPOCH};
use std::iter;

use sodiumoxide::crypto::hash;
use sodiumoxide::crypto::box_::SecretKey;

use transaction::Transaction;
use blockchain::Blockchain;

use byteorder::{ByteOrder, LittleEndian}; 

#[derive(Clone)]
pub struct Block {
    pub hash: Vec<u8>,
    pub prev_hash: Vec<u8>,
    merkle_root: String,
    pub transactions: Vec<Transaction>,
    timestamp: u64,
    nonce: i32,
}

impl Block {
    pub fn new(prev_hash: Vec<u8>) -> Block {
        let mut block = Block {
            hash: Vec::new(),
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

    pub fn calc_hash(&self) -> Vec<u8> {
        let mut buf = [0;8];
        LittleEndian::write_u64(&mut buf, self.timestamp.clone());
        let mut timestamp_vec = buf.to_vec().clone();

        let mut buf2 = [0;4];
        LittleEndian::write_i32(&mut buf2, self.nonce.clone());
        let mut nonce_vec = buf2.to_vec();

        let mut input_vec: Vec<u8> = Vec::new();
        input_vec.append(&mut timestamp_vec);
        input_vec.append(&mut nonce_vec);
        input_vec.append(&mut self.prev_hash.clone());
        input_vec.append(&mut self.merkle_root.as_bytes().to_vec().clone());
        
        let hashed_input = hash::hash(&input_vec);
        hashed_input.0.to_vec()
    }

    pub fn mine(&mut self, difficulty: usize) {
        self.merkle_root = Self::get_merkle_root(self.transactions.clone());
        let target: Vec<u8> = iter::repeat(0_u8).take(difficulty).collect(); 
        let mut hash_prefix: Vec<u8> = self.hash.clone().into_iter().take(difficulty).collect::<Vec<u8>>();
        println!("Starting to mine: {:?} {:?}", hash_prefix, target);
        while hash_prefix != target {
            println!("comparing {:?} to {:?}", hash_prefix, target);
            self.nonce += 1;
            self.hash = self.calc_hash();
            hash_prefix = self.hash.clone().into_iter().take(difficulty).collect::<Vec<u8>>();
        }
        println!("FINAL HASH: {:?}", self.hash); 
    }

    pub fn add_transaction(&mut self, mut transaction: Transaction, blockchain: Blockchain, receiver_priv_key: &SecretKey) -> bool {
        if self.prev_hash != vec![0_u8] {
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
