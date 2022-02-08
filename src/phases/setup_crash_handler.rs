pub struct SetupCrashHandler;

use human_panic::setup_panic;

use crate::base::phase;

impl phase::NonTerminalPhaseTrait for SetupCrashHandler {
    fn name(&self) -> &'static str {
        "SetupCrashHandler"
    }

    fn run(self: Box<Self>) -> phase::Phase {
        println!("Running SetupCrashHandler");

        setup_panic!(Metadata {
            version:  env!("CARGO_PKG_VERSION").into(),
            name:     env!("CARGO_PKG_NAME").into(),
            authors:  "tissue".into(),
            homepage: "N/A".into(),
        });

        phase::Phase::TerminalSuccess
    }
}
