
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

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_accounts(&self) -> &Vec<Account> {
        &self.accounts
    }

    pub fn set_accounts(&mut self, accounts: Vec<Account>) {
        self.accounts = accounts;
    }

    pub fn get_bank_name(&self) -> &String {
        &self.bank_name
    }

    pub fn set_bank_name(&mut self, bank_name: String) {
        self.bank_name = bank_name;
    }

    pub fn get_bank_api_key(&self) -> &String {
        &self.bank_api_key
    }

    pub fn set_bank_api_key(&mut self, bank_api_key: String) {
        self.bank_api_key = bank_api_key;
    }
}