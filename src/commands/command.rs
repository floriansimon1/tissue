use crate::commands::{lint_issue, show_issue};

#[derive(Debug)]
pub enum Command {
    Help,
    List,
    Lint(lint_issue::LintCommand),
    Show(show_issue::ShowCommand),
}
