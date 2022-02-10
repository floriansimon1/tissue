pub struct InitialPhase;

use crate::base::phase;
use crate::phases::global::Global;
use crate::phases::setup_crash_handler;

impl phase::NonTerminalPhaseTrait<Global> for InitialPhase {
    fn name(&self) -> &'static str {
        "InitialPhase"
    }

    fn run(self: Box<Self>, _: &mut Global) -> phase::Phase<Global> {
        phase::continue_with(Box::new(setup_crash_handler::SetupCrashHandler))
    }
}

pub fn make_initial_phase() -> phase::Phase<Global> {
    phase::continue_with(Box::new(InitialPhase))
}
