use std::collections::HashMap;
use sodiumoxide::crypto::box_;

use sodiumoxide::crypto::box_::{PublicKey,SecretKey};

use transaction::Transaction;
use transaction::TransactionInput;
use transaction::TransactionOutput;
use blockchain::Blockchain;

pub struct Wallet {
    pub private_key: SecretKey,
    pub public_key: PublicKey,
    UTXOs: HashMap<String, TransactionOutput>,
    blockchain: Blockchain,
}

impl Wallet {
    pub fn new(blockchain: Blockchain) -> Wallet {
        let (public_key, private_key) = box_::gen_keypair();
        
        Wallet {
            private_key,
            public_key,
            UTXOs: HashMap::new(),
            blockchain,
        }
    }

    pub fn get_balance(&mut self) -> f32 {
        let mut total = 0_f32;

        for (id, UTXO) in self.blockchain.UTXOs.iter() {
            if UTXO.is_mine(&self.public_key) {  
                total += UTXO.value;
                self.UTXOs.insert(UTXO.id.clone(), UTXO.clone());
            }
        }

        total 
    }

    pub fn send_funds(&mut self, receiver: PublicKey, value: f32) -> Result<Transaction,String> {
        if self.get_balance() < value {
            return Err(String::from("Insufficient funds"));
        }

        let mut inputs: Vec<TransactionInput> = Vec::new();

        let mut total = 0_f32;
        
        for (ref id, ref UTXO) in self.UTXOs.iter() {
            total += UTXO.value;
            inputs.push(TransactionInput::new(UTXO.id.clone()));
            if total > value {
                break;
            }
        }

        let transaction = Transaction::new(self.public_key, receiver, value, inputs.clone());

        for input in inputs.iter() {
            self.UTXOs.remove(&input.output_id);
        }

        Ok(transaction)
    }
}
