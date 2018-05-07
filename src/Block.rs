extern crate crypto;

use crypto::digest::Digest;
use crypto::sha2::Sha256;

use std::time::{SystemTime, UNIX_EPOCH};

pub struct Block {
    hash: String,
    prev_hash: String,
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
        let input = format!("{}{}{}{}", self.prev_hash, self.data, self.timestamp.to_string(), self.nonce);
        let mut sha = Sha256::new();
        sha.input_str(input.as_str());
        sha.result_str();
    }

    pub fn mine_block(&mut self, difficulty: i32) {
        target = "0".repeat(difficulty);
        while (hash[0..difficulty] != target) {
            self.nonce += 1;
            self.hash = self.calc_hash();
        }
    }
}
