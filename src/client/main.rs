use rusty_chain::core::transaction::{Encodable, Transaction};
use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;

fn connect() -> Option<TcpStream> {
    let peers_addr: Vec<String> = fs::read_to_string("src/client/peers")
        .unwrap()
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();
    for addr in peers_addr {
        println!("Connecting to peer {addr}...");
        match TcpStream::connect(&addr) {
            Ok(stream) => {
                println!("Connected to peer {addr}.");
                return Some(stream);
            }
            Err(e) => {
                println!("Connection failed: {e}");
            }
        }
    }
    None
}

fn main() -> std::io::Result<()> {
    println!("Looking for peers...");
    let Some(mut stream) = connect() else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Could not find peer.",
        ));
    };

    let tx = Transaction {
        from: String::from("a"),
        to: String::from("b"),
        amount: 1,
    };

    stream.write_all(&tx.encode())?;
    stream.shutdown(std::net::Shutdown::Write)?;

    println!("Message sent to server.");

    Ok(())
}
