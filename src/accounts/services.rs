use super::schemas::{Account, CreateAccountInput};
use rusqlite::{Connection, params};

// Add account
pub fn add_account(conn: &Connection, input: CreateAccountInput) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO accounts (name, amount_cents) VALUES (?1, ?2)",
        params![input.name, input.amount_cents],
    )?;
    Ok(())
}

// List accounts
pub fn list_accounts(conn: &Connection) -> rusqlite::Result<(Vec<Account>, i64)> {
    let accounts_data = _fetch_accounts(conn)?;
    let total_cents = _fetch_total(conn)?;

    let accounts = accounts_data
        .into_iter()
        .map(|(id, name, amount_cents)| Account {
            id,
            name,
            amount_cents,
            percentage: _calculate_percentage(amount_cents, total_cents),
        })
        .collect();

    Ok((accounts, total_cents))
}

fn _fetch_accounts(conn: &Connection) -> rusqlite::Result<Vec<(i64, String, i64)>> {
    let mut stmt = conn.prepare("SELECT id, name, amount_cents FROM accounts ORDER BY amount_cents DESC")?;
    let rows = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?;

    rows.collect()
}

fn _fetch_total(conn: &Connection) -> rusqlite::Result<i64> {
    conn.query_row(
        "SELECT COALESCE(SUM(amount_cents), 0) FROM accounts",
        [],
        |row| row.get(0),
    )
}

fn _calculate_percentage(amount: i64, total: i64) -> f64 {
    if total > 0 {
        (amount as f64 / total as f64) * 100.0
    } else {
        0.0
    }
}
