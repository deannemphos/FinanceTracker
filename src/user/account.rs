
// each user can have multiple accounts, each account can have multiple transactions and budget limits
// each account has a name, balance, and a list of transactions
// each transaction has a date, amount, and description
// each budget has a name, limit, and a list of transactions
pub struct account {
    pub name: String,
    pub balance: f64,
    pub spending_limit: f64,
    pub transactions: Vec<f64>,
    pub account_type: account_types
}

pub enum account_types {
    checking,
    savings,
    credit,
    investment
}