
mod account;

pub struct user {
    pub name: String,
    pub accounts: Vec<account>,
    pub bank_name: String,
    bank_api_key: String
}