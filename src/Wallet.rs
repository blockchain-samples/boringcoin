use std::collections::HashMap;
use sodiumoxide::crypto::box_;

struct Wallet {
    private_key: String,
    public_key: String,
    UTXOs: HashMap<String, TransactionOutput>,
    blockchain: &Blockchain,
}

impl Wallet {
    pub fn new(blockchain: &Blockchain) -> Wallet {
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

        for UTXO in self.blockchain.UTXOs {
            if UTXO.is_mine() {
                self.UTXOs.insert(UTXO.id, UTXO);
                total += UTXO.value;
            }
        }

        total 
    }

    pub fn send_funds(&self, receiver: String, value: f32) -> Result<Transaction> {
        if self.get_balance() < value {
            return Err("Insufficient funds");
        }

        let inputs: Vec<TransactionInput> = Vec::new();

        let total = 0_f32;
        
        for UTXO in self.UTXOs {
            total += UTXO.value;
            inputs.push(TransactionInput::new(UTXO.id));
            if total > value {
                break;
            }
        }

        let transaction = Transcation::new(self.public_key, receiver, value, inputs);

        for input in inputs {
            UTXOs.delete(input.output_id);
        }

        transaction
    }
}
