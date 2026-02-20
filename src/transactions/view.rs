use crate::core::utils::format_cents;
use crate::transactions::schemas::TransactionType;
use owo_colors::OwoColorize;

pub fn render_add_transaction(
    account_id: i64,
    transaction_type: TransactionType,
    cents: i64,
) -> String {
    let type_label = match transaction_type {
        TransactionType::Add => "adicionado a",
        TransactionType::Subtract => "removido de",
    };
    let type_color_label = match transaction_type {
        TransactionType::Add => format!("{}", type_label.bright_green()),
        TransactionType::Subtract => format!("{}", type_label.bright_red()),
    };
    format!(
        "{} {} {} conta #{} - {}",
        "✓".green().bold(),
        format_cents(cents).cyan(),
        type_color_label,
        account_id.to_string().yellow().bold(),
        "Transação registrada".bright_green()
    )
}
