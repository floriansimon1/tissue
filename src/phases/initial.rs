pub struct InitialPhase;

use crate::base::phase;
use crate::phases::setup_crash_handler;

impl phase::NonTerminalPhaseTrait for InitialPhase {
    fn name(&self) -> &'static str {
        "InitialPhase"
    }

    fn run(self: Box<Self>) -> phase::Phase {
        println!("Running InitialPhase");

        phase::continue_with(Box::new(setup_crash_handler::SetupCrashHandler))
    }
}

pub fn make_initial_phase() -> phase::Phase {
    phase::continue_with(Box::new(InitialPhase))
}
