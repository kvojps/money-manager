use crate::accounts::schemas::Account;
use crate::core::utils::format_cents;
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

    let mut output = String::new();
    output.push_str(&format!(
        "\n{}\n",
        "💰 CONTAS CADASTRADAS".bright_blue().bold()
    ));
    output.push_str(&format!("{}\n", "─".repeat(70).bright_black()));

    for account in accounts {
        let bar = _generate_bar(account.percentage, 15);
        output.push_str(&format!(
            "#{:<2} {:<15} {:<12} {}  {:.2}%\n",
            account.id,
            account.name.cyan().bold(),
            format_cents(account.amount_cents).green().bold(),
            bar,
            account.percentage
        ));
    }

    output.push_str(&format!("{}\n", "─".repeat(70).bright_black()));
    output.push_str(&format!(
        "{}  {}\n\n",
        "TOTAL:".bright_yellow().bold(),
        format_cents(total_cents).bright_green().bold()
    ));

    output
}

fn _generate_bar(percentage: f64, width: usize) -> String {
    let filled = (percentage / 100.0 * width as f64) as usize;
    let empty = width.saturating_sub(filled);

    format!(
        "[{}{}]",
        "█".repeat(filled).cyan(),
        "░".repeat(empty).bright_black(),
    )
}
