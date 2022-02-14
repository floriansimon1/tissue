use std::{env, path};

use clap;

use crate::commands;
use crate::base::phase;
use crate::phases::{global, configure_logging_backends};

pub struct ParseCommandLine;

const WORKING_DIRECTORY_ARGUMENT_NAME: &'static str = "Working directory";
const LIST_ISSUES_SUBCOMMAND:          &'static str = "list";

impl phase::NonTerminalPhaseTrait<global::Global> for ParseCommandLine {
    fn name(&self) -> &'static str {
        "ParseCommandLine"
    }

    fn run(self: Box<Self>, global: &mut global::Global) -> phase::Phase<global::Global> {
        let list_issues_subcommand = get_list_issues_subcommand();

        let mut app = clap
        ::App
        ::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .args(get_general_args())
        .subcommand(list_issues_subcommand);

        let version_message  = app.render_long_version();
        let matches_or_error = app.try_get_matches_from_mut(env::args_os());

        if let Err(error) = matches_or_error {
            return match &error.kind() {
                clap::ErrorKind::DisplayHelp    => { app.print_long_help().unwrap(); phase::Phase::TerminalSuccess },
                clap::ErrorKind::DisplayVersion => { print!("{}", version_message);  phase::Phase::TerminalSuccess },
                _                               => { error.print().unwrap();         phase::Phase::TerminalError   },
            }
        }

        let matches = matches_or_error.unwrap();

        global.command = matches
        .subcommand()
        .map(|(subcommand, _)| {
            match subcommand {
                LIST_ISSUES_SUBCOMMAND => commands::Command::List,
                _                      => panic!("A command is configured in the parser but is not handled!")
            }
        })
        .unwrap_or(commands::Command::Help);

        if let commands::Command::Help = global.command {
            app.print_long_help().unwrap();

            return phase::Phase::TerminalSuccess;
        }

        if let Some(path) = matches.value_of(WORKING_DIRECTORY_ARGUMENT_NAME).map(path::PathBuf::from) {
            global.logger.log_info(format!("Setting working directory to `{}`", path.display()));

            global.working_directory_path = path;
        }

        phase::continue_with(Box::new(configure_logging_backends::ConfigureLoggingBackends))
    }
}

fn get_list_issues_subcommand() -> clap::App<'static> {
    clap
    ::App
    ::new(LIST_ISSUES_SUBCOMMAND)
    .about("List all issues in the select project")
}

fn get_general_args() -> Vec<clap::Arg<'static>> {
    vec![
        clap
        ::Arg
        ::new(WORKING_DIRECTORY_ARGUMENT_NAME)
        .takes_value(true)
        .long("working-directory")
        .help("The location of the Git repository containing tissue data")
    ]
}
