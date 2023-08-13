
// each user can have multiple accounts, each account can have multiple transactions and budget limits
// each account has a name, balance, and a list of transactions
// each transaction has a date, amount, and description
// each budget has a name, limit, and a list of transactions
pub struct Account {
    pub name: String,
    pub balance: f64,                   // 64bit float for theoretical billionaires using the app
    pub spending_limit: f64,            // 64bit float for theoretical billionaires using the app
    pub transactions: Vec<f64>,
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

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_balance(&self) -> f64 {
        self.balance
    }

    pub fn set_balance(&mut self, balance: f64) {
        self.balance = balance;
    }

    pub fn get_spending_limit(&self) -> f64 {
        self.spending_limit
    }

    pub fn set_spending_limit(&mut self, spending_limit: f64) {
        self.spending_limit = spending_limit;
    }

    pub fn get_transactions(&self) -> &Vec<f64> {
        &self.transactions
    }

    pub fn set_transactions(&mut self, transactions: Vec<f64>) {
        self.transactions = transactions;
    }

    pub fn get_account_type(&self) -> &AccountTypes {
        &self.account_type
    }

    pub fn set_account_type(&mut self, account_type: AccountTypes) {
        self.account_type = account_type;
    }
}

pub enum AccountTypes {
    Checking,
    Savings,
    Credit,
    Investment
}

