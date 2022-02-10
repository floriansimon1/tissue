use crate::logging;
use crate::base::phase;
use crate::phases::global;
use crate::phases::parse_command_line;

pub struct ConfigureLoggingBackends;

impl phase::NonTerminalPhaseTrait<global::Global> for ConfigureLoggingBackends {
    fn name(&self) -> &'static str {
        "ConfigureLoggingBackends"
    }

    fn run(self: Box<Self>, global: &mut global::Global) -> phase::Phase<global::Global> {
        global.logger.register_backend(Box::new(logging::backends::StdoutBackend));

        phase::continue_with(Box::new(parse_command_line::ParseCommandLine))
    }
}
