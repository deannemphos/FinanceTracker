use crate::Transaction;


// each user can have multiple accounts, each account can have multiple transactions and budget limits
// each account has a name, balance, and a list of transactions
// each transaction has a date, amount, and description
// each budget has a name, limit, and a list of transactions
pub struct Account {
    pub name: String,
    pub balance: f64,                   // 64bit float for theoretical billionaires using the app
    pub spending_limit: f64,            // 64bit float for theoretical billionaires using the app
    pub transactions: Vec<Transaction>,
    pub account_type: AccountTypes
}

impl Account {
    pub fn new(name: String, balance: f64, spending_limit: f64, transactions: Vec<f64>, account_type: AccountTypes) -> Account {
        Account {
            name,
            balance,
            spending_limit,
            transactions,
            account_type
        }
    }
}

pub enum AccountTypes {
    Checking,
    Savings,
    Credit,
    Investment
}

