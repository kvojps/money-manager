use rusqlite::Connection;
use std::fs;
use std::path::PathBuf;

refinery::embed_migrations!("migrations");

pub fn build_db_path() -> Result<PathBuf, String> {
    let home = dirs::home_dir().ok_or_else(|| "Nao foi possivel localizar a home do usuario".to_string())?;
    let dir = home.join(".money-manager");
    fs::create_dir_all(&dir).map_err(|err| format!("Falha ao criar pasta {dir:?}: {err}"))?;
    Ok(dir.join("money_manager.db"))
}

pub fn init_db(conn: &mut Connection) -> Result<(), Box<dyn std::error::Error>> {
    migrations::runner().run(conn)?;
    Ok(())
}
