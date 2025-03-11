use crate::txn;

pub const NB_TXN_PER_BLOCK: usize = 3;

#[derive(Debug, Clone)]
pub struct BlockHeader {
    pub prev_hash: i8,
    pub nounce: i8,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub hash: i8,
    pub header: BlockHeader,
    pub txn: Vec<txn::Transaction>,
}

#[derive(Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

fn genesis_block() -> Block {
    Block {
        hash: 0,
        header: BlockHeader {
            prev_hash: 0,
            nounce: 0,
        },
        txn: vec![],
    }
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            blocks: vec![genesis_block()],
        }
    }
    pub fn mint(&mut self, block: Block) {
        self.blocks.push(block);
    }
    pub fn get_last_block(&self) -> &Block {
        self.blocks.last().expect("Erorr: no last block.")
    }
}
