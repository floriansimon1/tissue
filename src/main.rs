#![feature(process_exitcode_placeholder)]

mod base;
mod phases;

fn main() -> std::process::ExitCode {
    let mut current_state = phases::make_initial_phase();

    while current_state.can_continue() {
        current_state = current_state.next();
    }

    println!("Reached {}!", current_state.name());

    current_state.as_exit_code()
}
