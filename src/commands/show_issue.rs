use std::sync;

use git2;

use crate::phases::global;
use crate::io::{paging, safe_stdio};
use crate::{commands, errors, steps};

#[derive(Debug)]
pub struct ShowCommand {
    pub issue_name: String,
}

pub fn make_show_command(issue_name: String) -> commands::Command {
    commands::Command::Show(ShowCommand { issue_name })
}

pub fn show_issue(global: &global::Global, repository: sync::Arc<git2::Repository>, issue_name: &str)
-> Result<(), ()> {
    let issue_text = steps
    ::retrieve_issue_text(&global, &repository, issue_name)
    .map_err(errors::issue::explain_error)
    .map_err(String::from)
    .map_err(|error| global.logger.log_error(error))?;

    let pager = paging::Pager::new();

    safe_stdio::safe_println(&issue_text);

    pager.wait();

    Ok(())
}
