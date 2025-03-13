pub trait Encodable {
    fn encode(&self) -> Vec<u8>;
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub from: u8,
    pub to: u8,
    pub amount: u8,
}

impl Encodable for Transaction {
    fn encode(&self) -> Vec<u8> {
        vec![self.from, self.to, self.amount]
    }
}

pub fn decode(bytes: &[u8]) -> Option<Transaction> {
    if bytes.len() < 3 {
        return None;
    }
    Some(Transaction {
        from: bytes[0],
        to: bytes[1],
        amount: bytes[2],
    })
}
