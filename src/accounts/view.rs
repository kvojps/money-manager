use crate::accounts::schemas::Account;
use crate::core::utils::format_cents;
use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table, presets::UTF8_FULL};
use owo_colors::OwoColorize;

// Add account
pub fn render_add_account(name: &str, cents: i64) -> String {
    format!(
        "{} {} - {}",
        "✓".green().bold(),
        "Conta cadastrada:".bright_green(),
        format!("{} ({})", name.bold(), format_cents(cents).cyan())
    )
}

// List accounts
pub fn render_list_accounts(accounts: &[Account], total_cents: i64) -> String {
    if accounts.is_empty() {
        return format!("{}", "Nenhuma conta cadastrada.".yellow());
    }

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("ID")
                .fg(Color::Cyan)
                .add_attribute(Attribute::Bold),
            Cell::new("Conta")
                .fg(Color::Cyan)
                .add_attribute(Attribute::Bold),
            Cell::new("Valor")
                .fg(Color::Cyan)
                .add_attribute(Attribute::Bold),
            Cell::new("Proporção")
                .fg(Color::Cyan)
                .add_attribute(Attribute::Bold),
        ]);

    for account in accounts {
        let bar = _generate_bar_chart(account.percentage, 20);

        table.add_row(vec![
            Cell::new(format!("#{}", account.id)).fg(Color::DarkGrey),
            Cell::new(&account.name),
            Cell::new(format_cents(account.amount_cents)).fg(if account.amount_cents >= 0 {
                Color::Green
            } else {
                Color::Red
            }),
            Cell::new(bar),
        ]);
    }

    let mut output = String::new();
    output.push_str(&format!(
        "\n{}\n\n",
        "💰 CONTAS CADASTRADAS".bright_blue().bold()
    ));
    output.push_str(&table.to_string());
    output.push_str(&format!(
        "\n{} {}\n",
        "TOTAL:".bright_yellow().bold(),
        format_cents(total_cents).bright_green().bold()
    ));

    output
}

fn _generate_bar_chart(percentage: f64, width: usize) -> String {
    let filled = (percentage / 100.0 * width as f64) as usize;
    let empty = width.saturating_sub(filled);

    format!(
        "{}{} {:>6.2}%",
        "█".repeat(filled).cyan(),
        "░".repeat(empty).bright_black(),
        percentage
    )
}
