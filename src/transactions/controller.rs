use crate::core::utils::parse_decimal_to_cents;
use crate::transactions::schemas::{CreateTransactionInput, TransactionType};
use crate::transactions::services::add_transaction;
use crate::transactions::view::render_add_transaction;
use rusqlite::Connection;

pub fn handle_add_transaction(
    conn: &Connection,
    account_id: i64,
    transaction_type: &str,
    value: &str,
    description: Option<String>,
) -> Result<String, String> {
    let cents = parse_decimal_to_cents(value).map_err(|err| format!("Valor inválido: {err}"))?;
    let tx_type = TransactionType::from_str(transaction_type)
        .map_err(|err| format!("Tipo de transação inválido: {err}"))?;
    let input = CreateTransactionInput::new(account_id, tx_type, cents, description)
        .map_err(|err| format!("Erro ao validar transação: {err}"))?;
    add_transaction(conn, input).map_err(|err| format!("Erro ao registrar transação: {err}"))?;

    Ok(render_add_transaction(account_id, tx_type, cents))
}
