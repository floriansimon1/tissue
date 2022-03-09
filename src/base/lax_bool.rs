static TRUTHIES: &'static [&'static str] = &["yes", "true",  "y", "&check;", "✓"];
static FALSIES:  &'static [&'static str] = &["no",  "false", "n", "&cross;", "✗"];

pub fn parse_lax_bool(raw_value: &str) -> Option<bool> {
    let lowercased = raw_value.chars().map(|character| character.to_ascii_lowercase());

    if TRUTHIES.iter().any(|value| value.chars().eq(lowercased.clone())) {
        Some(true)
    } else if FALSIES.iter().any(|value| value.chars().eq(lowercased.clone())) {
        Some(false)
    } else {
        None
    }
}
