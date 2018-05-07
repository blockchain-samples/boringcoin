struct Blockchain {
    blocks: Vec<Block>,
    difficulty: i32,
}

impl Blockchain {
    fn new() -> Blockchain {
        let genesis_block = Block::new("and so it begins", "0");

        Blockchain {
            blocks: vec![genesis_block],
            difficulty: 5,
        }
    }

    fn add_block(&self, data: String) {
        let last_block = self.blocks.last().unwrap();
        let new_block = Block::new(data, last_block.hash);
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
        }

        true 
    }
}

