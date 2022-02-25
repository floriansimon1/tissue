use std::io;

pub fn safe_print(string: &str) {
    use std::io::Write;

    let mut stdout = io::stdout();

    match stdout.write(string.as_bytes()) {
        Err(error) if error.kind() != io::ErrorKind::BrokenPipe => panic!("An unknown error occurred while printing text!"),
        _                                                       => ()
    }
}

pub fn safe_println(string: &str) {
    safe_print(string);
    safe_print("\n");
}

pub fn safe_flush_stdout() {
    use io::Write;

    let _ = io::stdout().flush();
}
