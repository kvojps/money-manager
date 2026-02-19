pub fn parse_decimal_to_cents(input: &str) -> Result<i64, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err("valor vazio".to_string());
    }

    let mut value = trimmed.to_string();
    if value.contains(',') && !value.contains('.') {
        value = value.replace(',', ".");
    }

    let (negative, digits) = if let Some(rest) = value.strip_prefix('-') {
        (true, rest)
    } else {
        (false, value.as_str())
    };

    let parts: Vec<&str> = digits.split('.').collect();
    if parts.len() > 2 {
        return Err("formato invalido".to_string());
    }

    let whole = parts[0];
    let whole_value: i64 = whole
        .parse()
        .map_err(|_| "parte inteira invalida".to_string())?;

    let frac_value = if parts.len() == 2 {
        let frac = parts[1];
        if frac.len() > 2 {
            return Err("use no maximo duas casas decimais".to_string());
        }
        let padded = if frac.len() == 1 {
            format!("{frac}0")
        } else if frac.is_empty() {
            "00".to_string()
        } else {
            frac.to_string()
        };
        padded
            .parse::<i64>()
            .map_err(|_| "parte decimal invalida".to_string())?
    } else {
        0
    };

    let cents = whole_value
        .checked_mul(100)
        .and_then(|v| v.checked_add(frac_value))
        .ok_or_else(|| "valor muito grande".to_string())?;

    Ok(if negative { -cents } else { cents })
}

pub fn format_cents(cents: i64) -> String {
    let abs = cents.abs();
    let whole = abs / 100;
    let frac = abs % 100;
    if cents < 0 {
        format!("-{}.{:02}", whole, frac)
    } else {
        format!("{}.{:02}", whole, frac)
    }
}
