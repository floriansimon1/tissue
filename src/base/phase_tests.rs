struct Phase1;
struct Phase2;

use super::phase::{self, Phase, NonTerminalPhaseTrait};

impl NonTerminalPhaseTrait for Phase1 {
    fn name(&self) -> &'static str {
        "Phase 1"
    }

    fn run(self: Box<Self>) -> phase::Phase {
        phase::continue_with(Box::new(Phase2 {}))
    }
}

impl phase::NonTerminalPhaseTrait for Phase2 {
    fn name(&self) -> &'static str {
        "Phase 2"
    }

    fn run(self: Box<Self>) -> phase::Phase {
        phase::Phase::TerminalSuccess
    }
}

#[test]
fn check_iteration_on_phases() {
    let mut current_phase = phase::continue_with(Box::new(Phase1));

    match &current_phase {
        Phase::NonTerminalPhase(phase) => assert_eq!(phase.name(), (Phase1 {}).name()),
        _                              => panic!("We're not starting with Phase 1")
    }

    assert!(current_phase.can_continue(), "We cannot go anywhere from Phase 1");

    current_phase = current_phase.next();

    match &current_phase {
        Phase::NonTerminalPhase(phase) => assert_eq!(phase.name(), (Phase2 {}).name()),
        _                              => panic!("We did not reach Phase 2")
    }

    assert!(current_phase.can_continue(), "We could not run Phase 2");

    current_phase = current_phase.next();

    assert!(!current_phase.can_continue(), "Phase 2 was not the last one");

    if let Phase::TerminalSuccess = current_phase {} else {
        panic!("We stayed on phase 2")
    }

    current_phase = current_phase.next().next().next().next();

    if let Phase::TerminalSuccess = current_phase {} else {
        panic!("We did not remain on our terminal success")
    }
}

#[test]
fn check_exit_codes() {
    assert_eq!(format!("{:?}", (Phase::TerminalSuccess).as_exit_code()), "ExitCode(ExitCode(0))");
    assert_eq!(format!("{:?}", (Phase::TerminalError).as_exit_code()),   "ExitCode(ExitCode(1))");
}
