use std::sync;

use git2;
use antidote;

use crate::phases::global;
use crate::io::{paging, safe_stdio};
use crate::{commands, errors, steps};

#[derive(Debug, Clone)]
pub struct ShowCommand {
    pub issue_name: String,
}

pub fn make_show_command(issue_name: String) -> commands::Command {
    commands::Command::Show(ShowCommand { issue_name })
}

pub fn show_issue(global: sync::Arc<antidote::RwLock<global::Global>>, repository: sync::Arc<antidote::RwLock<git2::Repository>>, issue_name: &str)
-> Result<(), ()> {
    let issue_text = steps
    ::retrieve_issue_text(global.clone(), &*repository.read(), issue_name)
    .map_err(errors::issue_fetch::explain_issue_fetch_error)
    .map_err(String::from)
    .map_err(|error| global.read().logger.log_error(error))?;

    let pager = paging::Pager::new();

    safe_stdio::safe_print(&issue_text);

    pager.wait();

    Ok(())
}
