mod blockchain;
mod txn;
use blockchain::{Block, BlockHeader, Blockchain};
use std::{
    collections::VecDeque,
    io::prelude::*,
    net::{TcpListener, TcpStream},
};
use txn::Transaction;

fn handle_connection(mut stream: TcpStream) {
    println!("Handling connection.");
    let mut mempool: VecDeque<Transaction> = VecDeque::new();
    let mut blockchain = Blockchain::new();
    let mut buffer = [0; size_of::<Transaction>()];

    loop {
        if let Ok(()) = stream.read_exact(&mut buffer) {
            if let Some(buffer) = txn::decode(&buffer) {
                mempool.push_back(buffer.clone());
                println!("Received transaction: {:?}", buffer)
            }

            if mempool.len() >= blockchain::NB_TXN_PER_BLOCK {
                println!("Creating block...");
                let last_block = blockchain.get_last_block();
                let mut new_block = Block {
                    hash: 0,
                    header: BlockHeader {
                        prev_hash: last_block.hash,
                        nounce: last_block.header.nounce,
                    },
                    txn: vec![],
                };
                for _ in 0..blockchain::NB_TXN_PER_BLOCK {
                    new_block.txn.push(mempool.pop_front().unwrap())
                }
                blockchain.mint(new_block.clone());
                println!("Block #{} minted.", new_block.hash);
            }
        }
    }
}

fn create_listener(addr: &str) {
    let listener = TcpListener::bind(addr).unwrap();
    println!("Server listening on {addr}...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(e) => println!("{e}"),
        }
    }
}

fn main() {
    create_listener("127.0.0.1:8080");
}
