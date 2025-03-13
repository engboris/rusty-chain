use anyhow::{Error, Result};
use rusty_chain::core::{
    blockchain::{Block, BlockHeader, Blockchain, NB_TXN_PER_BLOCK},
    txn::{Transaction, decode},
};
use std::{
    collections::VecDeque,
    io::prelude::*,
    net::{TcpListener, TcpStream},
};

fn create_block(blockchain: &mut Blockchain, mempool: &mut VecDeque<Transaction>) {
    log::info!("Creating block...");
    let last_block = blockchain.get_last_block();
    let mut new_block = Block {
        hash: String::new(),
        header: BlockHeader {
            prev_hash: last_block.hash.clone(),
            nounce: last_block.header.nounce,
        },
        txn: vec![],
    };
    for _ in 0..NB_TXN_PER_BLOCK {
        new_block.txn.push(mempool.pop_front().unwrap())
    }
    blockchain.mint(&mut new_block);
    log::info!("Block {} minted.", new_block.hash);
}

fn handle_connection(
    mut stream: TcpStream,
    mempool: &mut VecDeque<Transaction>,
    blockchain: &mut Blockchain,
) -> Result<()> {
    log::info!("Handling connection.");
    let mut buffer = [0; size_of::<Transaction>()];

    loop {
        match stream.read_exact(&mut buffer) {
            Ok(()) => (),
            Err(_) => return Ok(()),
        }
        if let Some(buffer) = decode(&buffer) {
            mempool.push_back(buffer.clone());
            log::info!("Received transaction: {:?}", buffer)
        }
        if mempool.len() >= NB_TXN_PER_BLOCK {
            create_block(blockchain, mempool);
            log::info!("Blockchain has {} blocks.", blockchain.len());
        }
    }
}

pub fn create_listener(addr: &str) -> Result<()> {
    let listener = TcpListener::bind(addr).unwrap();
    log::info!("Server listening on {addr}...");

    let mut mempool = VecDeque::<Transaction>::new();
    let mut blockchain = Blockchain::new();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream, &mut mempool, &mut blockchain),
            Err(e) => Err(Error::new(e)),
        }?;
    }
    Ok(())
}
