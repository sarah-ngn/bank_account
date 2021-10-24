use chrono::{DateTime, Local};

pub struct Transaction {
    pub date: DateTime<Local>,
    pub amount: f32
}

impl Clone for Transaction {
    fn clone(&self) -> Transaction {
        Transaction {
            date: self.date,
            amount: self.amount
        }
    }
}