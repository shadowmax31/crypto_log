pub enum TransactionType {
    Buy,
    Sell
}

impl TransactionType {
    pub fn value(&self) -> String {
        match self {
            TransactionType::Buy => String::from("buy"),
            TransactionType::Sell => String::from("sell")
        }
    }
}