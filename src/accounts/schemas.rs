#[derive(Debug)]
pub struct CreateAccountInput {
    pub name: String,
    pub amount_cents: i64,
}

impl CreateAccountInput {
    pub fn new(name: String, amount_cents: i64) -> Result<Self, String> {
        if name.trim().is_empty() {
            return Err("Account name cannot be empty".to_string());
        }

        if name.len() > 100 {
            return Err("Account name cannot exceed 100 characters".to_string());
        }

        Ok(Self {
            name: name.trim().to_string(),
            amount_cents,
        })
    }
}

#[derive(Debug)]
pub struct Account {
    pub id: i64,
    pub name: String,
    pub amount_cents: i64,
    pub percentage: f64,
}
