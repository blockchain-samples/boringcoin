extern crate byteorder;

use std::time::UNIX_EPOCH;
use std::time::{Duration, SystemTime};

use sodiumoxide::crypto::box_::{PublicKey,SecretKey,Nonce};
use sodiumoxide::crypto::box_;
use sodiumoxide::crypto::hash;

use byteorder::{ByteOrder, LittleEndian};

use blockchain::Blockchain;

#[derive(Clone)]
pub struct Transaction {
    pub id: String,
    pub sender: PublicKey,
    pub receiver: PublicKey,
    pub value: f32,
    pub signature: Vec<u8>,
    pub nonce: Nonce,
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
    pub timestamp: u64,
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
        let sender_string = String::from_utf8(self.sender.0.to_vec()).unwrap();
        let receiver_string = String::from_utf8(self.receiver.0.to_vec()).unwrap();
        
        let mut sender_vec = sender.0.to_vec();
        let mut receiver_vec = receiver.0.to_vec();

        let mut buf = [0;4];
        LittleEndian::write_f32(&mut buf, self.value.clone());
        let value_vec = buf.to_vec();

        let mut buf2 = [0;4];
        LittleEndian::write_f32(&mut buf, self.timestamp.clone());
        let timestamp_vec = buf2.to_vec();

        let mut input_vec = Vec::new();
        input_vec.append(&mut sender_vec);
        input_vec.append(&mut receiver_vec);
        input_vec.append(&mut value_vec);
        input_vec.append(&mut timestamp_vec);

        let hashed_input = hash::hash(&input_vec);
        String::from_utf8(hashed_input.0.to_vec()).unwrap()
    }

    fn get_timestamp() -> u64 {
        let now = SystemTime::now();
        let millis = now.duration_since(UNIX_EPOCH).expect("time went backwards").as_secs();
        millis
    }

    pub fn generate_signature(&mut self, sender_priv_key: &SecretKey) {
        self.nonce = box_::gen_nonce();
        self.signature = box_::seal(&self.get_sig_info().into_bytes(), &self.nonce, &self.receiver, sender_priv_key);
    }

    pub fn verify_signature(&self, receiver_priv_key: &SecretKey) -> bool {
        let unencrypted = box_::open(&self.signature, &self.nonce, &self.sender, receiver_priv_key).unwrap(); 
        self.get_sig_info() == String::from_utf8(unencrypted).unwrap()
    }

    fn get_sig_info(&self) -> String {
        let sender_string = String::from_utf8(self.sender.0.to_vec()).unwrap();
        let receiver_string = String::from_utf8(self.receiver.0.to_vec()).unwrap();
        format!("{}{}{}{}", sender_string, receiver_string, self.value, self.timestamp)
    }

    pub fn process_transaction(&mut self, mut blockchain: Blockchain, receiver_priv_key: &SecretKey) -> bool {
        if !self.verify_signature(receiver_priv_key) {
            return false;
        }

        for input in self.inputs.iter_mut() {
            let cloned = blockchain.UTXOs.get(&input.output_id).unwrap().clone(); 
            input.UTXO = cloned;
        }

        if self.get_inputs_val() < blockchain.minimum_transaction {
            return false;
        }

        let left_over = self.get_inputs_val() - self.value;
        let id = self.calc_hash();
        self.outputs.push(TransactionOutput::new(self.receiver.clone(), self.value.clone(), self.id.clone()));
        self.outputs.push(TransactionOutput::new(self.sender.clone(), left_over, self.id.clone()));
    
        for output in self.outputs.iter() {
            let clone = output.clone();
            blockchain.UTXOs.insert(clone.id.clone(), clone);
        }

        for input in self.inputs.iter() {
            blockchain.UTXOs.remove(&input.UTXO.id);
        }
    
        true
    }

    pub fn get_inputs_val(&self) -> f32 {
        let mut total = 0_f32;
        
        for input in self.inputs.iter() {
            total += input.UTXO.value;
        }

        total 
    }

    pub fn get_outputs_val(&self) -> f32 {
        let mut total = 0_f32;

        for output in self.outputs.iter() {
            total += output.value;
        }

        total 
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct TransactionOutput {
    pub id: String,
    pub receiver: PublicKey,
    pub value: f32,
    pub transaction_id: String,
}

impl TransactionOutput {
    pub fn new(receiver: PublicKey, value: f32, transaction_id: String) -> TransactionOutput {
        let mut transaction_output = TransactionOutput {
            id: String::new(),
            receiver,
            value,
            transaction_id,
        };

        let hashed_receiver = hash::hash(&receiver.0.to_vec());
        transaction_output.id = String::from_utf8(hashed_receiver.0.to_vec()).unwrap();
        
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

















