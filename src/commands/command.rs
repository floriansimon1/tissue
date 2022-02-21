use crate::commands::show_issue;

#[derive(Debug)]
pub enum Command {
    Help,
    List,
    Show(show_issue::ShowCommand),
}
