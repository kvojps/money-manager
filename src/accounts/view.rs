use crate::core::utils::format_cents;

pub fn render_add_account(name: &str, cents: i64) -> String {
    format!("Conta cadastrada: {name} - {}", format_cents(cents))
}

pub fn render_list_accounts(rows: &[(i64, String, i64)]) -> String {
    if rows.is_empty() {
        return "Nenhuma conta cadastrada.".to_string();
    }

    let mut output = String::new();
    for (index, (id, name, cents)) in rows.iter().enumerate() {
        if index > 0 {
            output.push('\n');
        }
        output.push_str(&format!("#{id} | {name} | {}", format_cents(*cents)));
    }
    output
}
