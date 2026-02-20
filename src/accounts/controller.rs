use crate::accounts::services::{add_account, list_accounts};
use crate::accounts::view::{render_add_account, render_list_accounts};
use crate::core::utils::parse_decimal_to_cents;
use rusqlite::Connection;

pub fn handle_add(conn: &Connection, name: &str, value: &str) -> Result<String, String> {
    let cents = parse_decimal_to_cents(value).map_err(|err| format!("Valor invalido: {err}"))?;
    add_account(conn, name, cents).map_err(|err| format!("Erro ao salvar a conta: {err}"))?;

    Ok(render_add_account(name, cents))
}

pub fn handle_list(conn: &Connection) -> Result<String, String> {
    let (rows, total) =
        list_accounts(conn).map_err(|err| format!("Erro ao listar contas: {err}"))?;

    Ok(render_list_accounts(&rows, total))
}
