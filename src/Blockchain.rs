struct Blockchain {
    blocks: Vec<Block>,

}

impl Blockchain {
    fn is_valid(&self) -> bool {
        for block_pair in self.blocks.windows(2) {
            let prev_block = block_pair[0];
            let next_block = block_pair[1];

            if next_block.hash != next_block.calculate_hash() {
                return false;
            }

            if prev_block.hash != next_block.prev_hash {
                return false;
            }
        }

        true 
    }
}
