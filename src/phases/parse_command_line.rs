use std::path;

use clap;

use crate::base::phase;
use crate::phases::{global, verify_git_repository};

pub struct ParseCommandLine;

impl phase::NonTerminalPhaseTrait<global::Global> for ParseCommandLine {
    fn name(&self) -> &'static str {
        "ParseCommandLine"
    }

    fn run(self: Box<Self>, global: &mut global::Global) -> phase::Phase<global::Global> {
        let working_directory_argument_name = "Working directory";

        let working_directory_argument = clap
        ::Arg
        ::new(working_directory_argument_name)
        .takes_value(true)
        .long("working-directory")
        .help("The location of the Git repository containing tissue data");

        let mut app = clap
        ::App
        ::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg(working_directory_argument);

        let usage           = app.render_usage();
        let version_message = app.render_version();

        let matches         = app.try_get_matches();

        if let Err(error) = matches {
            return match &error.kind() {
                clap::ErrorKind::DisplayVersion => { print!("{}", version_message); phase::Phase::TerminalSuccess },
                clap::ErrorKind::DisplayHelp    => { println!("{}", usage);         phase::Phase::TerminalSuccess },
                _                               => { let _ = error.print();         phase::Phase::TerminalError   },
            }
        }

        if let Some(path) = matches.unwrap().value_of(working_directory_argument_name).map(path::PathBuf::from) {
            global.logger.log_info(format!("Setting working directory to `{}`", path.display()));

            global.working_directory_path = path;
        }

        phase::continue_with(Box::new(verify_git_repository::VerifyGitRepository))
    }
}
