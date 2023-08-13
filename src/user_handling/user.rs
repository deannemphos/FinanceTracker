
use crate::Account;

pub struct User {
    pub name: String,
    pub accounts: Vec<Account>,
    pub bank_name: String,
    bank_api_key: String
}

impl User {
    pub fn new(name: String, accounts: Vec<Account>, bank_name: String, bank_api_key: String) -> User {
        User {
            name,
            accounts,
            bank_name,
            bank_api_key
        }
    }
}