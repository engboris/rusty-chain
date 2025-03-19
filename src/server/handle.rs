use anyhow::{Error, Result};
use rusty_chain::core::{
    blockchain::{Block, BlockHeader, Blockchain, NB_TXN_PER_BLOCK},
    transaction::Transaction,
};
use std::{
    collections::VecDeque,
    io::{self, prelude::*},
    net::{TcpListener, TcpStream},
    time::SystemTime,
};

fn create_block(blockchain: &mut Blockchain, mempool: &mut VecDeque<Transaction>) {
    log::info!("Creating block...");
    let last_block = blockchain.get_last_block();
    let mut new_block = Block {
        hash: String::new(),
        time: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis(),
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

pub async fn decode_from_stream(stream: &mut TcpStream) -> io::Result<Option<Transaction>> {
    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer)?;
    Ok(Transaction::decode(&buffer))
}

async fn handle_connection(
    mut stream: TcpStream,
    mempool: &mut VecDeque<Transaction>,
    blockchain: &mut Blockchain,
) -> Result<()> {
    log::info!("Handling connection.");

    loop {
        match decode_from_stream(&mut stream).await {
            Err(e) => log::error!("Error: {e}"),
            Ok(None) => return Ok(()),
            Ok(Some(tx)) => {
                if Blockchain::valid_transaction(blockchain, &tx) {
                    mempool.push_back(tx.clone());
                    log::info!("Received transaction: {:?}", tx)
                } else {
                    log::error!("Rejected transaction: {:?}", tx)
                };
            }
        }
        if mempool.len() >= NB_TXN_PER_BLOCK {
            create_block(blockchain, mempool);
            log::info!("Blockchain has {} blocks.", blockchain.len());
        }
    }
}

pub async fn create_listener(addr: &str) -> Result<()> {
    let listener = TcpListener::bind(addr).unwrap();
    log::info!("Server listening on {addr}...");

    let mut mempool = VecDeque::<Transaction>::new();
    let mut blockchain = Blockchain::new();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream, &mut mempool, &mut blockchain).await,
            Err(e) => Err(Error::new(e)),
        }?;
    }
    Ok(())
}
