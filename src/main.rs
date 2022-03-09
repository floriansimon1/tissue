#![allow(unused_parens)]

use std::{thread, time, sync};

use antidote;

mod io;
mod git;
mod base;
mod steps;
mod users;
mod phases;
mod errors;
mod system;
mod issues;
mod logging;
mod commands;
mod structure;
mod configuration;

fn main() -> std::process::ExitCode {
    let mut current_state = phases::make_initial_phase();
    let     global        = sync::Arc::new(antidote::RwLock::new(phases::global::instantiate()));

    while current_state.can_continue() {
        global.read().logger.log_trace(format!("Reached phase `{}`", current_state.name()));

        current_state = current_state.next(global.clone());
    }

    global.read().logger.log_trace(format!("Reached terminal phase `{:?}`", current_state.as_exit_code()));

    while sync::Arc::strong_count(&global) > 1 {
        thread::sleep(time::Duration::from_millis(10));
    }

    sync
    ::Arc
    ::try_unwrap(global)
    .map(|global| global.into_inner().logger.await_termination())
    .unwrap_or_else(|_| panic!("Could not flush logs before exitting the app!"));

    current_state.as_exit_code()
}
