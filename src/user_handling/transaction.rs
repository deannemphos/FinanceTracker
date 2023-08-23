use std::fmt;

pub struct Transaction {
    pub id: u32,                // transaction id number, 32bit is prob overkill but oh well
    pub amount: f64,
    pub sender: String,         // whoever sent/requested the money
    pub description: String,
    pub transaction_type: TransactionType, 
    pub is_recurring: bool,
    pub date: String,           // <-- replace with Date type later?
}

impl Transaction {
    pub fn new(id: u32, amount: f64, sender: String, description: String, transaction_type: TransactionType, is_recurring: bool, date: String) -> Transaction {
        Transaction {
            id,
            amount,
            sender,
            description,
            transaction_type,
            is_recurring,
            date
        }
    }

    pub fn print(&self) {
        println!("Transaction ID: {}", self.id);
        println!("Amount: {}", self.amount);
        println!("Sender: {}", self.sender);
        println!("Description: {}", self.description);
        println!("Transaction Type: {}", self.transaction_type);
        println!("Is Recurring: {}", self.is_recurring);
        println!("Date: {}", self.date);
    }
}

pub enum TransactionType {
    Deposit,
    Withdrawal,
    Transfer
}

// formatter for printing out transaction type
impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TransactionType::Deposit => write!(f, "Deposit"),
            TransactionType::Withdrawal => write!(f, "Withdrawal"),
            TransactionType::Transfer => write!(f, "Transfer"),
        }
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Transaction ID: {}\nAmount: {}\nSender: {}\nDescription: {}\nTransaction Type: {}\nIs Recurring: {}\nDate: {}",
        self.id,
        self.amount,
        self.sender,
        self.description,
        self.transaction_type,
        self.is_recurring,
        self.date)
    }
}