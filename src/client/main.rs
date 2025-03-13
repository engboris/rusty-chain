use rusty_chain::core::txn::{Encodable, Transaction};
use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:8080";
    let mut stream = TcpStream::connect(addr)?;

    println!("Connection established with {addr}.");
    let tx = Transaction {
        from: 1,
        to: 2,
        amount: 1,
    };

    stream.write_all(&tx.encode())?;

    println!("Message sent to server.");

    Ok(())
}
