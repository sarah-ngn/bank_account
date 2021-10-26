use std::result;
use crate::transaction::Transaction;
use chrono;

struct BankAccount {
    id: u32,
    transactions: Vec<Transaction>
}

impl BankAccount {
    fn new(id: u32, transactions: &Vec<Transaction>) -> BankAccount{
        BankAccount{
            id,
            transactions: transactions.to_vec()
        }
    }

    fn deposit_amount(&mut self, amount: f32) -> Result<f32,&str> {
        if amount <= 0.0 {
            Err("Deposit : cannot deposit an amount below 0")
        } else {
            let transaction = Transaction {
                date: chrono::offset::Local::now(),
                amount
            };

            self.transactions.push(transaction);
            Ok(amount)
        }
    }

    fn withdraw_amount(&mut self, amount: f32) -> Result<f32,&str> {
        if amount <= 0.0 {
            Err("Withdraw : cannot withdraw an amount below 0")
        } else {
            let transaction = Transaction {
                date: chrono::offset::Local::now(),
                amount: amount * (-1.0)
            };

            self.transactions.push(transaction);
            Ok(amount)
        }
    }

}

#[test]
fn deposit_positive_amount() {
    let mut bank_account = BankAccount::new(1, &Vec::new());
    assert_eq!(bank_account.deposit_amount(1.0), Ok(1.0));
}

#[test]
#[should_panic]
fn deposit_negative_amount() {
    let mut bank_account = BankAccount::new(1, &Vec::new());
    assert_eq!(bank_account.deposit_amount(-1.0), Ok(-1.0));
}

#[test]
fn withdraw_positive_amount() {
    let mut bank_account = BankAccount::new(1, &Vec::new());
    assert_eq!(bank_account.deposit_amount(1.0), Ok(1.0));
}

#[test]
#[should_panic]
fn withdraw_negative_amount() {
    let mut bank_account = BankAccount::new(1, &Vec::new());
    assert_eq!(bank_account.deposit_amount(-1.0), Ok(-1.0));
}