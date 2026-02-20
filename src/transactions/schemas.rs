#[derive(Debug)]
pub struct CreateTransactionInput {
    pub account_id: i64,
    pub transaction_type: TransactionType,
    pub amount_cents: i64,
    pub description: Option<String>,
}

impl CreateTransactionInput {
    pub fn new(
        account_id: i64,
        transaction_type: TransactionType,
        amount_cents: i64,
        description: Option<String>,
    ) -> Result<Self, String> {
        if account_id <= 0 {
            return Err("Invalid account ID".to_string());
        }

        if amount_cents <= 0 {
            return Err("Transaction amount must be positive".to_string());
        }

        let description = description
            .map(|desc| {
                if desc.trim().is_empty() {
                    None
                } else if desc.len() > 255 {
                    Some(desc.chars().take(255).collect::<String>())
                } else {
                    Some(desc)
                }
            })
            .flatten();

        Ok(Self {
            account_id,
            transaction_type,
            amount_cents,
            description,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TransactionType {
    Add,
    Subtract,
}

impl TransactionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            TransactionType::Add => "add",
            TransactionType::Subtract => "subtract",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "add" => Ok(TransactionType::Add),
            "subtract" => Ok(TransactionType::Subtract),
            _ => Err(format!("Invalid transaction type: {}", s)),
        }
    }
}
