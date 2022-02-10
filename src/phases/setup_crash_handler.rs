pub struct SetupCrashHandler;

use human_panic::setup_panic;

use crate::base::phase;
use crate::phases::global::Global;
use crate::phases::configure_logging_backends;

impl phase::NonTerminalPhaseTrait<Global> for SetupCrashHandler {
    fn name(&self) -> &'static str {
        "SetupCrashHandler"
    }

    fn run(self: Box<Self>, _: &mut Global) -> phase::Phase<Global> {
        setup_panic!(Metadata {
            version:  env!("CARGO_PKG_VERSION").into(),
            name:     env!("CARGO_PKG_NAME").into(),
            authors:  "tissue".into(),
            homepage: "N/A".into(),
        });

        phase::continue_with(Box::new(configure_logging_backends::ConfigureLoggingBackends))
    }
}
