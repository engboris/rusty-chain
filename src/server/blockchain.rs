use rusty_chain::common::txn::Transaction;
use sha2::{Digest, Sha256};

pub const NB_TXN_PER_BLOCK: usize = 3;
pub const HASH_DIFFICULTY: usize = 3;

#[derive(Debug, Clone, Hash)]
pub struct BlockHeader {
    pub prev_hash: String,
    pub nounce: u64,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub hash: String,
    pub header: BlockHeader,
    pub txn: Vec<Transaction>,
}

impl Block {
    pub fn valid_hash(&self) -> bool {
        self.hash.starts_with(&"0".repeat(HASH_DIFFICULTY))
    }
    pub fn calculate_hash(&mut self) {
        self.hash = hex::encode(Sha256::digest(&self.hash));
    }
}

#[derive(Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block {
            hash: String::new(),
            header: BlockHeader {
                prev_hash: String::new(),
                nounce: 0,
            },
            txn: vec![],
        };
        Blockchain {
            blocks: vec![genesis_block],
        }
    }
    pub fn len(&self) -> usize {
        self.blocks.len()
    }
    pub fn mint(&mut self, block: &mut Block) {
        block.calculate_hash();
        while !block.valid_hash() {
            block.header.nounce += 1;
            println!("Nounce={}, hash={}...", block.header.nounce, block.hash);
            block.calculate_hash();
        }
        self.blocks.push((*block).clone());
    }
    pub fn get_last_block(&self) -> &Block {
        self.blocks.last().expect("Erorr: no last block.")
    }
}
