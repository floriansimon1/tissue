use std::sync;

use antidote;
use human_panic::setup_panic;

use crate::base::phase;
use crate::phases::{global, parse_command_line};

pub struct SetupCrashHandler;

impl phase::NonTerminalPhaseTrait<global::Global> for SetupCrashHandler {
    fn name(&self) -> &'static str {
        "SetupCrashHandler"
    }

    fn run(self: Box<Self>, _: sync::Arc<antidote::RwLock<global::Global>>) -> phase::Phase<global::Global> {
        setup_panic!(Metadata {
            version:  env!("CARGO_PKG_VERSION").into(),
            name:     env!("CARGO_PKG_NAME").into(),
            authors:  "tissue".into(),
            homepage: "N/A".into(),
        });

        phase::continue_with(Box::new(parse_command_line::ParseCommandLine))
    }
}
