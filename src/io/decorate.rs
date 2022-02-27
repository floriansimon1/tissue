use colored;

pub fn list_element(list_element: &str) -> String {
    format!("• {list_element}")
}

pub fn decorate_placeholder(placeholder: &str) -> String {
    use colored::Colorize;

    format!("<{placeholder}>").yellow().to_string()
}

pub fn decorate_success(message: &str) -> String {
    use colored::Colorize;

    format!("{} {message}", "✔".green())
}
