use std::sync;

use antidote;

pub enum Phase<Globals> {
    #[allow(dead_code)] TerminalError,
    TerminalSuccess,
    NonTerminalPhase(Box<dyn NonTerminalPhaseTrait<Globals>>)
}

impl<Globals> Phase<Globals> {
    pub fn can_continue(&self) -> bool {
        match self {
            Phase::NonTerminalPhase(_) => true,
            _                          => false
        }
    }

    pub fn next(self, globals: sync::Arc<antidote::RwLock<Globals>>) -> Phase<Globals> {
        match self {
            Phase::NonTerminalPhase(phase) => phase.run(globals),
            _                              => self
        }
    }

    pub fn as_exit_code(&self) -> std::process::ExitCode {
        match self {
            Phase::TerminalSuccess::<Globals> => std::process::ExitCode::SUCCESS,
            Phase::TerminalError::<Globals>   => std::process::ExitCode::FAILURE,
            _                                 => panic!("Trying to get a exit code for a non-terminal program phase!")
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

pub fn continue_with<Globals>(phase: Box<dyn NonTerminalPhaseTrait<Globals>>) -> Phase<Globals> {
    Phase::NonTerminalPhase(phase)
}

pub trait NonTerminalPhaseTrait<Globals> {
    fn run(self: Box<Self>, globals: sync::Arc<antidote::RwLock<Globals>>) -> Phase<Globals>;
    fn name(&self)                                                         -> &'static str;
}
