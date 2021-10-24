use chrono::{TimeZone,DateTime,Utc};

pub struct Transaction {
    date: DateTime<Utc>,
    amount: f32
}