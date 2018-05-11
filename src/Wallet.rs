use std::collections::HashMap;
use sodiumoxide::crypto::box_;

use sodiumoxide::crypto::box_::PublicKey;

use transaction::{Transaction,TransactionInput,TransactionOutput};
use blockchain::Blockchain;

struct Wallet {
    private_key: PublicKey,
    public_key: PublicKey,
    UTXOs: HashMap<String, TransactionOutput>,
    blockchain: Blockchain,
}

impl Wallet {
    pub fn new(blockchain: Blockchain) -> Wallet {
        let (private_key, public_key) = box_::gen_keypair();
        
        Wallet {
            private_key,
            public_key,
            UTXOs: HashMap::new(),
            blockchain,
        }
    }

    pub fn get_balance(&self) -> f32 {
        let total = 0_f32;

        for (id, UTXO) in self.blockchain.UTXOs {
            if UTXO.is_mine(&self.public_key) {
                self.UTXOs.insert(UTXO.id, UTXO);
                total += UTXO.value;
            }
        }

        total 
    }

    pub fn send_funds(&mut self, receiver: String, value: f32) -> Result<Transaction,String> {
        if self.get_balance() < value {
            return Err(String::from("Insufficient funds"));
        }

        let inputs: Vec<TransactionInput> = Vec::new();

        let total = 0_f32;
        
        for (id, UTXO) in self.UTXOs {
            total += UTXO.value;
            inputs.push(TransactionInput::new(UTXO.id));
            if total > value {
                break;
            }
        }

        let transaction = transaction::Transcation::new(self.public_key, receiver, value, inputs);

        for input in inputs {
            self.UTXOs.remove(&input.output_id);
        }

        Ok(transaction)
    }
}
