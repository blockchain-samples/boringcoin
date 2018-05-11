use block::Block;
use transaction::TransactionOutput;
use std::collections::HashMap;

pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub UTXOs: HashMap<String, TransactionOutput>,
    pub difficulty: i32,
    pub minimum_transaction: f32,
}

impl Blockchain {
    fn new() -> Blockchain {
        //let genesis_block = Block::new("and so it begins", "0");

        Blockchain {
            blocks: Vec::new(), // todo -> vec![genesis_block],
            UTXOs: HashMap::new(),
            difficulty: 5,
            minimum_transaction: 2.5,
        }
    }

    fn new_block(&self, data: String) -> Block {
        let last_block = self.blocks.last().unwrap();
        Block::new(last_block.hash)
    }

    pub fn add_block(&self, block: Block) {
        block.mine(self.difficulty);
        self.blocks.push(block);
    }

    fn is_valid(&self) -> bool {
        let hash_target = "0".repeat(self.difficulty as usize);
        let mut temp_UTXOs: HashMap<String, TransactionOutput> = HashMap::new();
        temp_UTXOs.insert(genesis_transaction.outputs[0].id, genesis_transaction.outputs[0]);

        for block_pair in self.blocks.windows(2) {
            let prev_block = block_pair[0];
            let next_block = block_pair[1];

            if next_block.hash != next_block.calc_hash() {
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

                for input in transaction.inputs {
                    temp_output = temp_UTXOs.get(&input.output_id).unwrap();

                    if input.UTXO.value != temp_output.value {
                        return false;
                    }

                    temp_UTXOs.remove(&input.output_id);
                }

                for output in transaction.outputs {
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

