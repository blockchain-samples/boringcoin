extern crate sodiumoxide;

use sodiumoxide::crypto::box_::curve25519xsalsa20poly1305::{ PublicKey, PrivateKey };
use sodiumoxide::crypto::box_;
use sodiumoxide::crypto::hash;

pub struct Transaction {
    id: String,
    sender: PublicKey,
    receiver: PublicKey,
    value: f32,
    signature: Vec<u8>,
    nonce: Nonce,
    inputs: Vec<TransactionInput>,
    outputs: Vec<TransactionOutput>,
    timestamp: u32,
}

impl Transaction {
    pub fn new(sender: PublicKey, receiver: PublicKey, value: f32, inputs: Vec<TransactionInput>) -> Transaction {
        let transaction = Transaction {
            id: String::new(),
            sender,
            receiver,
            value,
            signature: Vec::new(),
            inputs,
            outputs: Vec::new(),
            timestamp: Self::get_timestamp(),
        };
    }

    pub fn calc_hash(&self) -> String { 
        let input = format!("{}{}{}{}", self.sender, self.receiver, self.value, self.timestamp.to_string());
        hash::hash(input);
    }
    

    fn get_timestamp() -> u32 {
        let now = SystemTime::now();
        let millis = now.duration_since(UNIX_EPOCH).expect("time went backwards").as_secs();
    }

    pub fn generate_signature(&self, sender_priv_key: PrivateKey) {
        self.nonce = box_::gen_nonce();
        self.signature = box_::seal(get_sig_info(), &self.nonce, &self.receiver, sender_priv_key);
    }

    pub fn verify_signature(&self, receiver_priv_key: PrivateKey) -> bool {
        let unencrypted = box_::open(&self.signature, &self.nonce, &self.sender, receiver_priv_key).unwrap(); // TODO -> REMOVE UNWRAP 
        get_sig_info() == String::from_utf8(unencrypted)
    }

    fn get_sig_info(&self) -> String {
         format!("{}{}{}{}", self.sender, self.receiver, self.value, self.timestamp)
    }

    pub fn process_transcation(&self, blockchain: &Blockchain) -> bool {
        if !self.verify_signature() {
            return false;
        }

        for input in self.inputs {
            input.UTXO = blockchain.UTXOs.get(input.output_id);
        }

        if get_inputs_val() < blockchain.minimum_transaction {
            return false;
        }

        let left_over = get_inputs_val() - self.value;
        let id = self.calculate_hash();
        self.outputs.push(TransactionOutput::new(self.receiver, self.value, self.id));
        self.outputs.push(TransactionOutput::new(self.sender, left_over, self.id));
    
        for output in self.outputs {
            blockchain.UTXOs.insert(output.id, output);
        }

        for input in self.inputs {
            if input.UTXO == null {
                continue;
            }

            blockchain.UTXOs.remove(input.UTXO.id);
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

    pub fn get_outputs_val(&self) -> {
        let total = 0_f32;

        for output in self.outputs {
            total += output.value;
        }

        total 
    }
}

pub struct TransactionInput {
    output_id: String,
    UTXO: TransactionOutput,
}

impl TransactionInput {
    pub fn new(output_id: String) -> TransactionInput {
        TransactionInput {
            output_id,
            UTXO: TODO,
        }
    }
}

pub struct TransactionOutput {
    id: String,
    receiver: String,
    value: f32,
    transaction_id: String,
}

impl TransactionOutput {
    pub fn new(receiver: String, value: f32, transaction_id: String) -> TransactionOutput {
        let transaction_output = TransactionOutput {
            receiver,
            value,
            transaction_id,
        };

        transaction_output.id = get_string_from_key();
        
        transaction_output 
    }

    pub fn is_mine(&self, public_key: String) -> bool {
        public_key == self.receiver
    }
}



















