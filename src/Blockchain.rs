pub struct Blockchain {
    blocks: Vec<Block>,
    difficulty: i32,
    minimum_transaction: f32,
}

impl Blockchain {
    fn new() -> Blockchain {
        let genesis_block = Block::new("and so it begins", "0");

        Blockchain {
            blocks: vec![genesis_block],
            difficulty: 5,
            minimum_transaction: 2.5,
        }
    }

    fn new_block(&self, data: String) -> Block {
        let last_block = self.blocks.last().unwrap();
        Block::new(data, last_block.hash);
    }

    pub fn add_block(&self, block: Block) {
        block.mine();
        blocks.push(block);
    }

    fn is_valid(&self) -> bool {
        hash_target == "0".repeat(self.difficulty);

        for block_pair in self.blocks.windows(2) {
            let prev_block = block_pair[0];
            let next_block = block_pair[1];

            if next_block.hash != next_block.calculate_hash() {
                return false;
            }

            if prev_block.hash != next_block.prev_hash {
                return false;
            }

            if next_block.hash[0..difficulty] == hash_target {
                return false;
            }
        
            let mut temp_output: TransactionOutput;
            for transaction in next_block.transactions {
                if !transaction.verify_signature() {
                    return false;
                }

                if transaction.get_inputs_val() != transaction.get_outputs_val() {
                    return false;
                }

                for input in transaction.inputs() {
                    temp_output = temp_UTXOs.get(input.output_id).unwrap();

                    if input.UTXO.value != temp_output.value {
                        return false;
                    }

                    temp_UTXOs.remove(input.output_id);
                }

                for output in transaction.output() {
                    temp_UTXOs.insert(output.id, output);
                }

                if transaction.outputs[0].receiver != transaction.receiver {
                    return false;
                }

                if transaction.outputs[1].receiver != transaction.sender {
                    return false;
                }
            }
        }

        true 
    }
}

