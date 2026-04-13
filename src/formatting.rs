use ratatui::layout::Rect;

pub fn format_currency(amount: f64) -> String {
    let rounded = (amount * 100.0).round() / 100.0;
    let integer = rounded.trunc() as i64;
    let fraction = ((rounded.fract() * 100.0).round() as i64).abs();
    let negative = integer < 0;
    let digits = integer.abs().to_string();
    let reversed = digits.chars().rev().collect::<Vec<_>>();

    let mut grouped = String::new();
    for (idx, ch) in reversed.iter().enumerate() {
        if idx > 0 && idx % 3 == 0 {
            grouped.push('.');
        }
        grouped.push(*ch);
    }

    let mut formatted = grouped.chars().rev().collect::<String>();
    if fraction > 0 {
        formatted.push(',');
        formatted.push_str(&format!("{fraction:02}"));
    }

    if negative {
        format!("-Rp {formatted}")
    } else {
        format!("Rp {formatted}")
    }
}

pub fn compact_currency(amount: f64) -> String {
    let abs = amount.abs();
    if abs >= 1_000_000.0 {
        format!("{:.1}jt", amount / 1_000_000.0)
    } else if abs >= 1_000.0 {
        format!("{:.0}k", amount / 1_000.0)
    } else {
        format!("{:.0}", amount)
    }
}

pub fn compact_number(amount: f64) -> String {
    let abs = amount.abs();
    if abs >= 1_000_000_000.0 {
        format!("{:.2}b", amount / 1_000_000_000.0)
    } else if abs >= 1_000_000.0 {
        format!("{:.2}m", amount / 1_000_000.0)
    } else if abs >= 1_000.0 {
        format!("{:.1}k", amount / 1_000.0)
    } else {
        format!("{amount:.2}")
    }
}

pub fn format_decimal(amount: f64) -> String {
    if amount.abs() >= 1_000.0 {
        format!("{amount:.2}")
    } else if amount.abs() >= 1.0 {
        format!("{amount:.4}")
    } else {
        format!("{amount:.6}")
    }
}

pub fn format_percent(amount: f64) -> String {
    if amount >= 0.0 {
        format!("+{amount:.2}%")
    } else {
        format!("{amount:.2}%")
    }
}

pub fn short_date_label(date: &str) -> String {
    if date.len() >= 10 {
        date[5..10].to_string()
    } else {
        date.to_string()
    }
}

pub fn center_horizontally(area: Rect, width: u16) -> Rect {
    let centered_width = width.min(area.width).max(1);
    let x = area.x + (area.width.saturating_sub(centered_width)) / 2;
    Rect {
        x,
        y: area.y,
        width: centered_width,
        height: area.height,
    }
}
