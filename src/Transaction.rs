use std::time::UNIX_EPOCH;

use sodiumoxide::crypto::box_::PublicKey;
use sodiumoxide::crypto::box_;
use sodiumoxide::crypto::hash;
use sodiumoxide::crypto::box_::Nonce;

use blockchain::Blockchain;

pub struct Transaction {
    pub id: String,
    pub sender: PublicKey,
    pub receiver: PublicKey,
    pub value: f32,
    pub signature: Vec<u8>,
    pub nonce: Nonce,
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
    pub timestamp: u32,
}

impl Transaction {
    pub fn new(sender: PublicKey, receiver: PublicKey, value: f32, inputs: Vec<TransactionInput>) -> Transaction {
        Transaction {
            id: String::new(),
            sender,
            receiver,
            value,
            signature: Vec::new(),
            nonce: Nonce([0_u8;24]),
            inputs,
            outputs: Vec::new(),
            timestamp: Self::get_timestamp(),
        }
    }

    pub fn calc_hash(&self) -> String { 
        let input = format!("{}{}{}{}", self.sender, self.receiver, self.value, self.timestamp.to_string());
        String::from_utf8(hash::hash(input).0.to_vec()).unwrap()
    }

    fn get_timestamp() -> u32 {
        let now = SystemTime::now();
        let millis = now.duration_since(UNIX_EPOCH).expect("time went backwards").as_secs();
        millis
    }

    pub fn generate_signature(&mut self, sender_priv_key: &PrivateKey) {
        self.nonce = box_::gen_nonce();
        self.signature = box_::seal(self.get_sig_info(), &self.nonce, &self.receiver, sender_priv_key);
    }

    pub fn verify_signature(&self, receiver_priv_key: &PrivateKey) -> bool {
        let unencrypted = box_::open(&self.signature, &self.nonce, &self.sender, receiver_priv_key).unwrap(); // TODO -> REMOVE UNWRAP 
        self.get_sig_info() == String::from_utf8(unencrypted).unwrap()
    }

    fn get_sig_info(&self) -> String {
         format!("{}{}{}{}", self.sender, self.receiver, self.value, self.timestamp)
    }

    pub fn process_transaction(&mut self, blockchain: Blockchain) -> bool {
        if !self.verify_signature() {
            return false;
        }

        for input in self.inputs {
            input.UTXO = blockchain.UTXOs.get(&input.output_id).unwrap();
        }

        if self.get_inputs_val() < blockchain.minimum_transaction {
            return false;
        }

        let left_over = self.get_inputs_val() - self.value;
        let id = self.calc_hash();
        self.outputs.push(TransactionOutput::new(self.receiver, self.value, self.id));
        self.outputs.push(TransactionOutput::new(self.sender, left_over, self.id));
    
        for output in self.outputs {
            blockchain.UTXOs.insert(output.id, output);
        }

        for input in self.inputs {
            blockchain.UTXOs.remove(&input.UTXO.id);
        }
    
        true
    }

    pub fn get_inputs_val(&self) -> f32 {
        let total = 0_f32;
        
        for input in self.inputs {
            total += input.UTXO.value;
        }

        total 
    }

    pub fn get_outputs_val(&self) -> f32 {
        let total = 0_f32;

        for output in self.outputs {
            total += output.value;
        }

        total 
    }
}

pub struct TransactionInput {
    pub output_id: String,
    pub UTXO: TransactionOutput,
}

impl TransactionInput {
    pub fn new(output_id: String) -> TransactionInput {
        TransactionInput {
            output_id,
            UTXO: TransactionOutput::dud(),
        }
    }
}

pub struct TransactionOutput {
    pub id: String,
    pub receiver: PublicKey,
    pub value: f32,
    pub transaction_id: String,
}

impl TransactionOutput {
    pub fn new(receiver: PublicKey, value: f32, transaction_id: String) -> TransactionOutput {
        let transaction_output = TransactionOutput {
            id: String::new(),
            receiver,
            value,
            transaction_id,
        };

        let key_as_str = String::from_utf8(receiver.0.to_vec()).unwrap();
        transaction_output.id = String::from_utf8(hash::hash(key_as_str).0.to_vec()).unwrap();
        
        transaction_output 
    }

    pub fn dud() -> TransactionOutput {
        TransactionOutput {
            id: String::new(),
            receiver: PublicKey([0_u8;32]),
            value: 0_f32,
            transaction_id: String::new(),
        }
    }

    pub fn is_mine(&self, public_key: &PublicKey) -> bool {
        public_key == &self.receiver
    }
}



















