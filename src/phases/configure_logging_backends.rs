pub struct ConfigureLoggingBackends;

use crate::logging;
use crate::base::phase;
use crate::phases::global::Global;

impl phase::NonTerminalPhaseTrait<Global> for ConfigureLoggingBackends {
    fn name(&self) -> &'static str {
        "ConfigureLoggingBackends"
    }

    fn run(self: Box<Self>, global: &mut Global) -> phase::Phase<Global> {
        global.logger.register_backend(Box::new(logging::backends::StdoutBackend));

        phase::Phase::TerminalSuccess
    }
}
