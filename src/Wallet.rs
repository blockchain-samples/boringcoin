struct Wallet {
    private_key: String,
    public_key: String,
    UTXOs: // TODO
}

impl Wallet {
    pub fn new() -> Wallet {
        (private_key, public_key) = Self::generate_keys();
        
        Wallet {
            private_key,
            public_key,
        }
    }

    pub fn get_balance(&self) -> f32 {
        let total = 0_f32;

        for UTXO in somehow_get_blockchain().UTXOs {
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
