use anyhow::Result;
use rusty_chain::core::{
    blockchain::{Block, BlockHeader, Blockchain, NB_TXN_PER_BLOCK},
    transaction::{Encodable, Transaction},
};
use std::sync::Arc;
use std::{collections::VecDeque, fs, time::SystemTime};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;

// Propagates received transactions to other peer nodes in the network
async fn broadcast(tx: &Transaction) {
    let peers_addr: Vec<String> = fs::read_to_string("src/server/peers")
        .unwrap()
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();
    for addr in peers_addr {
        let mut stream = TcpStream::connect(&addr).await.unwrap();
        stream.write_all(&tx.encode()).await.unwrap();
    }
}

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
    stream.read_to_end(&mut buffer).await.unwrap();
    Ok(Transaction::decode(&buffer))
}

async fn handle_connection(
    mut stream: TcpStream,
    mempool: Arc<Mutex<VecDeque<Transaction>>>,
    blockchain: Arc<Mutex<Blockchain>>,
) -> Result<()> {
    log::info!("Handling connection.");
    let mut mempool = mempool.lock().await;
    let mut blockchain = blockchain.lock().await;

    loop {
        match decode_from_stream(&mut stream).await {
            Err(e) => log::error!("Error: {e}"),
            Ok(None) => return Ok(()),
            Ok(Some(tx)) => {
                if Blockchain::valid_transaction(&blockchain, &tx) {
                    broadcast(&tx).await;
                    mempool.push_back(tx.clone());
                    log::info!("Received transaction: {:?}", tx)
                } else {
                    log::error!("Rejected transaction: {:?}", tx)
                };
            }
        }
        if mempool.len() >= NB_TXN_PER_BLOCK {
            create_block(&mut blockchain, &mut mempool);
            log::info!("Blockchain has {} blocks.", &blockchain.len());
        }
    }
}

pub async fn create_listener(addr: &str) -> Result<()> {
    let listener = TcpListener::bind(addr).await.unwrap();
    log::info!("Server listening on {addr}...");

    let mempool = Arc::new(Mutex::new(VecDeque::<Transaction>::new()));
    let blockchain = Arc::new(Mutex::new(Blockchain::new()));

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let mempool = Arc::clone(&mempool);
        let blockchain = Arc::clone(&blockchain);
        tokio::spawn(handle_connection(stream, mempool, blockchain));
    }
}
