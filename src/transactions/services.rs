use super::schemas::{CreateTransactionInput, TransactionType};
use rusqlite::{Connection, params};

pub fn add_transaction(conn: &Connection, input: CreateTransactionInput) -> rusqlite::Result<()> {
    let account_exists: bool = conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM accounts WHERE id = ?1)",
        params![input.account_id],
        |row| row.get(0),
    )?;
    if !account_exists {
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }

    conn.execute(
        "INSERT INTO transactions (account_id, transaction_type, amount_cents, description)
         VALUES (?1, ?2, ?3, ?4)",
        params![
            input.account_id,
            input.transaction_type.as_str(),
            input.amount_cents,
            input.description
        ],
    )?;

    let delta = match input.transaction_type {
        TransactionType::Add => input.amount_cents,
        TransactionType::Subtract => -input.amount_cents,
    };
    conn.execute(
        "UPDATE accounts SET amount_cents = amount_cents + ?1 WHERE id = ?2",
        params![delta, input.account_id],
    )?;

    Ok(())
}
