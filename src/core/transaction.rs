pub trait Encodable {
    fn encode(&self) -> Vec<u8>;
}

pub type Address = String;

#[derive(Debug, Clone)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u8,
}

impl Encodable for Transaction {
    fn encode(&self) -> Vec<u8> {
        let mut v = Vec::new();
        v.extend(self.from.as_bytes());
        v.push(0);
        v.extend(self.to.as_bytes());
        v.push(0);
        v.push(self.amount);
        v
    }
}

impl Transaction {
    pub fn decode(bytes: &[u8]) -> Option<Transaction> {
        if bytes.len() < 3 {
            return None;
        }
        let mut parts = bytes.split(|&b| b == 0);
        let from_bytes = parts.next()?;
        let to_bytes = parts.next()?;
        let amount_byte = parts.next()?.first()?;

        Some(Transaction {
            from: String::from_utf8(from_bytes.to_vec()).ok()?,
            to: String::from_utf8(to_bytes.to_vec()).ok()?,
            amount: *amount_byte,
        })
    }
}
