use crate::Transaction;
use std::fmt;

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
    pub fn new(name: String, balance: f64, spending_limit: f64, transactions: Vec<Transaction>, account_type: AccountTypes) -> Account {
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

// formatter for printing out account type
impl fmt::Display for AccountTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AccountTypes::Checking => write!(f, "Deposit"),
            AccountTypes::Savings => write!(f, "Withdrawal"),
            AccountTypes::Credit => write!(f, "Transfer"),
            AccountTypes::Investment => write!(f, "Transfer"),
        }
    }
}


// formatter for printing account information & transactions
impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Account Name: {}", self.name)?;
        writeln!(f, "Account Type: {}", self.account_type)?;
        writeln!(f, "Balance: {}", self.balance)?;
        writeln!(f, "Spending Limit: {}", self.spending_limit)?;
        writeln!(f, "Transactions:")?;
        for transaction in &self.transactions {
            writeln!(f, "{}", transaction)?;
        }
        Ok(())
    }
}