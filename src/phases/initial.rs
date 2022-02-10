use crate::base::phase;
use crate::phases::global;
use crate::phases::setup_crash_handler;

pub struct InitialPhase;

impl phase::NonTerminalPhaseTrait<global::Global> for InitialPhase {
    fn name(&self) -> &'static str {
        "InitialPhase"
    }

    fn run(self: Box<Self>, _: &mut global::Global) -> phase::Phase<global::Global> {
        phase::continue_with(Box::new(setup_crash_handler::SetupCrashHandler))
    }
}

pub fn make_initial_phase() -> phase::Phase<global::Global> {
    phase::continue_with(Box::new(InitialPhase))
}
