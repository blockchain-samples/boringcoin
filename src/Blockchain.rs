use std::collections::HashMap;
use std::iter;

use sodiumoxide::crypto::box_::SecretKey;

use block::Block;
use transaction::{Transaction,TransactionOutput};

#[derive(Clone)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub UTXOs: HashMap<Vec<u8>, TransactionOutput>,
    pub difficulty: i32,
    pub minimum_transaction: f32,
}

impl Blockchain {
    pub fn new(difficulty: i32, minimum: f32) -> Blockchain {
        //let genesis_block = Block::new("and so it begins", "0");

        Blockchain {
            blocks: Vec::new(), // todo -> vec![genesis_block],
            UTXOs: HashMap::new(),
            difficulty,
            minimum_transaction: minimum,
        }
    }

    pub fn new_block(&self, data: String) -> Block {
        let last_block = self.blocks.last().unwrap();
        Block::new(last_block.hash.clone())
    }

    pub fn add_block(&mut self, mut block: Block) -> Vec<u8> {
        block.mine(self.difficulty as usize);
        let hash = block.hash.clone();
        self.blocks.push(block);
        hash
    }

    pub fn is_valid(&self, genesis_transaction: &Transaction) -> Result<(),String> {
        let hash_target: Vec<u8> = iter::repeat(0_u8).take(self.difficulty as usize).collect();
        let mut temp_UTXOs: HashMap<Vec<u8>, TransactionOutput> = HashMap::new();
        temp_UTXOs.insert(genesis_transaction.outputs[0].id.clone(), genesis_transaction.outputs[0].clone());

        // TODO -> THIS IS FAKE
        // TODO -> THIS IS FAKE
        let receiver_priv_key: SecretKey = SecretKey([0_u8;32]);
        // TODO -> THIS IS FAKE
        // TODO -> THIS IS FAKE
        let mut i = -1;
        for block_pair in self.blocks.windows(2) {
            i+=1;
            let prev_block = &block_pair[0];
            let next_block = &block_pair[1];

            if next_block.hash != next_block.calc_hash() {
                return Err(String::from("next_block's hash doesn't match itself"));
            }

            if prev_block.hash != next_block.prev_hash {
                println!("{:?} != {:?}", prev_block.hash, next_block.prev_hash);
                return Err(String::from("next_block.prev_hash doesn't match prev_block.hash"));
            }

            println!("SMELLS LIKE BACON IN THE MORNIN: {:?}", next_block.hash);
            let next_block_hash: Vec<u8> = next_block.hash.clone().into_iter().take(self.difficulty as usize).collect::<Vec<u8>>();
            if &next_block.hash[0..self.difficulty as usize] != &hash_target[..] {
                println!("{} {}", i, i + 1);
                println!("{:?} != {:?}", &next_block.hash[0..self.difficulty as usize], &hash_target[..]);
                return Err(String::from("next_block_hash != hash_target"));
            }
        
            let mut temp_output: TransactionOutput;
            for transaction in next_block.transactions.iter() {
                if !transaction.verify_signature(&receiver_priv_key) {
                    return Err(String::from("transaction cant be verified"));
                }

                if transaction.get_inputs_val() != transaction.get_outputs_val() {
                    return Err(String::from("transaction inputs dont match outputs"));
                }

                for input in transaction.inputs.iter() {
                    // TODO -> FIX THIS NASTY SHIT
                    let cloned = temp_UTXOs.get(&input.output_id).unwrap().clone();
                    temp_output = cloned;

                    if input.UTXO.value != temp_output.value {
                        return Err(String::from("input alue doesn't equal output value"));
                    }

                    temp_UTXOs.remove(&input.output_id);
                }

                for output in transaction.outputs.iter() {
                    temp_UTXOs.insert(output.id.clone(), output.clone());
                }

                if transaction.outputs[0].receiver != transaction.receiver {
                    return Err(String::from("Receivers don't match"));
                }

                if transaction.outputs[1].receiver != transaction.sender {
                    return Err(String::from("receiver doesn't match sender"));
                }
            }
        }

        Ok(()) 
    }
}

