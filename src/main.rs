#![feature(process_exitcode_placeholder)]

mod base;
mod phases;
mod logging;

fn main() -> std::process::ExitCode {
    let mut current_state = phases::make_initial_phase();
    let mut global        = phases::global::instantiate();

    while current_state.can_continue() {
        global.logger.log_trace(format!("Reached phase `{}`", current_state.name()));

        current_state = current_state.next(&mut global);
    }

    global.logger.log_trace(format!("Reached terminal phase `{:?}`", current_state.as_exit_code()));

    global.logger.await_termination();

    current_state.as_exit_code()
}
