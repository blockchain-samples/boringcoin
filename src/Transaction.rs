pub struct Transaction {
    id: String,
    sender: String,
    receiver: String,
    value: f32,
    signature: String,
    inputs: Vec<TransactionInput>,
    outputs: Vec<TransactionOutput>,
    timestamp: u32,
}

impl Transaction {
    pub fn new(sender: String, receiver: String, value: f32, input: Vec<TransactionInput>) -> Transaction {
        let transaction = Transaction {
            id: String::new(),
            sender,
            receiver,
            value,
            input,
            output: Vec::new(),
            timestamp: Self::get_timestamp(),
        };
    }

    pub fn calc_hash(&self) -> String { 
        let input = format!("{}{}{}{}", self.sender, self.receiver, self.value, self.timestamp.to_string());
        let mut sha = Sha256::new();
        sha.input_str(input.as_str());
        sha.result_str();
    }

    fn get_timestamp() -> u32 {
        let now = SystemTime::now();
        let millis = now.duration_since(UNIX_EPOCH).expect("time went backwards").as_secs();
    }

    pub fn generate_signature() -> String {
        
    }

    pub fn verify_signature() -> bool {

    }

    pub fn process_transcation(&self) -> bool {
        if !self.verify_signature() {
            return false;
        }

        for input in self.inputs {
            input.UTXO = somehow_get_blockchain().UTXOs.get(input.output_id);
        }

        if get_inputs_val() < somehow_get_blockchain().minimum_transaction {
            return false;
        }

        let left_over = get_inputs_val() - self.value;
        let id = self.calculate_hash();
        self.outputs.push(TransactionOutput::new(self.receiver, self.value, self.id));
        self.outputs.push(TransactionOutput::new(self.sender, left_over, self.id));
    
        for output in self.outputs {
            somehow_get_blockchain().UTXOs.insert(output.id, output);
        }

        for input in self.inputs {
            if input.UTXO == null {
                continue;
            }

            somehow_get_blockchain().UTXOs.remove(input.UTXO.id);
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



















