use rusqlite::{Connection, params};

pub fn add_account(conn: &Connection, name: &str, cents: i64) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO accounts (name, amount_cents) VALUES (?1, ?2)",
        params![name, cents],
    )?;
    Ok(())
}

pub fn list_accounts(conn: &Connection) -> rusqlite::Result<Vec<(i64, String, i64)>> {
    let mut stmt = conn.prepare("SELECT id, name, amount_cents FROM accounts ORDER BY id ASC")?;
    let rows = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?;

    let mut results = Vec::new();
    for row in rows {
        results.push(row?);
    }
    Ok(results)
}
