pub struct Transaction {
    pub id: u32,                // transaction id number, 32bit is prob overkill but oh well
    pub amount: f64,
    pub recipient: String,
    pub description: String,
    pub transaction_type: TransactionType, 
    pub is_recurring: bool,
    pub date: String,           // <-- replace with Date type later?
}

impl Transaction {
    pub fn new(id: u32, amount: f64, recipient: String, description: String, transaction_type: TransactionType, is_recurring: bool, date: String) -> Transaction {
        Transaction {
            id,
            amount,
            recipient,
            description,
            transaction_type,
            is_recurring,
            date
        }
    }
}

pub enum TransactionType {
    Deposit,
    Withdrawal,
    Transfer
}