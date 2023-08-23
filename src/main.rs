// source $HOME/.cargo/env  # to set up the environment

// import modules from other files
mod user_handling {
    pub mod account;
    pub mod user;
    pub mod transaction;
}

use crate::user_handling::account::Account;
use crate::user_handling::account::AccountTypes;
use crate::user_handling::user::User;
use crate::user_handling::transaction::Transaction;
use crate::user_handling::transaction::TransactionType;

fn main() {

    let mut sample_transaction = Transaction {
        id: 1,
        amount: 100.00,
        sender: "Microsoft Inc.".to_string(),
        description: "Deposit".to_string(),
        transaction_type: TransactionType::Deposit,
        is_recurring: false,
        date: "2021-01-01".to_string()
    };

    let mut sample_checking_account = Account {
        name: "Dean's Checking Account".to_string(), 
        balance: 630.00, 
        spending_limit: 150.25,
        transactions: vec![sample_transaction],
        account_type: AccountTypes::Checking};

    println!("Checking:\n{}",
        sample_checking_account);
    

    println!("Hello, world!");
}
