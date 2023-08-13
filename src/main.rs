// source $HOME/.cargo/env  # to set up the environment

// import modules from other files
mod user_handling {
    pub mod account;
    pub mod user;
}

use crate::user_handling::account::Account;
use crate::user_handling::account::AccountTypes;
use crate::user_handling::user::User;

fn main() {

    let mut sample_checking_account = Account {
        name: "Dean's Checking Account".to_string(), 
        balance: 630.00, 
        spending_limit: 150.25,
        transactions: vec![600.0, -250.00],
        account_type: AccountTypes::Checking};

    println!("Account name: {}\nCurrent balance: ${}\nSpending limit: {}\nRecent Transactions: {:#?}",
        sample_checking_account.name,
        sample_checking_account.balance,
        sample_checking_account.spending_limit,
        sample_checking_account.transactions);
    

    println!("Hello, world!");
}
