use rusqlite::Connection;
use std::fs;
use std::path::PathBuf;

pub fn build_db_path() -> Result<PathBuf, String> {
    let home = dirs::home_dir().ok_or_else(|| "Nao foi possivel localizar a home do usuario".to_string())?;
    let dir = home.join(".money-manager");
    fs::create_dir_all(&dir).map_err(|err| format!("Falha ao criar pasta {dir:?}: {err}"))?;
    Ok(dir.join("money_manager.db"))
}

pub fn init_db(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS accounts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            amount_cents INTEGER NOT NULL
        )",
        [],
    )?;
    Ok(())
}
