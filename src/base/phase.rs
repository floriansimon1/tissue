pub enum Phase {
    #[allow(dead_code)] TerminalError,
    TerminalSuccess,
    NonTerminalPhase(Box<dyn NonTerminalPhaseTrait>)
}

impl Phase {
    pub fn can_continue(&self) -> bool {
        match self {
            Phase::NonTerminalPhase(_) => true,
            _                          => false
        }
    }

    pub fn next(self) -> Phase {
        match self {
            Phase::NonTerminalPhase(phase) => phase.run(),
            _                              => self
        }
    }

    pub fn as_exit_code(&self) -> std::process::ExitCode {
        match self {
            Phase::TerminalSuccess => std::process::ExitCode::SUCCESS,
            Phase::TerminalError   => std::process::ExitCode::FAILURE,
            _                      => panic!("Trying to get a exit code for a non-terminal program phase!")
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Phase::TerminalSuccess         => "TerminalSuccess",
            Phase::TerminalError           => "TerminalError",
            Phase::NonTerminalPhase(phase) => phase.name()
        }
    }
}

pub fn continue_with(phase: Box<dyn NonTerminalPhaseTrait>) -> Phase {
    Phase::NonTerminalPhase(phase)
}

pub trait NonTerminalPhaseTrait {
    fn run(self: Box<Self>) -> Phase;
    fn name(&self)          -> &'static str;
}
